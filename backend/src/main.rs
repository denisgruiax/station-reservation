use tokio::net::TcpListener;
use axum::{Router, routing::get, extract::State, Json};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

mod models;
use models::*;

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        stations: Arc::new(Mutex::new(vec![
            Station { id: 1, name: "Alpha".into(), reserved: false },
            Station { id: 2, name: "Beta".into(), reserved: false },
        ])),
    };

    let app = Router::new()
        .route("/api/stations", get(get_stations))
        .with_state(shared_state)
        .fallback_service(ServeDir::new("../frontend")); // for static files

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_stations(State(state): State<AppState>) -> Json<Vec<Station>> {
    let stations = state.stations.lock().unwrap().clone();
    Json(stations)
}
