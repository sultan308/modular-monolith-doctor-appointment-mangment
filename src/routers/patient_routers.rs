use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get, post};

use mongodb::bson::{oid::ObjectId};

use appointment_booking::payloads::patient_payloads::CreatePatientPayload;
use appointment_booking::responses::ResponsePatient;

use crate::AppState;

pub fn get_patients_router(app_state: AppState) -> Router {
    let  patients_router= Router::new()
        .route("/patient", post(create_patient_handler))
        .route("/patient/{patient_id}", get(get_patient_by_id_handler))
        .with_state(app_state);
    patients_router
}

async fn create_patient_handler(State(app_state): State<AppState>,
                                   Json(create_patient_payload): Json<CreatePatientPayload>) -> (StatusCode, Json<ResponsePatient>){

        let mut patients_controller = app_state.patient_controllers.lock().await;
        let response_patient = patients_controller.create_patient(create_patient_payload).await.unwrap();
        (StatusCode::CREATED, Json::from(response_patient))
}

async fn get_patient_by_id_handler(State(app_state): State<AppState>,
                                Path(patient_id): Path<ObjectId>) -> (StatusCode, Json<ResponsePatient>){

    let mut patients_controller = app_state.patient_controllers.lock().await;
    let response_patient = patients_controller.get_patient(patient_id).await.unwrap();
    (StatusCode::OK, Json::from(response_patient))
}

