import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { AppSettings, VpnStatus } from '../types/vpn';

const EVENT_STATUS_CHANGED = 'vpn-status-changed';

export function getVpnStatus(): Promise<VpnStatus> {
  return invoke<VpnStatus>('get_vpn_status');
}

export function connectVpn(): Promise<VpnStatus> {
  return invoke<VpnStatus>('connect_vpn');
}

export function disconnectVpn(): Promise<VpnStatus> {
  return invoke<VpnStatus>('disconnect_vpn');
}

export function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings');
}

export function saveSettings(settings: AppSettings): Promise<AppSettings> {
  return invoke<AppSettings>('save_settings', { settings });
}

export function saveAndroidAccount(email: string, accountType?: string | null): Promise<AppSettings> {
  return invoke<AppSettings>('save_android_account', {
    email,
    accountType: accountType ?? null,
  });
}

export function getTelemetryConsent(): Promise<boolean | null> {
  return invoke<boolean | null>('get_telemetry_consent');
}

export function setTelemetryConsent(consent: boolean): Promise<void> {
  return invoke<void>('set_telemetry_consent', { consent });
}

export async function onVpnStatusChanged(
  handler: (status: VpnStatus) => void,
): Promise<() => void> {
  return listen<VpnStatus>(EVENT_STATUS_CHANGED, (event) => {
    handler(event.payload);
  });
}
