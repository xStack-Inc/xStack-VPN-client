#![cfg(desktop)]

use tauri::{
    image::Image,
    include_image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result,
};

use crate::{commands, vpn::status::VpnStatus};

const TRAY_ID: &str = "main-tray";

pub fn create_tray(app: &AppHandle, status: VpnStatus) -> Result<()> {
    let menu = build_menu(app, status)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(tray_icon(status))
        .icon_as_template(cfg!(target_os = "macos"))
        .tooltip(status.tray_text())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => show_main_window(app),
            "toggle" => commands::toggle_from_tray(app.clone()),
            "quit" => {
                log::info!("завершение приложения из системного трея");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(&tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

pub fn update_tray(app: &AppHandle, status: VpnStatus) -> Result<()> {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        tray.set_tooltip(Some(status.tray_text()))?;
        tray.set_icon_with_as_template(Some(tray_icon(status)), cfg!(target_os = "macos"))?;
        let menu = build_menu(app, status)?;
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if let Err(error) = window.show().and_then(|_| window.set_focus()) {
            log::error!("ошибка открытия главного окна: {error}");
        } else {
            log::info!("открытие главного окна");
        }
    }
}

fn build_menu(app: &AppHandle, status: VpnStatus) -> Result<Menu<tauri::Wry>> {
    let status_item = MenuItem::with_id(app, "status", status.tray_text(), false, None::<&str>)?;
    let open_item = MenuItem::with_id(app, "open", "Открыть", true, None::<&str>)?;
    let toggle_item = MenuItem::with_id(
        app,
        "toggle",
        status.toggle_text(),
        status.can_toggle(),
        None::<&str>,
    )?;
    let quit_item = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    Menu::with_items(
        app,
        &[
            &status_item,
            &separator,
            &open_item,
            &toggle_item,
            &separator,
            &quit_item,
        ],
    )
}

fn tray_icon(status: VpnStatus) -> Image<'static> {
    let icon = match status {
        VpnStatus::Connected => include_image!("./icons/tray-on.png"),
        VpnStatus::Connecting | VpnStatus::Disconnecting => include_image!("./icons/tray-busy.png"),
        VpnStatus::Disconnected | VpnStatus::Error => include_image!("./icons/tray-off.png"),
    };

    icon.to_owned()
}
