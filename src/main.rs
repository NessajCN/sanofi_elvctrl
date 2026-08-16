use axum::{Router, routing::post};
use serial2::{
    SerialPort,
    rs4xx::{Rs485Config, TransceiverMode},
};
use std::{
    error::Error,
    sync::{
        Arc, Mutex,
        mpsc::{Sender, channel},
    },
    thread,
};

use crate::handler::elevator_control;
// use tracing_subscriber::fmt::format::FmtSpan;
use tracing::{error, info};

mod handler;

#[derive(Clone)]
pub struct ControlChannel {
    // Wrap mutable values in a thread-safe container like Arc + Mutex/RwLock
    sender: Arc<Mutex<Sender<Vec<u8>>>>,
}

impl ControlChannel {
    pub fn send(&self, cmd: Vec<u8>) {
        let tx = self.sender.lock().unwrap();
        let _ = tx.send(cmd);
    }
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
    let port = SerialPort::open("/dev/ttyUSB0", 9600)?;

    // Configure RS-485 transceiver mode (Linux only)
    // This enables the RS485 mode structure in the underlying OS driver
    let rs485conf = Rs485Config::new();
    let mode = TransceiverMode::Rs485(rs485conf);

    if let Err(e) = port.set_rs4xx_mode(mode) {
        error!(
            "Note: Software RS485 configuration not supported or failed on this hardware/driver: {e}"
        );
        return Err(e.into());
    }
    // Spawn an std thread to handle channel receiver
    thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            if let Err(e) = port.write(&cmd) {
                error!("Failed to send command {cmd:?} to elevator! Error: {e:?}");
            }
            if let Err(e) = port.flush() {
                error!("Failed to flush! Error: {e:?}");
            }
            let mut buffer = [0u8; 64];
            match port.read(&mut buffer) {
                Ok(0) => (),
                Ok(s) => {
                    info!("{s} bytes received after sending elevator command.");
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    error!("Error: Read timed out. No response from RS-485 device.");
                }
                Err(e) => {
                    error!("Error reading from port: {:?}", e);
                }
            }
        }
        info!("Channel closed");
    });

    let sender = Arc::new(Mutex::new(tx));
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // .route("/", get(root))
        // `POST /elevctrl` goes to `elevator_control`
        .route("/elevctrl", post(elevator_control))
        .with_state(sender);

    // run our app with hyper, listening globally on port 3030
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }
