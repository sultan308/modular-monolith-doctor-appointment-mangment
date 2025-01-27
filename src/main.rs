mod routers;

use std::env;
use std::sync::{Arc};
use dotenv::dotenv;
use futures::lock::Mutex;
use doctor_availability::controllers::{DoctorsController, SlotsController};
use mongodb::{Client};
use mongodb::bson::DateTime;
use routers::{get_doctors_router, get_doctor_slots_router};

#[derive(Clone)]
struct AppState {
    pub doctors_controller: Arc<Mutex<DoctorsController>>,
    pub doctor_slots_controller: Arc<Mutex<SlotsController>>
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let uri = env::var("MONGO_DB_URI").unwrap();
    let client = Client::with_uri_str(uri).await.unwrap();
    let db = client.database("doctor-appointment-booking-system");
    let app_state = AppState{
        doctors_controller: Arc::new(Mutex::new(DoctorsController::with_mongo_db(&db))),
        doctor_slots_controller: Arc::new(Mutex::new(SlotsController::with_mongo_db(&db))),
    };
    let app = get_doctors_router(app_state.clone()).merge(get_doctor_slots_router(app_state.clone()));
    println!("{}", DateTime::now());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

