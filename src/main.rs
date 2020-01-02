use flutter_engine::loader::load_snapshot;
use flutter_plugins::settings;
use flutter_winit::FlutterWindow;
use glutin::window::WindowBuilder;
use std::path::PathBuf;

mod calc_channel;
mod msg_stream_channel;

fn main() {
    env_logger::init();

    let asset_dir = std::env::var("FLUTTER_ASSET_DIR").expect("FLUTTER_ASSET_DIR");

    let (_lib, snapshot) = std::env::var("FLUTTER_AOT_SNAPSHOT")
        .ok()
        .map(|aot_path| {
            unsafe { load_snapshot(&PathBuf::from(aot_path)) }
                .ok()
                .map(|(lib, snapshot)| (Some(lib), Some(snapshot)))
                .unwrap_or_default()
        })
        .unwrap_or_default();

    let window = WindowBuilder::new().with_title("Flutter App Demo");
    let flutter = FlutterWindow::new(window).unwrap();
    let flutter = flutter.with_resource_context().unwrap();

    flutter
        .add_plugin(calc_channel::CalcPlugin::default())
        .add_plugin(msg_stream_channel::MsgStreamPlugin::default());

    flutter
        .start_engine(&PathBuf::from(asset_dir), &[], snapshot)
        .unwrap();

    flutter.with_plugin(|p: &settings::SettingsPlugin| {
        p.start_message()
            .set_text_scale_factor(1.0)
            .set_platform_brightness(settings::PlatformBrightness::Dark)
            .send();
    });

    flutter.run();
}
