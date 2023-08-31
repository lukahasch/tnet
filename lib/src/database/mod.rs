use self::world::{Command, Turtle, TurtleID};
pub mod world;
use crate::database::world::block::Block;
use crate::database::world::Position;

macro_rules! db_comm_gem {
    (
        $(
            $name:ident : $($req:tt),* => $resp:ty;
        )*
    ) => {
        use serde_derive::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub enum DatabaseRequest {
            $(
                $name($($req),*),
            )*
        }

        $(
            #[allow(non_snake_case)]
            pub fn $name(nc:&mut nats::Connection,$($req: $req),*) -> std::io::Result<$resp> {
                let subject = "database";
                let data = serde_json::to_vec(&DatabaseRequest::$name($($req),*)).unwrap();
                let msg = nc.request(subject, data)?;
                match serde_json::from_slice(&msg.data) {
                    Ok(data) => Ok(data),
                    Err(i) => Err(std::io::Error::new(std::io::ErrorKind::Other,format!("Error parsing response: {}",i))),
                }
            }
        )*
    }
}

db_comm_gem!(
    GetTurtle: TurtleID => Option<Turtle>;
    NewTurtleID: () => TurtleID;
    SetTurtle: Turtle => ();
    NextCommand: TurtleID => Option<Command>;
    SetBlock: Position,Block => ();
);

#[macro_export]
macro_rules! request {
    ($name:ident,$nc:expr,$($arg:expr),*) => {
        {
            use lib::database::DatabaseRequest;
            use lib::database::$name;
            $name($nc,$($arg),*)
        }
    };
}

#[macro_export]
macro_rules! update_turtle {
    ($nc:expr,$id:expr,$($field:ident => $data:expr,)*) => {
        {
            move || -> std::io::Result<()> {
                let mut turtle = match request!(GetTurtle,$nc,$id)? {
                    Some(turtle) => turtle,
                    None => return Err(std::io::Error::new(std::io::ErrorKind::Other,"Turtle not found")),
                };
                $(
                    turtle.$field = $data;
                )*
                request!(SetTurtle,$nc,turtle)
            }()
        }
    };
}
