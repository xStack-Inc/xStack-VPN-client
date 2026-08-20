import { invoke } from '@tauri-apps/api/core';

export interface AndroidAccountSelection {
  granted: boolean;
  email?: string | null;
  accountType?: string | null;
  reason?: string | null;
}

export function isAndroidRuntime(): boolean {
  return /\bAndroid\b/i.test(window.navigator.userAgent);
}

export function requestAndroidAccount(): Promise<AndroidAccountSelection> {
  return invoke<AndroidAccountSelection>('plugin:android-account|request_account');
}
