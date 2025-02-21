mod routers;
mod app_state;

use std::env;
use dotenv::dotenv;
use mongodb::{Client, Database};
use mongodb::bson::DateTime;
use routers::{get_doctor_slots_router, get_doctors_router, get_patients_router};
use app_state::AppState;

async fn connect_to_mongo_db() -> Database {
    dotenv().ok();
    let uri = env::var("MONGO_DB_URI")
        .unwrap_or_else(|err| panic!("Failed to load MONGO_DB_URI from env vars: {}", err));

    let client = Client::with_uri_str(uri).await
        .unwrap_or_else(|err| panic!("Failed to connect to MONGO_DB_URI : {}", err));

    client.database("doctor-appointment-booking-system")

}

#[tokio::main]
async fn main() {
    let mongodb = connect_to_mongo_db().await;
    println!("{} :: connected to mongo db", DateTime::now());

    let app_state = AppState::with_mongo_db(&mongodb);
    println!("{} :: app state loaded", DateTime::now());

    let app = get_doctors_router(app_state.clone())
                        .merge(get_doctor_slots_router(app_state.clone()))
                        .merge(get_patients_router(app_state.clone()));

    println!("{} :: starting server...", DateTime::now());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("{} :: listening on http://localhost:{}", DateTime::now(), port);
    axum::serve(listener, app).await.unwrap();
}

