use std::sync::{Arc, Mutex};

use crate::{
    settings::AppSettings,
    vpn::{backend::VpnBackend, mock::MockVpnBackend},
};

pub struct AppState {
    pub vpn: Arc<Mutex<Box<dyn VpnBackend>>>,
    pub settings: Arc<Mutex<AppSettings>>,
}

impl AppState {
    pub fn new(settings: AppSettings) -> Self {
        Self {
            vpn: Arc::new(Mutex::new(Box::new(MockVpnBackend::new()))),
            settings: Arc::new(Mutex::new(settings)),
        }
    }
}
