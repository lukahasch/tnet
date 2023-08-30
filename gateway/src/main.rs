use std::sync::Mutex;
use std::sync::atomic::AtomicPtr;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use gateway::{Event, RegisterTurtle};
use lib::database::world::{Command, Turtle};
use lib::database::Database;
use lib::events::{BlockEvent, EventStream, LogEvent, TurtleEvent};
#[macro_use]
extern crate lazy_static;

//use lazy static to initialize NATS connection

lazy_static! {
    static ref NATS:AtomicPtr<Mutex<nats::Connection>> = AtomicPtr::new({
        let nc = Mutex::new(nats::connect("nats").unwrap());
        Box::into_raw(Box::new(nc))
    });
}

#[inline]
fn nc() -> std::sync::MutexGuard<'static, nats::Connection> {
    unsafe { &mut *NATS.load(std::sync::atomic::Ordering::Relaxed) }.lock().unwrap()
}

#[tokio::main]
async fn main() {
    let ip = String::from("127.0.0.1");
    let port = String::from("8080");
    EventStream::Logs
        .publish(&mut nc(), LogEvent::Log("[Gateway] starting up".to_string()))
        .unwrap();
    let router = axum::Router::new()
        .route("/register", post(register))
        .route("/command/:turtle_id", get(command))
        .route("/update/:turtle_id", post(update));
    axum::Server::bind(&format!("{}:{}", ip, port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn update(
    Path(turtle_id):Path<String>,
    Json(event): Json<Event>,
) -> Result<StatusCode, StatusCode> {
    let turtle_id = match turtle_id.parse::<u128>() {
        Ok(turtle_id) => turtle_id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    match event {
        Event::Move(from, to) => {
            EventStream::Turtles
                .publish(&mut nc(), TurtleEvent::Move(turtle_id, from, to))
                .unwrap();
        }
        Event::FinishedCommand => {
            EventStream::Turtles
                .publish(&mut nc(), TurtleEvent::FinishedCommand(turtle_id))
                .unwrap();
        }
        Event::Inventory(inventory) => {
            EventStream::Turtles
                .publish(&mut nc(), TurtleEvent::Inventory(turtle_id, inventory))
                .unwrap();
        }
        Event::Fuel(fuel) => {
            EventStream::Turtles
                .publish(&mut nc(), TurtleEvent::Fuel(turtle_id, fuel))
                .unwrap();
        }
        Event::Direction(direction) => {
            EventStream::Turtles
                .publish(&mut nc(), TurtleEvent::Direction(turtle_id, direction))
                .unwrap();
        }
        Event::Block(position, block) => {
            EventStream::Blocks
                .publish(&mut nc(), BlockEvent::Update(position, block))
                .unwrap();
        }
    }
    Ok(StatusCode::OK)
}

async fn register(Json(register): Json<RegisterTurtle>) -> Result<String, StatusCode> {
    let new_uuid = match Database::NewTurtleID.request(&mut nc()) {
        Some(new_uuid) => new_uuid,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let turtle = Turtle {
        uuid: new_uuid,
        position: register.position,
        direction: register.direction,
        inventory: register.inventory,
        fuel: register.fuel,
    };
    Database::UpdateTurtle(turtle)
        .publish(&mut nc())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    EventStream::Turtles
        .publish(&mut nc(), TurtleEvent::Register(new_uuid))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(new_uuid.to_string())
}


async fn command(Path(turtle_id):Path<String>) -> Result<String, StatusCode> {
    let turtle_id = match turtle_id.parse::<u128>() {
        Ok(turtle_id) => turtle_id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let command: Option<Command> = match Database::NextCommand(turtle_id).request(&mut nc()) {
        Some(command) => command,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    match command {
        Some(command) => Ok(serde_json::to_string(&command).unwrap()),
        None => Err(StatusCode::NO_CONTENT),
    }
}
