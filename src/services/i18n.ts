import type { Language, VpnStatus } from '../types/vpn';

const dictionaries = {
  ru: {
    appName: 'Mock VPN',
    server: 'Автоматический сервер',
    location: 'Локация: Auto',
    ip: 'IP-адрес',
    duration: 'Длительность',
    traffic: 'Передано',
    settings: 'Настройки',
    close: 'Закрыть',
    launchOnStartup: 'Запускать вместе с ОС',
    minimizeToTray: 'Сворачивать в трей при закрытии',
    autoConnect: 'Автоматически подключаться после запуска',
    language: 'Язык интерфейса',
    connect: 'Включить',
    disconnect: 'Выключить',
    statuses: {
      disconnected: 'VPN выключен',
      connecting: 'Подключение...',
      connected: 'VPN включен',
      disconnecting: 'Отключение...',
      error: 'Ошибка',
    },
  },
  en: {
    appName: 'Mock VPN',
    server: 'Automatic server',
    location: 'Location: Auto',
    ip: 'IP address',
    duration: 'Duration',
    traffic: 'Transferred',
    settings: 'Settings',
    close: 'Close',
    launchOnStartup: 'Launch on startup',
    minimizeToTray: 'Minimize to tray on close',
    autoConnect: 'Connect automatically after launch',
    language: 'Interface language',
    connect: 'Connect',
    disconnect: 'Disconnect',
    statuses: {
      disconnected: 'VPN is off',
      connecting: 'Connecting...',
      connected: 'VPN is on',
      disconnecting: 'Disconnecting...',
      error: 'Error',
    },
  },
} as const;

export function t(language: Language) {
  return dictionaries[language];
}

export function statusText(status: VpnStatus, language: Language): string {
  return dictionaries[language].statuses[status];
}

