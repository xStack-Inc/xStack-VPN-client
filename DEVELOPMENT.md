# xStack VPN: техническая документация

Этот файл предназначен для разработчиков и сборщиков приложения. Пользовательская инструкция находится в [README.md](README.md).

## Технологии

- Tauri 2;
- Rust backend;
- Vue 3 Composition API;
- TypeScript;
- Vite;
- CSS без тяжелых UI-фреймворков;
- `@tauri-apps/plugin-autostart`;
- `tauri-plugin-log`;
- `reqwest` для отправки телеметрии.

## Требования для разработки

- Node.js 20 или новее;
- Rust stable;
- системные зависимости Tauri для вашей ОС;
- на Linux: WebKitGTK, librsvg, AppImage tooling и зависимости Tauri;
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

Артефакты появляются в `src-tauri/target/release/bundle/`.

## Сборка macOS DMG

Для Apple Silicon:

```bash
rustup target add aarch64-apple-darwin
CI=true npm run tauri:build -- --target aarch64-apple-darwin --bundles dmg
```

Для Intel:

```bash
rustup target add x86_64-apple-darwin
CI=true npm run tauri:build -- --target x86_64-apple-darwin --bundles dmg
```

`CI=true` отключает Finder/AppleScript-оформление DMG, которое может зависать в headless или sandbox-окружениях.

## Сборка Windows

Windows-сборку надежнее выполнять на Windows runner или Windows-машине:

```bash
npm run tauri:build -- --target x86_64-pc-windows-msvc
```

Для portable `.exe` можно использовать основной бинарный файл из:

```text
src-tauri/target/x86_64-pc-windows-msvc/release/
```

Для установщика используйте bundle-артефакты Tauri в:

```text
src-tauri/target/x86_64-pc-windows-msvc/release/bundle/
```

## Сборка Linux AppImage

```bash
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```

Нужны системные зависимости Tauri/WebKitGTK и инструменты для AppImage.

## Android APK signing

Android не устанавливает неподписанные APK. Release keystore не хранится в Git.

Локальный файл `xstack-vpn-release.keystore` добавлен в `.gitignore`. Для CI его нужно положить в GitHub Actions secrets в base64:

```bash
base64 -i xstack-vpn-release.keystore | pbcopy
```

Создайте secrets рядом с `TELEMETRY_AUTH`:

- `ANDROID_KEYSTORE_BASE64` — base64-содержимое `xstack-vpn-release.keystore`;
- `ANDROID_KEYSTORE_PASSWORD` — пароль keystore;
- `ANDROID_KEY_ALIAS` — alias ключа, например `xstack-vpn`;
- `ANDROID_KEY_PASSWORD` — пароль ключа. Если он совпадает с паролем keystore, можно задать то же значение.

Workflow подписывает `*-unsigned.apk` через `zipalign` и `apksigner`, затем публикует только подписанный APK artifact.

## GitHub Actions

Workflow `.github/workflows/build.yml` собирает артефакты для:

- Windows x86_64;
- macOS x86_64;
- macOS arm64;
- Linux x86_64.

Результаты публикуются как GitHub Actions artifacts.

## Структура проекта

```text
src/
  assets/            брендовые ресурсы и шрифты
  components/        UI-компоненты
  services/          Tauri IPC, форматирование, i18n, frontend state machine
  stores/            Vue store для состояния VPN и настроек
  types/             TypeScript-типы
  views/             основные экраны
src-tauri/
  capabilities/      Tauri permissions
  icons/             иконки приложения и трея
  Info.plist         дополнительные macOS Info.plist ключи
  src/
    commands.rs      IPC-команды frontend -> Rust
    lib.rs           инициализация Tauri
    settings.rs      локальное сохранение настроек
    state.rs         общее состояние приложения
    telemetry.rs     отправка телеметрии
    tray.rs          системный трей
    vpn/             абстракция VPN backend
```

## VPN backend

Текущая версия содержит имитационную реализацию backend для демонстрации UI и lifecycle приложения. Реальное VPN-подключение, WireGuard, OpenVPN, TUN/TAP, изменение маршрутов и DNS пока не реализованы.

Backend изолирован за trait `VpnBackend`, чтобы позже добавить реализации:

- `WireGuardBackend`;
- `OpenVpnBackend`;
- `CustomVpnBackend`.

Frontend не должен запускать shell-команды и не должен передавать произвольные пути к исполняемым файлам. Все действия VPN должны проходить через Rust backend.

## Телеметрия

Адрес отправки задается при сборке:

```bash
TELEMETRY_URL="http://example.local:5145" npm run tauri:build
```

Basic Auth задается через `TELEMETRY_AUTH`. Значение должно быть base64 от строки `login:password`:

```bash
printf 'user:P@ssw0rd' | base64

TELEMETRY_URL="http://178.57.68.241:5145" \
TELEMETRY_AUTH="dXNlcjpQQHNzdzByZA==" \
npm run tauri:build
```

В запрос уходит заголовок:

```text
Authorization: Basic <TELEMETRY_AUTH>
```

## macOS Info.plist

Дополнительные ключи macOS находятся в:

```text
src-tauri/Info.plist
```

Tauri объединяет этот файл с генерируемым `Info.plist`. Сейчас там включен:

```text
NSAppTransportSecurity.NSAllowsArbitraryLoads = true
```

Это разрешает сетевые соединения, которые иначе могут блокироваться ATS.

## Code signing для Windows

Для production нужен code signing certificate. Без подписи Windows SmartScreen может показывать предупреждение, потому что приложение не имеет репутации издателя.

Для подписи в CI потребуется:

- сертификат code signing;
- пароль/ключи в CI secrets;
- шаг подписи `.exe` и/или `.msi`.

## Signing и notarization для macOS

Проект использует ad-hoc подпись:

```json
"macOS": {
  "signingIdentity": "-"
}
```

Это помогает сделать bundle технически валидным, но не заменяет Developer ID и notarization. Для production потребуется:

- Apple Developer ID certificate;
- hardened runtime;
- корректные entitlements;
- notarization через Apple notary service;
- stapling ticket.

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

Production frontend build:

```bash
npm run build
```
