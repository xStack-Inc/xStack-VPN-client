use crate::vpn::{error::VpnError, status::VpnStatus};

pub trait VpnBackend: Send {
    fn connect(&mut self) -> Result<(), VpnError>;
    fn complete_connect(&mut self) -> Result<(), VpnError>;
    fn disconnect(&mut self) -> Result<(), VpnError>;
    fn complete_disconnect(&mut self) -> Result<(), VpnError>;
    fn status(&self) -> VpnStatus;
}
