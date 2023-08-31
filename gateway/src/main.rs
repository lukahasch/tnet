use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use gateway::{Event, RegisterTurtle};
use lib::database::world::Turtle;
use lib::events::{BlockEvent, LogEvent, TurtleEvent};
use lib::{request, update_turtle};
use std::sync::atomic::AtomicPtr;
use std::sync::Mutex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NATS: AtomicPtr<Mutex<nats::Connection>> = AtomicPtr::new({
        let nc = Mutex::new(nats::connect("127.0.0.1").unwrap());
        Box::into_raw(Box::new(nc))
    });
}

#[inline]
fn nc() -> std::sync::MutexGuard<'static, nats::Connection> {
    unsafe { &mut *NATS.load(std::sync::atomic::Ordering::Relaxed) }
        .lock()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let ip = String::from("127.0.0.1");
    let port = String::from("8080");
    let router = axum::Router::new()
        .route("/register", post(register))
        .route("/command/:turtle_id", get(command))
        .route("/update/:turtle_id", post(update));
    tokio::join!(
        axum::Server::bind(&format!("{}:{}", ip, port).parse().unwrap())
            .serve(router.into_make_service()),
        async move {
            LogEvent::Log("Gateway started".to_string())
                .publish(&mut nc())
                .unwrap();
        },
    )
    .0
    .unwrap();
}

async fn update(
    Path(turtle_id): Path<String>,
    Json(event): Json<Event>,
) -> Result<StatusCode, StatusCode> {
    let turtle_id = match turtle_id.parse::<u128>() {
        Ok(turtle_id) => turtle_id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    match event {
        Event::Block(position, block) => {
            request!(SetBlock, &mut nc(), position, block)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            BlockEvent::Update(position, block)
                .publish(&mut nc())
                .unwrap();
        }
        Event::Log(msg) => {
            LogEvent::Log(msg).publish(&mut nc()).unwrap();
        }
        Event::Move(from, to) => {
            update_turtle!(
                &mut nc(),
                turtle_id,
                position => to,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            TurtleEvent::Move(turtle_id, from, to)
                .publish(&mut nc())
                .unwrap();
        }
        Event::Direction(direction) => {
            update_turtle!(
                &mut nc(),
                turtle_id,
                direction => direction,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            TurtleEvent::Direction(turtle_id, direction)
                .publish(&mut nc())
                .unwrap();
        }
        Event::Fuel(fuel) => {
            update_turtle!(
                &mut nc(),
                turtle_id,
                fuel => fuel,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            TurtleEvent::Fuel(turtle_id, fuel)
                .publish(&mut nc())
                .unwrap();
        }
        Event::Inventory(inventory) => {
            update_turtle!(
                &mut nc(),
                turtle_id,
                inventory => inventory,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            TurtleEvent::Inventory(turtle_id, inventory)
                .publish(&mut nc())
                .unwrap();
        }
        Event::FinishedCommand => {
            TurtleEvent::FinishedCommand(turtle_id)
                .publish(&mut nc())
                .unwrap();
        }
    }
    Ok(StatusCode::OK)
}

async fn register(Json(register): Json<RegisterTurtle>) -> Result<String, StatusCode> {
    let turtle_id =
        request!(NewTurtleID, &mut nc(), ()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let turtle = Turtle {
        uuid: turtle_id,
        position: register.position,
        fuel: register.fuel,
        inventory: register.inventory,
        direction: register.direction,
    };
    request!(SetTurtle, &mut nc(), turtle).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    TurtleEvent::Register(turtle_id).publish(&mut nc()).unwrap();
    Ok(turtle_id.to_string())
}

async fn command(Path(turtle_id): Path<String>) -> Result<String, StatusCode> {
    let turtle_id = match turtle_id.parse::<u128>() {
        Ok(turtle_id) => turtle_id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let command = request!(NextCommand, &mut nc(), turtle_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(serde_json::to_string(&command).unwrap())
}
