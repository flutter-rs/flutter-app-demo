# flutter-app

A desktop app built using flutter & rust.

![screenshot](https://raw.githubusercontent.com/gliheng/flutter-rs/master/www/images/screenshot_mac.png)


- Support Hot reload
- MethodChannel, EventChannel
- Async runtime using tokio
- System dialogs
- Clipboard support
- Cross platform support, Runs on mac, windows, linux
- Support distribution format: (windows NSIS, mac app, mac dmg, linux snap)

# Install requirements

- [Rust](https://www.rust-lang.org/tools/install)

- libglfw:
    - Install on Mac with: `brew install glfw`
    - Install on linux with `apt install libglfw3`
    
- [flutter sdk](https://flutter.io)

# Config flutter engine version
flutter-rs need to know your flutter engine version.
You can set this using any of the following methods.
- If you have flutter cli in your PATH, you're set.
- Set FLUTTER_ROOT environment variable to your flutter sdk path
- Set FLUTTER_ENGINE_VERSION environment variable to your engine version

# Develop
- To develop with cli hot-reloading, simple run:

    `python ./scripts/run.py`

- To debug using VS Code dart tools:

    Start process using `cargo run`

    Then attach to debugger using
    `flutter attach --debug-uri=DEBUG_URI`

# Distribute
- To build distribution, use:
    `python ./scripts/build.py mac|dmg|nsis|snap`

    **Note:**
    Build scripts are written in python3. Install python depenendencies using `pip3 install -r scripts/requirements.txt`
    Build on Windows require [NSIS3](https://sourceforge.net/projects/nsis/files/NSIS%203/)
