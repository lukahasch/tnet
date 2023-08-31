use serde_derive::{Deserialize, Serialize};

use crate::database::world::{block::Block, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockEvent {
    Update(Position, Block),
}

impl BlockEvent {
    pub fn publish(&self, nc: &mut nats::Connection) -> std::io::Result<()> {
        let subject = "block_updates";
        let data = serde_json::to_vec(self)?;
        nc.publish(subject, data)
    }
}
