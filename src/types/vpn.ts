export type VpnStatus =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'disconnecting'
  | 'error';

export type Language = 'ru' | 'en';

export interface AppSettings {
  launchOnStartup: boolean;
  minimizeToTray: boolean;
  autoConnect: boolean;
  language: Language;
  telemetryConsent: boolean | null;
  deviceId: string;
  androidAccountEmail: string | null;
  androidAccountType: string | null;
}

export interface VpnStats {
  ipAddress: string;
  durationSeconds: number;
  transferredBytes: number;
}
