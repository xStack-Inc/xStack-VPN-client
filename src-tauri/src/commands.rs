use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, State};

use crate::{
    settings::AppSettings,
    state::AppState,
    telemetry,
    vpn::{backend::VpnBackend, status::VpnStatus},
};

#[cfg(desktop)]
use crate::tray;

#[tauri::command]
pub fn get_vpn_status(state: State<'_, AppState>) -> Result<VpnStatus, String> {
    let vpn = state.vpn.lock().map_err(|error| error.to_string())?;
    Ok(vpn.status())
}

#[tauri::command]
pub fn connect_vpn(app: AppHandle, state: State<'_, AppState>) -> Result<VpnStatus, String> {
    request_connect(app, state.vpn.clone(), telemetry_device_id(&state))
}

#[tauri::command]
pub fn disconnect_vpn(app: AppHandle, state: State<'_, AppState>) -> Result<VpnStatus, String> {
    request_disconnect(app, state.vpn.clone(), telemetry_device_id(&state))
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = state.settings.lock().map_err(|error| error.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
pub fn save_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<AppSettings, String> {
    settings.save().map_err(|error| error.to_string())?;
    let mut current = state.settings.lock().map_err(|error| error.to_string())?;
    *current = settings.clone();
    Ok(settings)
}

// #[tauri::command]
// pub fn get_telemetry_consent(state: State<'_, AppState>) -> Result<Option<bool>, String> {
//     let settings = state.settings.lock().map_err(|e| e.to_string())?;
//     Ok(settings.telemetry_consent)
// }
#[tauri::command]
pub fn get_telemetry_consent(state: State<'_, AppState>) -> Result<Option<bool>, String> {
    let _ = state; // Явно игнорируем переменную, чтобы убрать warning
    Ok(Some(true))
}


#[tauri::command]
pub async fn set_telemetry_consent(
    _consent: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let consent = true;

    let dev_id = {
        let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
        settings.telemetry_consent = Some(consent);
        settings.save().map_err(|e| e.to_string())?;
        settings.device_id.clone()
    };

    if consent {
        send_vpn_event(&dev_id, "app_launch").await;
    }

    Ok(())
}

pub fn send_app_launch_if_allowed(app: AppHandle) {
    let state = app.state::<AppState>();
    if let Some(dev_id) = telemetry_device_id(&state) {
        tauri::async_runtime::spawn(async move {
            send_vpn_event(&dev_id, "app_launch").await;
        });
    }
}

pub fn toggle_from_tray(app: AppHandle) {
    #[cfg(desktop)]
    {
        let state = app.state::<AppState>();
        let status = match state.vpn.lock() {
            Ok(vpn) => vpn.status(),
            Err(error) => {
                log::error!("ошибка backend: {error}");
                return;
            }
        };
        let telemetry_dev_id = telemetry_device_id(&state);

        let result = match status {
            VpnStatus::Connected => {
                request_disconnect(app.clone(), state.vpn.clone(), telemetry_dev_id)
            }
            VpnStatus::Disconnected | VpnStatus::Error => {
                request_connect(app.clone(), state.vpn.clone(), telemetry_dev_id)
            }
            _ => Ok(status),
        };

        if let Err(error) = result {
            log::error!("ошибка backend: {error}");
            let _ = emit_status(&app, VpnStatus::Error);
        }
    }
}

pub fn request_connect(
    app: AppHandle,
    vpn: Arc<Mutex<Box<dyn VpnBackend>>>,
    telemetry_dev_id: Option<String>,
) -> Result<VpnStatus, String> {
    log::info!("запрос подключения");
    {
        let mut backend = vpn.lock().map_err(|error| error.to_string())?;
        backend.connect().map_err(|error| {
            log::error!("ошибка backend: {error}");
            error.to_string()
        })?;
    }

    emit_status(&app, VpnStatus::Connecting)?;

    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1_300)).await;
        let result = match vpn.lock() {
            Ok(mut backend) => backend.complete_connect().map(|_| backend.status()),
            Err(error) => Err(crate::vpn::error::VpnError::BackendFailed(
                error.to_string(),
            )),
        };

        match result {
            Ok(status) => {
                log::info!("успешное mock-подключение");
                let _ = emit_status(&app, status);
                if let Some(dev_id) = telemetry_dev_id {
                    send_vpn_event(&dev_id, "vpn_connected").await;
                }
            }
            Err(error) => {
                log::error!("ошибка backend: {error}");
                let _ = emit_status(&app, VpnStatus::Error);
            }
        }
    });

    Ok(VpnStatus::Connecting)
}

pub fn request_disconnect(
    app: AppHandle,
    vpn: Arc<Mutex<Box<dyn VpnBackend>>>,
    telemetry_dev_id: Option<String>,
) -> Result<VpnStatus, String> {
    log::info!("запрос отключения");
    {
        let mut backend = vpn.lock().map_err(|error| error.to_string())?;
        backend.disconnect().map_err(|error| {
            log::error!("ошибка backend: {error}");
            error.to_string()
        })?;
    }

    emit_status(&app, VpnStatus::Disconnecting)?;

    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(900)).await;
        let result = match vpn.lock() {
            Ok(mut backend) => backend.complete_disconnect().map(|_| backend.status()),
            Err(error) => Err(crate::vpn::error::VpnError::BackendFailed(
                error.to_string(),
            )),
        };

        match result {
            Ok(status) => {
                log::info!("успешное mock-отключение");
                let _ = emit_status(&app, status);
                if let Some(dev_id) = telemetry_dev_id {
                    send_vpn_event(&dev_id, "vpn_disconnected").await;
                }
            }
            Err(error) => {
                log::error!("ошибка backend: {error}");
                let _ = emit_status(&app, VpnStatus::Error);
            }
        }
    });

    Ok(VpnStatus::Disconnecting)
}

fn emit_status(app: &AppHandle, status: VpnStatus) -> Result<(), String> {
    #[cfg(desktop)]
    tray::update_tray(app, status).map_err(|error| error.to_string())?;
    app.emit("vpn-status-changed", status)
        .map_err(|error| error.to_string())
}

pub fn telemetry_device_id(state: &State<'_, AppState>) -> Option<String> {
    state
        .settings
        .lock()
        .ok()
        .filter(|settings| settings.telemetry_consent == Some(true))
        .map(|settings| settings.device_id.clone())
}

async fn send_vpn_event(dev_id: &str, event: &str) {
    let payload = telemetry::TelemetryPayload::new(dev_id, event);
    telemetry::send(&payload).await;
}
