use flutter_plugins::settings;
use flutter_winit::FlutterWindow;
use glutin::window::WindowBuilder;
use std::path::Path;

mod calc_channel;
mod msg_stream_channel;

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();

    let assets_dir = std::env::var("FLUTTER_ASSET_DIR").expect("FLUTTER_ASSET_DIR");

    let mut args = Vec::with_capacity(1);

    if let Ok(snapshot) = std::env::var("FLUTTER_AOT_SNAPSHOT") {
        if Path::new(&snapshot).exists() {
            args.push(format!("--aot-shared-library-name={}", snapshot));
        }
    }

    let window = WindowBuilder::new().with_title("Flutter App Demo");
    let flutter = FlutterWindow::new(window).unwrap();
    let flutter = flutter.with_resource_context().unwrap();

    flutter
        .add_plugin(calc_channel::CalcPlugin::default())
        .add_plugin(msg_stream_channel::MsgStreamPlugin::default());

    flutter.start_engine(Path::new(&assets_dir), &args).unwrap();

    flutter.with_plugin(|p: &settings::SettingsPlugin| {
        p.start_message()
            .set_text_scale_factor(1.0)
            .set_platform_brightness(settings::PlatformBrightness::Dark)
            .send();
    });

    flutter.run();
}

#[cfg(target_os = "android")]
fn main() {
    use android_logger::Config;
    use android_ndk::android_app::AndroidApp;
    use jni_android_sys::android::content::Context;
    use log::Level;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::PathBuf;

    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Debug)
            .with_tag("flutter_app_demo"),
    );

    let android_app = unsafe { AndroidApp::from_ptr(android_glue::get_android_app()) };

    let assets_dir = PathBuf::from(OsStr::from_bytes(
        android_app.activity().internal_data_path().to_bytes(),
    ));

    let mut args = Vec::with_capacity(1);

    let vm = unsafe { jni_glue::VM::from_jni_local(&*android_app.activity().vm()) };

    let snapshot = vm.with_env(|env| {
        let context = Context::new(env).unwrap();
        let info = context.getApplicationInfo().unwrap().unwrap();
        let lib_dir = info.nativeLibraryDir().unwrap().to_string().unwrap();
        Path::new(&lib_dir).join("app.so")
    });

    if snapshot.exists() {
        args.push(format!("--aot-shared-library-name={}", snapshot.display()));
    } else {
        std::fs::create_dir_all(&assets_dir).unwrap();
        copy_asset(
            &android_app.activity().asset_manager(),
            b"kernel_blob.bin\0",
            &assets_dir.join("kernel_blob.bin"),
        );
    }

    let window = WindowBuilder::new().with_title("Flutter App Demo");
    let flutter = FlutterWindow::new(window).unwrap();

    flutter
        .add_plugin(calc_channel::CalcPlugin::default())
        .add_plugin(msg_stream_channel::MsgStreamPlugin::default());

    flutter.start_engine(Path::new(&assets_dir), &args).unwrap();

    flutter.with_plugin(|p: &settings::SettingsPlugin| {
        p.start_message()
            .set_text_scale_factor(1.0)
            .set_platform_brightness(settings::PlatformBrightness::Dark)
            .send();
    });

    flutter.run();
}

#[cfg(target_os = "android")]
fn copy_asset(asset_manager: &android_ndk::asset::AssetManager, asset: &[u8], path: &Path) {
    use std::ffi::CStr;
    use std::io::Read;

    let bytes: Vec<u8> = asset_manager
        .open(CStr::from_bytes_with_nul(asset).unwrap())
        .unwrap()
        .bytes()
        .collect::<Result<_, _>>()
        .unwrap();
    std::fs::write(path, bytes).unwrap();
}
