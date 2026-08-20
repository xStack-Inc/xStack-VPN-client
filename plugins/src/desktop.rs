use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<AndroidAccount<R>> {
    Ok(AndroidAccount(app.clone()))
}

/// Access to the android-account APIs.
pub struct AndroidAccount<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AndroidAccount<R> {
    pub fn request_account(&self) -> crate::Result<AccountSelection> {
        Ok(AccountSelection {
            granted: false,
            reason: Some("unsupported_platform".to_string()),
            ..AccountSelection::default()
        })
    }
}
