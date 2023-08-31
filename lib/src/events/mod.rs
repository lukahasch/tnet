pub mod block_update;
pub mod log_update;
pub mod turtle_update;

pub use block_update::BlockEvent;
pub use log_update::LogEvent;
use serde::de::DeserializeOwned;
pub use turtle_update::TurtleEvent;

pub enum EventStream {
    Blocks,
    Turtles,
    Logs,
}

impl EventStream {
    pub fn topic(&self) -> &'static str {
        match self {
            EventStream::Blocks => "block_updates",
            EventStream::Turtles => "turtle_updates",
            EventStream::Logs => "log_updates",
        }
    }

    pub fn listen<R: DeserializeOwned>(&self, nc: &mut nats::Connection) -> Option<R> {
        let sub = match nc.subscribe(self.topic()) {
            Ok(sub) => sub,
            Err(_) => return None,
        };
        let msg = match sub.next() {
            Some(msg) => msg,
            None => return None,
        };
        match serde_json::from_slice(&msg.data) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }
}
