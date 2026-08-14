use crate::vpn::status::VpnStatus;

#[derive(Debug, thiserror::Error)]
pub enum VpnError {
    #[error("invalid VPN state transition from {from:?} to {to:?}")]
    InvalidTransition { from: VpnStatus, to: VpnStatus },
    #[error("mock backend failure: {0}")]
    BackendFailed(String),
}
