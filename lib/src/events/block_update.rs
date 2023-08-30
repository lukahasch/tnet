use serde_derive::{Deserialize, Serialize};

use crate::database::world::{block::Block, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockEvent {
    Update(Position, Block),
}
