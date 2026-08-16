use axum::{
    Router,
    routing::{get, post},
};
use std::{error::Error, sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}}};

use crate::handler::elevator_control;
// use tracing_subscriber::fmt::format::FmtSpan;
// use tracing::{error, info};

mod handler;

#[derive(Clone)]
pub struct ControlChannel {
    // Wrap mutable values in a thread-safe container like Arc + Mutex/RwLock
    sender: Arc<Mutex<Sender<Vec<u8>>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // initialize tracing
    tracing_subscriber::fmt()
        .compact()
        // .with_timer(ChronoLocal::new(String::from("[%F %T]")))
        .without_time()
        .with_target(false)
        .init();

    let (tx, rx) = channel::<Vec<u8>>();
    let sender = Arc::new(Mutex::new(tx));
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /elevctrl` goes to `elevator_control`
        .route("/elevctrl", post(elevator_control))
        .with_state(sender);

    // run our app with hyper, listening globally on port 3030
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
