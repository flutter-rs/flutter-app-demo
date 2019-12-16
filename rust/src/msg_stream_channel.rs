use flutter_plugins::prelude::*;
use futures::compat::Future01CompatExt;
use futures::future::FutureExt;
use log::info;
use std::iter::repeat;
use std::time::Duration;
use stream_cancel::{StreamExt as StreamExt2, Trigger, Tripwire};
use tokio::prelude::*;

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
            handler: Arc::new(RwLock::new(Handler { stop_trigger: None })),
        }
    }
}

struct Handler {
    stop_trigger: Option<Trigger>,
}

impl EventHandler for Handler {
    fn on_listen(
        &mut self,
        args: Value,
        engine: FlutterEngine,
    ) -> Result<Value, MethodCallError> {
        if let Value::I32(n) = args {
            info!("Random stream invoked with params {}", n);
        }

        let (trigger, tripwire) = Tripwire::new();
        self.stop_trigger = Some(trigger);

        let rt = engine.clone();
        engine.run_in_background(future::lazy(move || {
            let v = vec![
                "Hello?",
                "What's your name?",
                "How old are you?",
                "Maybe we can be friend together...",
                "Do you have a brother or sister?",
            ];
            stream::iter_ok::<_, ()>(repeat(v).flatten())
                .throttle(Duration::from_secs(1))
                .map_err(|e| eprintln!("Error = {:?}", e))
                .take_until(tripwire)
                .for_each(move |v| {
                    rt.with_channel(CHANNEL_NAME, move |channel| {
                        if let Some(channel) = channel.try_as_method_channel() {
                            let ret = Value::String(String::from(v));
                            channel.send_success_event(&ret);
                        }
                    });
                    Ok(())
                })
        }).into_future().compat().map(|_| ()));
        Ok(Value::Null)
    }

    fn on_cancel(&mut self, _: FlutterEngine) -> Result<Value, MethodCallError> {
        // drop the trigger to stop stream
        self.stop_trigger.take();
        Ok(Value::Null)
    }
}
