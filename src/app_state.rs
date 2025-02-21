use std::sync::Arc;
use doctor_availability::controllers::{DoctorsController, SlotsController};
use futures::lock::Mutex;
use appointment_booking::controllers::{AppointmentsController, PatientController};
use doctor_appointment_management::DoctorAppointmentsManagementController;

#[derive(Clone)]
pub struct AppState {
    pub appointments_controller: Arc<Mutex<AppointmentsController>>,
    pub doctor_appointments_management_controller:Arc<Mutex<DoctorAppointmentsManagementController>>,
    pub doctors_controller: Arc<Mutex<DoctorsController>>,
    pub doctor_slots_controller: Arc<Mutex<SlotsController>>,
    pub patient_controllers: Arc<Mutex<PatientController>>,

}

impl AppState {
    pub fn with_mongo_db( db: &mongodb::Database) -> Self {
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
}