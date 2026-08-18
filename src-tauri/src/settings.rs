use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ru,
    En,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub launch_on_startup: bool,
    pub minimize_to_tray: bool,
    pub auto_connect: bool,
    pub language: Language,
    /// None = не спрашивали ещё, Some(true/false) = ответил
    #[serde(default)]
    pub telemetry_consent: Option<bool>,
    /// Анонимный UUID, генерируется один раз и хранится локально
    #[serde(default = "new_device_id")]
    pub device_id: String,
}

fn new_device_id() -> String {
    Uuid::new_v4().to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            launch_on_startup: false,
            minimize_to_tray: true,
            auto_connect: false,
            language: Language::Ru,
            telemetry_consent: None,
            device_id: new_device_id(),
        }
    }
}

impl AppSettings {
    pub fn load() -> io::Result<Self> {
        let path = settings_path()?;
        if !path.exists() {
            let settings = Self::default();
            settings.save()?;
            return Ok(settings);
        }

        let raw = fs::read_to_string(path)?;
        serde_json::from_str(&raw).map_err(io::Error::other)
    }

    pub fn save(&self) -> io::Result<()> {
        let path = settings_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let raw = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, raw)
    }
}

fn settings_path() -> io::Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "example", "MockVpnClient")
        .ok_or_else(|| io::Error::other("cannot resolve config directory"))?;
    Ok(dirs.config_dir().join("settings.json"))
}
