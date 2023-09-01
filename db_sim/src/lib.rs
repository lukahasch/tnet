use std::{collections::HashMap, time::Duration, io, sync::{Arc, Mutex}};

use lib::database::{world::{Position, block::Block, TurtleID, Turtle}, DatabaseRequest};

pub struct Database {
    pub world: HashMap<Position, Block>,
    pub turtles: HashMap<TurtleID, Turtle>,
    pub claimed: HashMap<Position, Duration>,
}

impl Database {
    pub fn run() -> io::Result<()> {
        let db = Arc::new(Mutex::new(Database {
            world: HashMap::new(),
            turtles: HashMap::new(),
            claimed: HashMap::new(),
        }));
        let db2 = db.clone();
        std::thread::spawn(move || {
            let db = db2;
            let mut last_tick = std::time::Instant::now();
            loop {
                let now = std::time::Instant::now();
                let delta = now - last_tick;
                last_tick = now;
                let mut db = db.lock().unwrap();
                for (_, time) in db.claimed.iter_mut() {
                    *time -= delta;
                }
                db.claimed.retain(|_, time| time.as_millis() > 0);
                std::thread::sleep(Duration::from_millis(250));
            }
        });
        let nc = nats::connect("nats")?;
        let sub = nc.subscribe("database")?;
        while let Some(msg) = sub.next() {
            let request:DatabaseRequest = serde_json::from_slice(&msg.data).unwrap();
            match request {
                DatabaseRequest::GetTurtle(turtle_id) => {
                    let db = db.lock().unwrap();
                    let turtle = db.turtles.get(&turtle_id).cloned();
                    let data = serde_json::to_vec(&turtle).unwrap();
                    let _ = msg.respond(data);
                },
                DatabaseRequest::NewTurtleID(_) => {
                    let uuid = uuid::Uuid::new_v4().as_u128();
                    let data = serde_json::to_vec(&uuid).unwrap();
                    let _ = msg.respond(data);
                },
                DatabaseRequest::SetTurtle(turtle) => {
                    let mut db = db.lock().unwrap();
                    db.turtles.insert(turtle.uuid, turtle);
                },
                DatabaseRequest::NextCommand(turtle_id) => {
                    let db = db.lock().unwrap();
                    match db.turtles.get(&turtle_id) {
                        None => {
                            let data = serde_json::to_vec(&None::<()>).unwrap();
                            let _ = msg.respond(data);
                        },
                        Some(turtle) => {
                            let command = turtle.commands.first().cloned();
                            let data = serde_json::to_vec(&command).unwrap();
                            let _ = msg.respond(data);
                        },
                    }
                },
                DatabaseRequest::SetBlock(position, block) => {
                    let mut db = db.lock().unwrap();
                    db.world.insert(position, block);
                },
                DatabaseRequest::GetBlock(position) => {
                    let db = db.lock().unwrap();
                    let block = db.world.get(&position).cloned();
                    let data = serde_json::to_vec(&block).unwrap();
                    let _ = msg.respond(data);
                },
                DatabaseRequest::NewProzessID(_) => {
                    let uuid = uuid::Uuid::new_v4().as_u128();
                    let data = serde_json::to_vec(&uuid).unwrap();
                    let _ = msg.respond(data);
                },
                DatabaseRequest::ClaimTurtle(turtle_id, prozess_id) => {
                    let mut db = db.lock().unwrap();
                    match db.turtles.get_mut(&turtle_id) {
                        None => {
                            let data = serde_json::to_vec(&false).unwrap();
                            let _ = msg.respond(data);
                        },
                        Some(turtle) => {
                            if turtle.claim.is_none() {
                                turtle.claim = Some(prozess_id);
                                let data = serde_json::to_vec(&true).unwrap();
                                let _ = msg.respond(data);
                            } else {
                                let data = serde_json::to_vec(&false).unwrap();
                                let _ = msg.respond(data);
                            }
                        },
                    }
                },
                DatabaseRequest::ReleaseTurtle(_) => todo!(),
                DatabaseRequest::GetTurtleProzess(_) => todo!(),
                DatabaseRequest::AddTurtleCommand(_, _) => todo!(),
                DatabaseRequest::GetTurtleCommands(_) => todo!(),
                DatabaseRequest::ClearTurtleCommands(_) => todo!(),
                DatabaseRequest::ClaimPosition(_, _) => todo!(),
                DatabaseRequest::ReleasePosition(_) => todo!(),
                DatabaseRequest::GetClaimTime(_) => todo!(),
            }
        };
        Ok(())
    }
}