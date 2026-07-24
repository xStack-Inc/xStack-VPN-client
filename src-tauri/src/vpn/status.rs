use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VpnStatus {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error,
}

impl VpnStatus {
    pub fn tray_text(self) -> &'static str {
        match self {
            Self::Disconnected => "VPN выключен",
            Self::Connecting => "Подключение...",
            Self::Connected => "VPN включен",
            Self::Disconnecting => "Отключение...",
            Self::Error => "Ошибка",
        }
    }

    pub fn toggle_text(self) -> &'static str {
        match self {
            Self::Connected => "Выключить VPN",
            Self::Disconnecting => "Отключение...",
            _ => "Включить VPN",
        }
    }

    pub fn can_toggle(self) -> bool {
        !matches!(self, Self::Connecting | Self::Disconnecting)
    }
}

