import { computed, reactive, readonly } from 'vue';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import type { AppSettings, VpnStats, VpnStatus } from '../types/vpn';
import { isAndroidRuntime, requestAndroidAccount } from '../services/androidAccount';
import { t } from '../services/i18n';
import {
  connectVpn,
  disconnectVpn,
  getSettings,
  getTelemetryConsent,
  getVpnStatus,
  onVpnStatusChanged,
  saveAndroidAccount,
  saveSettings,
  setTelemetryConsent,
} from '../services/tauriVpn';
import { canRequestConnect, canRequestDisconnect, isTransitioning } from '../services/vpnStateMachine';

interface VpnStoreState {
  status: VpnStatus;
  settings: AppSettings;
  stats: VpnStats;
  isLoaded: boolean;
  errorMessage: string | null;
  androidAccountError: string | null;
  androidAccountRequesting: boolean;
  showConsentDialog: boolean;
}

const defaultSettings: AppSettings = {
  launchOnStartup: false,
  minimizeToTray: true,
  autoConnect: false,
  language: 'ru',
  telemetryConsent: null,
  deviceId: '',
  androidAccountEmail: null,
  androidAccountType: null,
};

const state = reactive<VpnStoreState>({
  status: 'disconnected',
  settings: defaultSettings,
  stats: {
    ipAddress: '100.96.12.34',
    durationSeconds: 0,
    transferredBytes: 0,
  },
  isLoaded: false,
  errorMessage: null,
  androidAccountError: null,
  androidAccountRequesting: false,
  showConsentDialog: false,
});

let timer: number | null = null;
let unsubscribeStatus: (() => void) | null = null;

const ANDROID_ACCOUNT_EMAIL_KEY = 'xstack.vpn.androidAccountEmail';
const ANDROID_ACCOUNT_TYPE_KEY = 'xstack.vpn.androidAccountType';

function readCachedAndroidAccount(): { email: string; accountType: string | null } | null {
  if (!isAndroidRuntime()) {
    return null;
  }

  const email = window.localStorage.getItem(ANDROID_ACCOUNT_EMAIL_KEY);
  if (!email) {
    return null;
  }

  return {
    email,
    accountType: window.localStorage.getItem(ANDROID_ACCOUNT_TYPE_KEY),
  };
}

function cacheAndroidAccount(email: string, accountType?: string | null) {
  if (!isAndroidRuntime()) {
    return;
  }

  window.localStorage.setItem(ANDROID_ACCOUNT_EMAIL_KEY, email);

  if (accountType) {
    window.localStorage.setItem(ANDROID_ACCOUNT_TYPE_KEY, accountType);
  } else {
    window.localStorage.removeItem(ANDROID_ACCOUNT_TYPE_KEY);
  }
}

function startStatsTimer() {
  stopStatsTimer();
  timer = window.setInterval(() => {
    state.stats.durationSeconds += 1;
    state.stats.transferredBytes += 18_432;
  }, 1000);
}

function stopStatsTimer() {
  if (timer !== null) {
    window.clearInterval(timer);
    timer = null;
  }
}

function applyStatus(status: VpnStatus) {
  state.status = status;

  if (status === 'connected') {
    startStatsTimer();
    return;
  }

  if (status === 'disconnected') {
    stopStatsTimer();
    state.stats.durationSeconds = 0;
    state.stats.transferredBytes = 0;
    state.errorMessage = null;
    return;
  }

  if (status === 'error') {
    stopStatsTimer();
    state.errorMessage = 'Backend error';
  }
}

export function useVpnStore() {
  const canToggle = computed(() => !isTransitioning(state.status));
  const isConnected = computed(() => state.status === 'connected');
  const androidAccountRequired = computed(() =>
    isAndroidRuntime() && !state.settings.androidAccountEmail,
  );

  async function initialize() {
    if (state.isLoaded) {
      return;
    }

    const [settings, status] = await Promise.all([getSettings(), getVpnStatus()]);
    state.settings = settings;

    const cachedAccount = readCachedAndroidAccount();
    if (!state.settings.androidAccountEmail && cachedAccount) {
      try {
        state.settings = await saveAndroidAccount(cachedAccount.email, cachedAccount.accountType);
      } catch {
        state.settings = {
          ...state.settings,
          androidAccountEmail: cachedAccount.email,
          androidAccountType: cachedAccount.accountType,
        };
      }
    }

    applyStatus(status);
    unsubscribeStatus = await onVpnStatusChanged(applyStatus);
    state.isLoaded = true;

    // Показываем диалог если пользователь ещё не отвечал
    const consent = await getTelemetryConsent();
    if (consent === null) {
      state.showConsentDialog = true;
    }
  }

  async function respondToConsent(consent: boolean) {
    state.showConsentDialog = false;
    await setTelemetryConsent(consent);
    state.settings = { ...state.settings, telemetryConsent: consent };
  }

  async function toggleVpn() {
    if (!canToggle.value || state.androidAccountRequesting) {
      return;
    }

    state.errorMessage = null;
    state.androidAccountError = null;

    if (canRequestConnect(state.status)) {
      const hasAndroidAccount = await ensureAndroidAccount();
      if (!hasAndroidAccount) {
        return;
      }

      applyStatus('connecting');
      const status = await connectVpn();
      applyStatus(status);
      return;
    }

    if (canRequestDisconnect(state.status)) {
      applyStatus('disconnecting');
      const status = await disconnectVpn();
      applyStatus(status);
    }
  }

  async function ensureAndroidAccount(): Promise<boolean> {
    if (!androidAccountRequired.value) {
      return true;
    }

    const cachedAccount = readCachedAndroidAccount();
    if (cachedAccount) {
      try {
        const saved = await saveAndroidAccount(cachedAccount.email, cachedAccount.accountType);
        state.settings = {
          ...saved,
          androidAccountEmail: saved.androidAccountEmail ?? cachedAccount.email,
          androidAccountType: saved.androidAccountType ?? cachedAccount.accountType,
        };
      } catch {
        state.settings = {
          ...state.settings,
          androidAccountEmail: cachedAccount.email,
          androidAccountType: cachedAccount.accountType,
        };
      }
      return true;
    }

    state.androidAccountRequesting = true;
    try {
      const selected = await requestAndroidAccount();
      if (!selected.granted || !selected.email) {
        state.androidAccountError = t(state.settings.language).androidAccountDenied;
        return false;
      }

      cacheAndroidAccount(selected.email, selected.accountType);
      try {
        const saved = await saveAndroidAccount(selected.email, selected.accountType);
        state.settings = {
          ...saved,
          androidAccountEmail: saved.androidAccountEmail ?? selected.email,
          androidAccountType: saved.androidAccountType ?? selected.accountType ?? null,
        };
      } catch {
        state.settings = {
          ...state.settings,
          androidAccountEmail: selected.email,
          androidAccountType: selected.accountType ?? null,
        };
      }
      return true;
    } catch (error) {
      state.androidAccountError = error instanceof Error
        ? error.message
        : t(state.settings.language).androidAccountDenied;
      return false;
    } finally {
      state.androidAccountRequesting = false;
    }
  }

  async function updateSettings(settings: AppSettings) {
    if (settings.launchOnStartup !== state.settings.launchOnStartup) {
      if (settings.launchOnStartup) {
        await enable();
      } else {
        await disable();
      }
    }

    state.settings = await saveSettings(settings);
  }

  function dispose() {
    stopStatsTimer();
    unsubscribeStatus?.();
    unsubscribeStatus = null;
  }

  return {
    state: readonly(state),
    canToggle,
    isConnected,
    androidAccountRequired,
    initialize,
    toggleVpn,
    updateSettings,
    respondToConsent,
    dispose,
  };
}
