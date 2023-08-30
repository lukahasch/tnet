use crate::database::world::item::Item;
use serde_derive::{Deserialize, Serialize};

pub mod block;
pub mod item;

pub type Position = (i32, i32, i32, i32);
pub type TurtleID = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turtle {
    pub position: Position,
    pub direction: Direction,
    pub inventory: [(Item, u8); 16],
    pub fuel: u64,
    pub uuid: TurtleID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    
}
