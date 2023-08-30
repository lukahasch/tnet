pub mod world;
use self::world::{Turtle, TurtleID};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Database {
    NewTurtleID,
    UpdateTurtle(Turtle),
    NextCommand(TurtleID),
}

impl Database {
    pub fn request<R: DeserializeOwned>(&self, nc: &mut nats::Connection) -> Option<R> {
        let subject = "database";
        let request = match serde_json::to_vec(self) {
            Ok(request) => request,
            Err(_) => return None,
        };
        let response = match nc.request(subject, request) {
            Ok(response) => response,
            Err(_) => return None,
        };
        match serde_json::from_slice(&response.data) {
            Ok(response) => Some(response),
            Err(_) => None,
        }
    }

    pub fn publish(&self, nc: &mut nats::Connection) -> io::Result<()> {
        let subject = "database";
        let data = serde_json::to_vec(self)?;
        nc.publish(subject, data)
    }
}
