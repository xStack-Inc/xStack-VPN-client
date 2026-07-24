# Mock VPN Client

Mock VPN Client - MVP кроссплатформенного desktop VPN-клиента на Tauri 2. В текущей версии приложение не создает реальное VPN-подключение, не меняет маршруты, DNS, TUN/TAP-интерфейсы и не запускает WireGuard/OpenVPN. Вместо этого реализован mock backend, который имитирует подключение и отключение, сохраняя единую модель состояния для UI, Rust backend и системного трея.

## Технологии

- Tauri 2;
- Rust backend;
- Vue 3 Composition API;
- TypeScript;
- Vite;
- CSS без тяжелых UI-фреймворков;
- официальный плагин `@tauri-apps/plugin-autostart`;
- `tauri-plugin-log` для базового логирования.

## Требования для разработки

- Node.js 20 или новее;
- Rust stable;
- системные зависимости Tauri для вашей ОС;
- на Linux: WebKitGTK, librsvg, AppImage tooling и зависимости, описанные в документации Tauri;
- на Windows: Microsoft C++ Build Tools;
- на macOS: Xcode Command Line Tools.

## Установка зависимостей

```bash
npm install
```

## Запуск в development-режиме

```bash
npm run tauri:dev
```

Frontend dev server запускается на `http://127.0.0.1:1420`.

## Локальная сборка

```bash
npm run tauri:build
```

Артефакты Tauri появляются в `src-tauri/target/release/bundle/`.

## Сборка под ОС

Windows x86_64 собирается на Windows runner:

```bash
npm run tauri:build
```

macOS x86_64:

```bash
rustup target add x86_64-apple-darwin
npm run tauri:build -- --target x86_64-apple-darwin
```

macOS arm64:

```bash
rustup target add aarch64-apple-darwin
npm run tauri:build -- --target aarch64-apple-darwin
```

Linux x86_64:

```bash
npm run tauri:build
```

Не предполагается один универсальный бинарный файл для Windows, macOS и Linux. Для каждой платформы создается отдельный артефакт.

## Структура проекта

```text
src/
  components/        UI-компоненты
  services/          Tauri IPC, форматирование, i18n, frontend state machine
  stores/            Vue store для состояния VPN и настроек
  types/             TypeScript-типы
  views/             Основные экраны
src-tauri/
  capabilities/      Минимальные Tauri permissions
  icons/             Иконки приложения
  src/
    commands.rs      IPC-команды frontend -> Rust
    main.rs          Инициализация Tauri, логирование, окно
    settings.rs      Локальное сохранение настроек
    state.rs         Общее состояние приложения
    tray.rs          Системный трей
    vpn/             Абстракция и mock VPN backend
```

## Mock VPN backend

`MockVpnBackend` реализует trait `VpnBackend`:

- `connect()` переводит состояние в `Connecting`;
- `complete_connect()` завершает mock-подключение состоянием `Connected`;
- `disconnect()` переводит состояние в `Disconnecting`;
- `complete_disconnect()` завершает mock-отключение состоянием `Disconnected`;
- `status()` возвращает единое состояние backend.

Задержка подключения и отключения реализована на уровне сервиса команд Tauri, чтобы UI и трей получали промежуточные состояния сразу.

## Добавление настоящего VPN backend

Новая реализация должна реализовать `VpnBackend` в `src-tauri/src/vpn/`. Например:

- `WireGuardBackend`;
- `OpenVpnBackend`;
- `CustomVpnBackend`.

Frontend не должен запускать shell-команды и не должен передавать произвольные пути к исполняемым файлам. Все операции запуска, остановки, проверки статуса и обработки ошибок должны проходить через Rust backend и выделенный сервис управления VPN. Секреты, приватные ключи и пароли нельзя хранить в исходном коде или логах.

## Системный трей

Трей создается в `src-tauri/src/tray.rs`. В меню есть:

- текущий статус VPN;
- `Открыть`;
- `Включить VPN` или `Выключить VPN`;
- `Выход`.

При закрытии главного окна приложение по умолчанию скрывается и продолжает работать в фоне. Полное завершение происходит через `Выход` в трее. Клик по иконке трея открывает главное окно там, где это поддерживается платформой.

## Настройки

Настройки сохраняются локально в пользовательской config-директории:

- запуск вместе с ОС;
- сворачивание в трей при закрытии;
- автоматическое подключение после запуска;
- язык интерфейса: русский или английский.

Автозапуск включается и отключается через официальный Tauri autostart plugin.

## Логирование

Логируются запуск приложения, открытие/закрытие окна, сворачивание в трей, запрос подключения, успешное mock-подключение, запрос отключения, успешное mock-отключение, ошибки backend и завершение приложения.

## Тестирование

Rust:

```bash
cd src-tauri
cargo test
```

Frontend:

```bash
npm test
```

## GitHub Actions

Workflow `.github/workflows/build.yml` собирает отдельные артефакты:

- `vpn-client-windows-x86_64`;
- `vpn-client-macos-x86_64`;
- `vpn-client-macos-arm64`;
- `vpn-client-linux-x86_64`.

Имена финальных файлов в artifacts должны однозначно указывать ОС и архитектуру. Точные расширения зависят от bundle target, доступных инструментов runner и настроек Tauri.

## Code signing для Windows

В MVP signing не настроен. Windows SmartScreen может показывать предупреждение, потому что приложение не подписано сертификатом издателя и не имеет репутации. Для production потребуется code signing certificate, настройка подписи `.exe`/`.msi` и безопасное хранение сертификата в CI secrets.

## Signing и notarization для macOS

В MVP signing и notarization не настроены. macOS Gatekeeper может предупреждать о неизвестном разработчике. Для production потребуется Apple Developer ID certificate, подпись `.app`/`.dmg`, hardened runtime, корректные entitlements и notarization через Apple notary service.

## AppImage для Linux

Linux-сборка настраивается через Tauri bundle target `appimage`. Для успешной сборки runner или локальная машина должны иметь системные зависимости Tauri/WebKitGTK и инструменты, необходимые для AppImage.

## Известные ограничения

- нет настоящего VPN-подключения;
- нет WireGuard/OpenVPN/TUN/TAP;
- нет изменения маршрутов и DNS;
- нет kill switch и split tunneling;
- нет загрузки VPN-конфигураций;
- нет авторизации, подписок, updater и удаленного API;
- IP, длительность и трафик являются mock-данными.

