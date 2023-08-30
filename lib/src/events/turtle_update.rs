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
