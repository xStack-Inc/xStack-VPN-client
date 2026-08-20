use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AndroidAccountExt;
use crate::Result;

#[command]
pub(crate) async fn request_account<R: Runtime>(app: AppHandle<R>) -> Result<AccountSelection> {
    app.android_account().request_account()
}
