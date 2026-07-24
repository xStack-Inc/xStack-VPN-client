import type { VpnStatus } from '../types/vpn';

export function canRequestConnect(status: VpnStatus): boolean {
  return status === 'disconnected' || status === 'error';
}

export function canRequestDisconnect(status: VpnStatus): boolean {
  return status === 'connected';
}

export function isTransitioning(status: VpnStatus): boolean {
  return status === 'connecting' || status === 'disconnecting';
}

export function requestedStatusAfterToggle(status: VpnStatus): VpnStatus {
  if (canRequestConnect(status)) {
    return 'connecting';
  }

  if (canRequestDisconnect(status)) {
    return 'disconnecting';
  }

  return status;
}

