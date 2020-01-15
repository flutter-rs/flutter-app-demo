# flutter-app-demo

A desktop app built using flutter & rust.

![flutter-app-demo][flutter-app-demo]

- Support Hot reload
- MethodChannel, EventChannel
- System dialogs
- Clipboard support
- Cross platform support, runs on mac, windows and linux

# Install requirements

- [Rust](https://www.rust-lang.org/tools/install)

- [flutter sdk](https://flutter.io)

# Develop

To develop, install [cargo-flutter](https://github.com/flutter-rs/cargo-flutter).

- Create a flutter-rs app

    `git clone https://github.com/flutter-rs/flutter-app-template`

- Run a flutter-rs app in dev mode

    `cargo flutter run`

- Bundle a flutter-rs app for distribution

    `cargo flutter --format appimage build --release`

# License
Copyright 2020 flutter-rs

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

[flutter-app-demo]: https://user-images.githubusercontent.com/741807/72479965-2c200580-37f6-11ea-8ddd-b91fa8759c94.png
