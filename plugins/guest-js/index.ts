import { invoke } from '@tauri-apps/api/core'

export interface AndroidAccountSelection {
  granted: boolean
  email?: string | null
  accountType?: string | null
  reason?: string | null
}

export async function requestAndroidAccount(): Promise<AndroidAccountSelection> {
  return await invoke<AndroidAccountSelection>('plugin:android-account|request_account')
}
