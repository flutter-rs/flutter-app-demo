# flutter-app

A desktop app built using flutter & rust.

![screenshot](https://raw.githubusercontent.com/flutter-rs/flutter-rs/master/www/images/screenshot_mac.png)


- Support Hot reload
- MethodChannel, EventChannel
- Async runtime using tokio
- System dialogs
- Clipboard support
- Cross platform support, Runs on mac, windows, linux
- Support distribution format: (windows NSIS, mac app, mac dmg, linux snap)

# Install requirements

- [Rust](https://www.rust-lang.org/tools/install)

- [flutter sdk](https://flutter.io)

# Config flutter engine version
flutter-rs need to know your flutter engine version.
You can set this using any of the following methods.
- If you have flutter cli in your PATH, you're set.
- Set FLUTTER_ROOT environment variable to your flutter sdk path
- Set FLUTTER_ENGINE_VERSION environment variable to your engine version

# Develop

To develop, install [cargo-flutter](https://github.com/flutter-rs/cargo-flutter).

- Create a flutter-rs app

    `git clone https://github.com/flutter-rs/flutter-app-template`

- Run a flutter-rs app in dev mode

    `cargo flutter run`

- Bundle a flutter-rs app for distribution

    `cargo flutter --format appimage build --release`
