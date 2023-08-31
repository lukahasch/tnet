use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogEvent {
    Log(String),
}

impl LogEvent {
    pub fn publish(&self, nc: &mut nats::Connection) -> std::io::Result<()> {
        let subject = "log_updates";
        let data = serde_json::to_vec(self)?;
        nc.publish(subject, data)
    }
}

pub fn log(nc: &mut nats::Connection, msg: String) {
    LogEvent::Log(msg).publish(nc).unwrap();
}
