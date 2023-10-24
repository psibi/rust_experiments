use anyhow::Result;
use axum::{response::Html, routing::get, Router, extract::State};
use tokio::sync::Mutex;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tracing::info;

struct App {
    should_log: Mutex<bool>,
}

impl App {
    pub(crate) async fn run(app: Arc<App>) -> Result<()> {
        info!("In run method");
        while *app.should_log.lock().await {
            info!("Log from run application");
            tokio::time::sleep(Duration::from_secs(2)).await
        }
	info!("Stopping the log permanently");
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Arc::new(App { should_log: Mutex::new(true) });

    let web_app = Router::new()
        .route("/", get(handler))
        .route("/stop_log", get(stop_log))
        .with_state(app.clone());
    tracing::info!("Main application");

    tokio::spawn(async { App::run(app).await });

    // run it
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(web_app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn stop_log(State(input): State<Arc<App>>) -> Html<&'static str> {
    let mut should_log = input.should_log.lock().await;
    *should_log = false;
    Html("<h1>Stopped log</h1>")
}
