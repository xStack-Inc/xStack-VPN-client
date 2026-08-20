use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AndroidAccount;
#[cfg(mobile)]
use mobile::AndroidAccount;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-account APIs.
pub trait AndroidAccountExt<R: Runtime> {
    fn android_account(&self) -> &AndroidAccount<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidAccountExt<R> for T {
    fn android_account(&self) -> &AndroidAccount<R> {
        self.state::<AndroidAccount<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-account")
        .invoke_handler(tauri::generate_handler![commands::request_account])
        .setup(|app, api| {
            #[cfg(mobile)]
            let android_account = mobile::init(app, api)?;
            #[cfg(desktop)]
            let android_account = desktop::init(app, api)?;
            app.manage(android_account);
            Ok(())
        })
        .build()
}
