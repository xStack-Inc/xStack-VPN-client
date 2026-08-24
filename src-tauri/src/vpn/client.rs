use crate::vpn::{backend::VpnBackend, error::VpnError, status::VpnStatus};

pub struct XStackVpnBackend {
    status: VpnStatus,
    fail_next_connect: bool,
    fail_next_disconnect: bool,
}

impl Default for XStackVpnBackend {
    fn default() -> Self {
        Self {
            status: VpnStatus::Disconnected,
            fail_next_connect: false,
            fail_next_disconnect: false,
        }
    }
}

impl XStackVpnBackend {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn fail_next_connect(&mut self) {
        self.fail_next_connect = true;
    }

    #[cfg(test)]
    pub fn fail_next_disconnect(&mut self) {
        self.fail_next_disconnect = true;
    }

    fn transition(&mut self, to: VpnStatus) -> Result<(), VpnError> {
        let allowed = matches!(
            (self.status, to),
            (VpnStatus::Disconnected, VpnStatus::Connecting)
                | (VpnStatus::Error, VpnStatus::Connecting)
                | (VpnStatus::Connecting, VpnStatus::Connected)
                | (VpnStatus::Connecting, VpnStatus::Error)
                | (VpnStatus::Connected, VpnStatus::Disconnecting)
                | (VpnStatus::Disconnecting, VpnStatus::Disconnected)
                | (VpnStatus::Disconnecting, VpnStatus::Error)
        );

        if !allowed {
            return Err(VpnError::InvalidTransition {
                from: self.status,
                to,
            });
        }

        self.status = to;
        Ok(())
    }
}

impl VpnBackend for XStackVpnBackend {
    fn connect(&mut self) -> Result<(), VpnError> {
        self.transition(VpnStatus::Connecting)
    }

    fn complete_connect(&mut self) -> Result<(), VpnError> {
        if self.fail_next_connect {
            self.fail_next_connect = false;
            self.transition(VpnStatus::Error)?;
            return Err(VpnError::BackendFailed(
                "connect failed by test flag".into(),
            ));
        }

        self.transition(VpnStatus::Connected)
    }

    fn disconnect(&mut self) -> Result<(), VpnError> {
        self.transition(VpnStatus::Disconnecting)
    }

    fn complete_disconnect(&mut self) -> Result<(), VpnError> {
        if self.fail_next_disconnect {
            self.fail_next_disconnect = false;
            self.transition(VpnStatus::Error)?;
            return Err(VpnError::BackendFailed(
                "disconnect failed by test flag".into(),
            ));
        }

        self.transition(VpnStatus::Disconnected)
    }

    fn status(&self) -> VpnStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_disconnected() {
        let backend = XStackVpnBackend::new();
        assert_eq!(backend.status(), VpnStatus::Disconnected);
    }

    #[test]
    fn connects_from_disconnected() {
        let mut backend = XStackVpnBackend::new();
        backend.connect().unwrap();
        assert_eq!(backend.status(), VpnStatus::Connecting);
        backend.complete_connect().unwrap();
        assert_eq!(backend.status(), VpnStatus::Connected);
    }

    #[test]
    fn disconnects_from_connected() {
        let mut backend = XStackVpnBackend::new();
        backend.connect().unwrap();
        backend.complete_connect().unwrap();
        backend.disconnect().unwrap();
        assert_eq!(backend.status(), VpnStatus::Disconnecting);
        backend.complete_disconnect().unwrap();
        assert_eq!(backend.status(), VpnStatus::Disconnected);
    }

    #[test]
    fn rejects_duplicate_connect_while_connecting() {
        let mut backend = XStackVpnBackend::new();
        backend.connect().unwrap();
        let error = backend.connect().unwrap_err();
        assert!(matches!(error, VpnError::InvalidTransition { .. }));
        assert_eq!(backend.status(), VpnStatus::Connecting);
    }

    #[test]
    fn rejects_disconnect_while_disconnected() {
        let mut backend = XStackVpnBackend::new();
        let error = backend.disconnect().unwrap_err();
        assert!(matches!(error, VpnError::InvalidTransition { .. }));
        assert_eq!(backend.status(), VpnStatus::Disconnected);
    }

    #[test]
    fn handles_connect_error() {
        let mut backend = XStackVpnBackend::new();
        backend.fail_next_connect();
        backend.connect().unwrap();
        let error = backend.complete_connect().unwrap_err();
        assert!(matches!(error, VpnError::BackendFailed(_)));
        assert_eq!(backend.status(), VpnStatus::Error);
    }

    #[test]
    fn handles_disconnect_error() {
        let mut backend = XStackVpnBackend::new();
        backend.connect().unwrap();
        backend.complete_connect().unwrap();
        backend.fail_next_disconnect();
        backend.disconnect().unwrap();
        let error = backend.complete_disconnect().unwrap_err();
        assert!(matches!(error, VpnError::BackendFailed(_)));
        assert_eq!(backend.status(), VpnStatus::Error);
    }
}
