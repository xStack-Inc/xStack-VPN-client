use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryPayload {
    pub device_id: String,
    pub app_version: String,
    pub os: String,
    pub os_version: String,
    pub arch: String,
    pub event: String,
}

impl TelemetryPayload {
    pub fn new(device_id: &str, event: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            os_version: os_version(),
            arch: std::env::consts::ARCH.to_string(),
            event: event.to_string(),
        }
    }
}

pub async fn send(payload: &TelemetryPayload, url: &str, auth: &str) {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            log::warn!("telemetry: failed to build client: {e}");
            return;
        }
    };

    match client
        .post(url)
        .header("X-Auth-User", auth)
        .json(payload)
        .send()
        .await
    {
        Ok(r) => log::debug!("telemetry: sent, status={}", r.status()),
        Err(e) => log::warn!("telemetry: send failed: {e}"),
    }
}

fn os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "ver"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("PRETTY_NAME="))
                    .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
            })
            .unwrap_or_else(|| "linux".to_string())
    }
    #[cfg(target_os = "android")]
    {
        "android".to_string()
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        "unknown".to_string()
    }
}
