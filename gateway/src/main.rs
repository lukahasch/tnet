use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use gateway::{Event, RegisterTurtle};
use lib::database::world::{Command, Turtle};
use lib::database::Database;
use lib::events::{BlockEvent, EventStream, LogEvent, TurtleEvent};

#[tokio::main]
async fn main() {
    let ip = String::from("127.0.0.1");
    let port = String::from("8080");
    let mut nc = nats::connect("nats").unwrap();
    EventStream::Logs
        .publish(&mut nc, LogEvent::Log("[Gateway] starting up".to_string()))
        .unwrap();
    let router = axum::Router::new()
        .route("/register", post(register))
        .route("/command", get(command))
        .route("/update", post(update));
    tokio::spawn(
        axum::Server::bind(&format!("{}:{}", ip, port).parse().unwrap())
            .serve(router.into_make_service()),
    );
}

async fn update(
    Query(turtle_id): Query<u128>,
    Json(event): Json<Event>,
) -> Result<StatusCode, StatusCode> {
    let mut nc = nats::connect("nats").unwrap();
    match event {
        Event::Move(from, to) => {
            EventStream::Turtles
                .publish(&mut nc, TurtleEvent::Move(turtle_id, from, to))
                .unwrap();
        }
        Event::FinishedCommand => {
            EventStream::Turtles
                .publish(&mut nc, TurtleEvent::FinishedCommand(turtle_id))
                .unwrap();
        }
        Event::Inventory(inventory) => {
            EventStream::Turtles
                .publish(&mut nc, TurtleEvent::Inventory(turtle_id, inventory))
                .unwrap();
        }
        Event::Fuel(fuel) => {
            EventStream::Turtles
                .publish(&mut nc, TurtleEvent::Fuel(turtle_id, fuel))
                .unwrap();
        }
        Event::Direction(direction) => {
            EventStream::Turtles
                .publish(&mut nc, TurtleEvent::Direction(turtle_id, direction))
                .unwrap();
        }
        Event::Block(position, block) => {
            EventStream::Blocks
                .publish(&mut nc, BlockEvent::Update(position, block))
                .unwrap();
        }
    }
    Ok(StatusCode::OK)
}

async fn register(Json(register): Json<RegisterTurtle>) -> Result<String, StatusCode> {
    let mut nc = nats::connect("nats").unwrap();
    let new_uuid = match Database::NewTurtleID.request(&mut nc) {
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
        .publish(&mut nc)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    EventStream::Turtles
        .publish(&mut nc, TurtleEvent::Register(new_uuid))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(new_uuid.to_string())
}

async fn command(Query(turtle_id): Query<u128>) -> Result<String, StatusCode> {
    let mut nc = nats::connect("nats").unwrap();
    let command: Option<Command> = match Database::NextCommand(turtle_id).request(&mut nc) {
        Some(command) => command,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    match command {
        Some(command) => Ok(serde_json::to_string(&command).unwrap()),
        None => Err(StatusCode::NO_CONTENT),
    }
}
