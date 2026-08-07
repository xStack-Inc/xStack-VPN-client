#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod settings;
mod state;
mod telemetry;
mod vpn;

#[cfg(desktop)]
mod tray;

use crate::{settings::AppSettings, state::AppState};

#[allow(unused_imports)]
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let settings = AppSettings::load().unwrap_or_else(|error| {
        log::error!("ошибка загрузки настроек: {error}");
        AppSettings::default()
    });

    #[cfg(desktop)]
    let auto_connect = settings.auto_connect;

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build());

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        None,
    ));

    builder
        .manage(AppState::new(settings))
        .invoke_handler(tauri::generate_handler![
            commands::get_vpn_status,
            commands::connect_vpn,
            commands::disconnect_vpn,
            commands::get_settings,
            commands::save_settings,
            commands::get_telemetry_consent,
            commands::set_telemetry_consent,
        ])
        .setup(move |app| {
            log::info!("запуск приложения");

            #[cfg(desktop)]
            {
                tray::create_tray(app.handle(), crate::vpn::status::VpnStatus::Disconnected)?;

                if auto_connect {
                    let state = app.state::<AppState>();
                    if let Err(error) =
                        commands::request_connect(app.handle().clone(), state.vpn.clone())
                    {
                        log::error!("ошибка автоподключения: {error}");
                    }
                }
            }

            let _ = app;
            Ok(())
        })
        .on_window_event(|_window, _event| {
            #[cfg(desktop)]
            if let tauri::WindowEvent::CloseRequested { api, .. } = _event {
                let app = _window.app_handle();
                let _minimize_to_tray = app
                    .state::<AppState>()
                    .settings
                    .lock()
                    .map(|settings| settings.minimize_to_tray)
                    .unwrap_or(true);

                log::info!("закрытие главного окна");

                api.prevent_close();
                if let Err(error) = _window.hide() {
                    log::error!("ошибка сворачивания в трей: {error}");
                } else {
                    log::info!("сворачивание в трей");
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
