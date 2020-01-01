use async_std::task;
use flutter_plugins::prelude::*;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

const PLUGIN_NAME: &str = module_path!();
const CHANNEL_NAME: &str = "rust/msg_stream";

pub struct MsgStreamPlugin {
    channel: Weak<EventChannel>,
    handler: Arc<RwLock<Handler>>,
}

impl Plugin for MsgStreamPlugin {
    fn plugin_name() -> &'static str {
        PLUGIN_NAME
    }

    fn init_channels(&mut self, registrar: &mut ChannelRegistrar) {
        let event_handler = Arc::downgrade(&self.handler);
        self.channel = registrar.register_channel(EventChannel::new(CHANNEL_NAME, event_handler));
    }
}

impl Default for MsgStreamPlugin {
    fn default() -> Self {
        Self {
            channel: Weak::new(),
            handler: Arc::new(RwLock::new(Handler {
                stop_trigger: Default::default(),
            })),
        }
    }
}

struct Handler {
    stop_trigger: Arc<AtomicBool>,
}

impl EventHandler for Handler {
    fn on_listen(&mut self, args: Value, engine: FlutterEngine) -> Result<Value, MethodCallError> {
        if let Value::I32(n) = args {
            info!("Random stream invoked with params {}", n);
        }

        let rt = engine.clone();
        let stop_trigger = Arc::new(AtomicBool::new(false));
        self.stop_trigger = stop_trigger.clone();
        engine.run_in_background(async move {
            let msgs = vec![
                "Hello?",
                "What's your name?",
                "How old are you?",
                "Maybe we can be friend together...",
                "Do you have a brother or sister?",
            ];

            loop {
                for msg in msgs.iter() {
                    task::sleep(Duration::from_secs(1)).await;
                    if stop_trigger.load(Ordering::Relaxed) {
                        break;
                    }
                    rt.with_channel(CHANNEL_NAME, move |channel| {
                        if let Some(channel) = channel.try_as_method_channel() {
                            let ret = Value::String(String::from(*msg));
                            channel.send_success_event(&ret);
                        }
                    });
                }
            }
        });
        Ok(Value::Null)
    }

    fn on_cancel(&mut self, _: FlutterEngine) -> Result<Value, MethodCallError> {
        self.stop_trigger.store(true, Ordering::Relaxed);
        Ok(Value::Null)
    }
}
