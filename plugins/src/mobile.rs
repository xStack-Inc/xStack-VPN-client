use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_android_account);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AndroidAccount<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("com.xstack.vpn.androidaccount", "AndroidAccountPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_android_account)?;
    Ok(AndroidAccount(handle))
}

/// Access to the android-account APIs.
pub struct AndroidAccount<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidAccount<R> {
    pub fn request_account(&self) -> crate::Result<AccountSelection> {
        self.0
            .run_mobile_plugin("requestAccount", RequestAccountRequest::default())
            .map_err(Into::into)
    }
}
