mod routers;

use std::env;
use std::sync::{Arc};
use dotenv::dotenv;
use futures::lock::Mutex;
use appointment_booking::controllers::{AppointmentsController, PatientController};
use doctor_appointment_management::{DoctorAppointmentsManagementController};
use doctor_availability::controllers::{DoctorsController, SlotsController};
use mongodb::{Client, Database};
use mongodb::bson::DateTime;
use routers::{get_doctors_router, get_doctor_slots_router, get_patients_router};

#[derive(Clone)]
struct AppState {
    pub appointments_controller: Arc<Mutex<AppointmentsController>>,
    pub doctor_appointments_management_controller:Arc<Mutex<DoctorAppointmentsManagementController>>,
    pub doctors_controller: Arc<Mutex<DoctorsController>>,
    pub doctor_slots_controller: Arc<Mutex<SlotsController>>,
    pub patient_controllers: Arc<Mutex<PatientController>>,

}
fn get_app_state(db: &Database) -> AppState {
    let doctors_controller =  Arc::new(Mutex::new(DoctorsController::with_mongo_db(&db)));
    let doctor_appointments_management_controller = Arc::new(Mutex::new(DoctorAppointmentsManagementController::with_mongo_db(&db)));
    let doctor_slots_controller = Arc::new(Mutex::new(SlotsController::with_mongo_db(&db)));
    let patient_controllers =  Arc::new(Mutex::new(PatientController::with_mongo_db(&db)));

    let appointments_controller = Arc::new(Mutex::new(AppointmentsController::with_slots_controller_and_mongo_db(doctor_slots_controller.clone(), &db)));

    AppState {
        appointments_controller,
        doctor_appointments_management_controller,
        doctors_controller,
        doctor_slots_controller,
        patient_controllers,
    }

}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let uri = env::var("MONGO_DB_URI").unwrap();
    let client = Client::with_uri_str(uri).await.unwrap();
    let db = client.database("doctor-appointment-booking-system");

    let app_state = get_app_state(&db);

    let app = get_doctors_router(app_state.clone())
                        .merge(get_doctor_slots_router(app_state.clone()))
                        .merge(get_patients_router(app_state.clone()));
    println!("{} :: starting server...", DateTime::now());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

