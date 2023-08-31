use lib::database::world::block::Block;
use lib::database::world::item::Item;
use lib::database::world::{Direction, Position};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegisterTurtle {
    pub position: Position,
    pub direction: Direction,
    pub inventory: [(Item, u8); 16],
    pub fuel: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Event {
    Move(Position, Position),
    FinishedCommand,
    Inventory([(Item, u8); 16]),
    Fuel(u64),
    Direction(Direction),
    Block(Position, Block),
    Log(String),
}
