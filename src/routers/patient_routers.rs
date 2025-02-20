use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::extract::rejection::JsonRejection;
use axum::routing::{get, post};

use mongodb::bson::{oid::ObjectId};
use shared::errors::{ApplicationResult, ApplicationError};

use appointment_booking::payloads::patient_payloads::CreatePatientPayload;
use appointment_booking::responses::{AvailableSlotResponse, ResponsePatient, PatientAppointmentResponse};

use crate::AppState;

pub fn get_patients_router(app_state: AppState) -> Router {
    let  patients_router= Router::new()
        .route("/patient", post(create_patient_handler))
        .route("/patient/{patient_id}", get(get_patient_by_id_handler))
        .route("/patient/{patient_id}/appointments", get(get_patient_appointments))
        .route("/patient/{patient_id}/available-appointments", get(get_available_appointments))
        .route("/patient/{patient_id}/available-appointments/{slot_id}/book", post(book_patient_appointment))
        .with_state(app_state);
    patients_router
}

async fn create_patient_handler(State(app_state): State<AppState>,
                                   create_patient_payload: Result<Json<CreatePatientPayload>,JsonRejection>) -> ApplicationResult<(StatusCode, Json<ResponsePatient>)>{
        let Json(create_patient_payload) = create_patient_payload.map_err(|e| ApplicationError::InvalidRequestPayload("Invalid create patient body".to_string(), Box::new(e)))?;
        let mut patients_controller = app_state.patient_controllers.lock().await;
        let response_patient = patients_controller.create_patient(create_patient_payload).await?;
        Ok((StatusCode::CREATED, Json::from(response_patient)))
}

async fn get_patient_by_id_handler(State(app_state): State<AppState>,
                                Path(patient_id): Path<ObjectId>) -> ApplicationResult<(StatusCode, Json<ResponsePatient>)>{

    let mut patients_controller = app_state.patient_controllers.lock().await;
    let response_patient = patients_controller.get_patient(patient_id).await?;
    Ok((StatusCode::OK, Json::from(response_patient)))
}
async fn get_patient_appointments(State(app_state): State<AppState>,
                                   Path(patient_id): Path<ObjectId>) -> ApplicationResult<(StatusCode, Json<Vec<PatientAppointmentResponse>>)>{

    let appointments_controller = app_state.appointments_controller.lock().await;
    let patient_appointments = appointments_controller.get_patient_booked_appointments(patient_id).await?;
    Ok((StatusCode::OK, Json::from(patient_appointments)))
}

async fn get_available_appointments(State(app_state): State<AppState>,
                                   Path(patient_id): Path<ObjectId>) -> ApplicationResult<(StatusCode, Json<Vec<AvailableSlotResponse>>)>{

    let appointments_controller = app_state.appointments_controller.lock().await;
    let response_patient = appointments_controller.get_available_appointments(patient_id).await?;
    Ok((StatusCode::OK, Json::from(response_patient)))
}

async fn book_patient_appointment(State(app_state): State<AppState>,
                                   Path((patient_id, slot_id)): Path<(ObjectId, ObjectId)>
                                    ) -> ApplicationResult<(StatusCode, Json<PatientAppointmentResponse>)>{

    let mut appointments_controller = app_state.appointments_controller.lock().await;
    let response_booked_patient_appointment = appointments_controller.book_appointment(patient_id, slot_id).await?;
    Ok((StatusCode::CREATED, Json::from(response_booked_patient_appointment)))
}



