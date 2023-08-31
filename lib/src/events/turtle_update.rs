use crate::database::world::{item::Item, Direction, Position, TurtleID};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum TurtleEvent {
    Move(TurtleID, Position, Position),
    FinishedCommand(TurtleID),
    Inventory(TurtleID, [(Item, u8); 16]),
    Fuel(TurtleID, u64),
    Direction(TurtleID, Direction),
    Register(TurtleID),
}

impl TurtleEvent {
    pub fn publish(&self, nc: &mut nats::Connection) -> std::io::Result<()> {
        let subject = "turtle_updates";
        let data = serde_json::to_vec(self)?;
        nc.publish(subject, data)
    }
}
