use chrono::Local;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use serde::Serialize;
use std::net::{IpAddr, UdpSocket};

const TELEMETRY_URL: &str = match option_env!("TELEMETRY_URL") {
    Some(v) => v,
    None => "",
};
const TELEMETRY_AUTH: &str = match option_env!("TELEMETRY_AUTH") {
    Some(v) => v,
    None => "",
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryPayload {
    pub device_id: String,
    pub app_version: String,
    pub os: String,
    pub os_version: String,
    pub arch: String,
    pub event: String,
    pub hostname: String,
    pub username: String,
    pub local_ips: Vec<String>,
    pub ad_info: AdInfo,
    /// ISO 8601 с локальным timezone offset, например "2026-08-07T10:23:35+03:00"
    pub timestamp: String,
}

/// Возвращает текущее время с локальным UTC-offset: "2026-08-07T10:23:35+03:00"
pub fn now_iso8601() -> String {
    Local::now().to_rfc3339()
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdInfo {
    /// USERDOMAIN или имя домена из переменных окружения
    pub domain: Option<String>,
    /// USERDNSDOMAIN — FQDN домена AD (только Windows)
    pub dns_domain: Option<String>,
    /// LOGONSERVER — имя DC, через который выполнен вход (только Windows)
    pub logon_server: Option<String>,
    /// Признак что пользователь вошёл через домен (domain != hostname)
    pub is_domain_user: bool,
}

impl TelemetryPayload {
    pub fn new(device_id: &str, event: &str) -> Self {
        let hostname = hostname();
        let ad_info = collect_ad_info(&hostname);
        Self {
            device_id: device_id.to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            os_version: os_version(),
            arch: std::env::consts::ARCH.to_string(),
            event: event.to_string(),
            hostname: hostname.clone(),
            username: username(),
            local_ips: local_ips(),
            ad_info,
            timestamp: now_iso8601(),
        }
    }
}

pub async fn send(payload: &TelemetryPayload) {
    if TELEMETRY_URL.is_empty() {
        return;
    }
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

    let mut request = client.post(TELEMETRY_URL).json(payload);

    if let Some(auth_header) = telemetry_authorization_header(TELEMETRY_AUTH) {
        request = request.header(AUTHORIZATION, auth_header);
    }

    match request.send().await {
        Ok(r) => log::debug!("telemetry: sent, status={}", r.status()),
        Err(e) => log::warn!("telemetry: send failed: {e}"),
    }
}

fn telemetry_authorization_header(base64_credentials: &str) -> Option<HeaderValue> {
    let credentials = base64_credentials.trim();
    if credentials.is_empty() {
        return None;
    }

    HeaderValue::from_str(&format!("Basic {credentials}")).ok()
}

// ---------------------------------------------------------------------------
// Сбор системной информации
// ---------------------------------------------------------------------------

fn hostname() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| cmd_output("hostname", &[]))
    }
    #[cfg(not(target_os = "windows"))]
    {
        cmd_output("hostname", &[])
    }
}

fn username() -> String {
    // USERNAME на Windows, USER на Unix
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown".to_string())
}

fn local_ips() -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();

    // Получаем исходящий IP через UDP-trick (соединение не устанавливается)
    if let Ok(sock) = UdpSocket::bind("0.0.0.0:0") {
        if sock.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = sock.local_addr() {
                ips.push(addr.ip().to_string());
            }
        }
    }

    // Все адреса через hostname -I (Linux/macOS) или ipconfig (Windows)
    #[cfg(target_os = "windows")]
    {
        let out = cmd_output("ipconfig", &[]);
        for line in out.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("IPv4 Address") {
                if let Some(ip) = rest.split(':').nth(1) {
                    let ip = ip.trim().to_string();
                    if !ips.contains(&ip) {
                        ips.push(ip);
                    }
                }
            }
            if let Some(rest) = line.strip_prefix("IPv6 Address") {
                if let Some(ip) = rest.split(':').nth(1) {
                    let ip = ip.trim().to_string();
                    if !ips.contains(&ip) {
                        ips.push(ip);
                    }
                }
            }
        }
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let out = cmd_output("hostname", &["-I"]);
        for part in out.split_whitespace() {
            if part.parse::<IpAddr>().is_ok() && !ips.contains(&part.to_string()) {
                ips.push(part.to_string());
            }
        }
    }

    ips
}

fn collect_ad_info(hostname: &str) -> AdInfo {
    #[cfg(target_os = "windows")]
    {
        let domain = std::env::var("USERDOMAIN").ok();
        let dns_domain = std::env::var("USERDNSDOMAIN").ok();
        let logon_server = std::env::var("LOGONSERVER")
            .ok()
            .map(|s| s.trim_start_matches('\\').to_string());

        // Пользователь доменный если USERDOMAIN не совпадает с именем компьютера
        let is_domain_user = domain
            .as_deref()
            .map(|d| !d.eq_ignore_ascii_case(hostname))
            .unwrap_or(false);

        AdInfo {
            domain,
            dns_domain,
            logon_server,
            is_domain_user,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        // На macOS/Linux проверяем через переменную окружения и dscl (macOS) / realm (Linux)
        let domain = std::env::var("USERDOMAIN")
            .ok()
            .or_else(|| macos_ad_domain());

        let is_domain_user = domain
            .as_deref()
            .map(|d| !d.eq_ignore_ascii_case(hostname))
            .unwrap_or(false);

        AdInfo {
            domain,
            dns_domain: None,
            logon_server: None,
            is_domain_user,
        }
    }
}

#[cfg(target_os = "macos")]
fn macos_ad_domain() -> Option<String> {
    // dsconfigad -show выводит Active Directory domain если машина привязана
    let out = cmd_output("dsconfigad", &["-show"]);
    for line in out.lines() {
        if line.contains("Active Directory Domain") {
            return line.split('=').nth(1).map(|s| s.trim().to_string());
        }
    }
    None
}

#[cfg(not(target_os = "macos"))]
fn macos_ad_domain() -> Option<String> {
    None
}

fn os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        // Читаем из реестра — самый надёжный способ на Windows
        let ver = cmd_output(
            "powershell",
            &["-NoProfile", "-Command",
              "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion' | Select-Object -ExpandProperty ProductName) + ' ' + (Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion' | Select-Object -ExpandProperty DisplayVersion)"
            ],
        );
        let ver = ver.trim().to_string();
        if ver.is_empty() {
            "unknown".to_string()
        } else {
            ver
        }
    }
    #[cfg(target_os = "macos")]
    {
        cmd_output("sw_vers", &["-productVersion"])
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|s| {
                s.lines().find(|l| l.starts_with("PRETTY_NAME=")).map(|l| {
                    l.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string()
                })
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

// Запускает команду и возвращает stdout. Никогда не паникует.
fn cmd_output(program: &str, args: &[&str]) -> String {
    let mut cmd = std::process::Command::new(program);
    cmd.args(args);

    // Скрываем консольное окно на Windows
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_basic_authorization_header_from_base64_credentials() {
        let header = telemetry_authorization_header("dXNlcjpQQHNzdzByZA==").unwrap();
        assert_eq!(header.to_str().unwrap(), "Basic dXNlcjpQQHNzdzByZA==");
    }

    #[test]
    fn trims_basic_authorization_credentials() {
        let header = telemetry_authorization_header("  dXNlcjpQQHNzdzByZA==\n").unwrap();
        assert_eq!(header.to_str().unwrap(), "Basic dXNlcjpQQHNzdzByZA==");
    }

    #[test]
    fn skips_authorization_header_when_credentials_are_empty() {
        assert!(telemetry_authorization_header("").is_none());
        assert!(telemetry_authorization_header("   ").is_none());
    }
}
