use axum::extract::{Path, State, Json, rejection::JsonRejection};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get,post};
use chrono::Utc;
use mongodb::bson::{oid::ObjectId};
use doctor_availability::payloads::{CreateDoctorPayload, UpdateDoctorPayload};
use doctor_availability::responses::ResponseDoctor;
use doctor_appointment_management::ResponseAppointment;
use shared::errors::{ApplicationError, ApplicationResult};

use crate::AppState;
pub fn get_doctors_router(app_state: AppState) -> Router {
    let  doctors_router= Router::new()
        .route("/", get(|| async { "Hello, World!" }))

        .route("/doctors", get(get_doctors)
            .post(create_doctor_handler))

        .route("/doctors/{doctor_id}", get(get_doctor_handler)
            .put(update_doctor_handler))
        .route("/doctors/{doctor_id}/appointments",get(get_doctor_appointments))
        .route("/doctors/{doctor_id}/appointments/{appointment_id}/cancel",post(cancel_doctor_appointment))
        .route("/doctors/{doctor_id}/appointments/{appointment_id}/complete",post(complete_doctor_appointment))
        .with_state(app_state);
    doctors_router
}

async fn create_doctor_handler(State(app_state): State<AppState>,
                               create_doctor_payload: Result<Json<CreateDoctorPayload>,JsonRejection>) -> ApplicationResult<(StatusCode, Json<ResponseDoctor>)>{
    let Json(create_doctor_payload) = create_doctor_payload.map_err(|e| ApplicationError::InvalidRequestPayload("Invalid create doctor body".to_string(), Box::new(e)))?;
    let mut doctors_controller = app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.create_doctor(create_doctor_payload).await?;
    Ok((StatusCode::CREATED, Json(response_doctor)))
}

async fn update_doctor_handler(State(app_state): State<AppState>,
                               Path(doctor_id): Path<ObjectId>,
                               update_doctor_payload: Result<Json<UpdateDoctorPayload>,JsonRejection>) -> ApplicationResult<(StatusCode, Json<ResponseDoctor>)>{

    let Json(update_doctor_payload) = update_doctor_payload.map_err(|e| ApplicationError::InvalidRequestPayload("Invalid create doctor body".to_string(), Box::new(e)))?;
    let mut doctors_controller = app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.update_doctor(doctor_id , update_doctor_payload).await?;
    Ok((StatusCode::OK, Json::from(response_doctor)))
}

async fn get_doctor_handler(State(app_state): State<AppState>,
                            Path(doctor_id): Path<ObjectId>) -> ApplicationResult<(StatusCode, Json<ResponseDoctor>)>{
    let doctors_controller = &app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.get_by_id(doctor_id).await?;
    Ok((StatusCode::OK, Json::from(response_doctor)))
}

async fn get_doctors(State(app_state): State<AppState>) -> ApplicationResult<(StatusCode, Json<Vec<ResponseDoctor>>)>{
    let doctors_controller = &app_state.doctors_controller.lock().await;
    let response_doctors = doctors_controller.get_all().await?;
    Ok((StatusCode::OK, Json::from(response_doctors)))
}
async fn get_doctor_appointments(State(app_state): State<AppState>,
                                Path(doctor_id): Path<ObjectId>) -> ApplicationResult<(StatusCode, Json<Vec<ResponseAppointment>>)>{
    let doctor_appointments_management_controller = app_state.doctor_appointments_management_controller.lock().await;
    let appointments = doctor_appointments_management_controller.get_all_doctor_appointments(doctor_id,
                                                                                                                      Some(Utc::now()),
                                                                                                                      None).await?;
    Ok((StatusCode::OK, Json::from(appointments)))
}

async fn cancel_doctor_appointment(State(app_state): State<AppState>,
                                 Path((_doctor_id, appointment_id)): Path<(ObjectId, ObjectId)>) -> ApplicationResult<(StatusCode, Json<ResponseAppointment>)> {
    let doctor_appointments_management_controller = app_state.doctor_appointments_management_controller.lock().await;
    let appointment = doctor_appointments_management_controller.cancel_appointment(appointment_id).await?;
    Ok((StatusCode::OK, Json::from(appointment)))
}
async fn complete_doctor_appointment(State(app_state): State<AppState>,
                                   Path((_doctor_id, appointment_id)): Path<(ObjectId, ObjectId)>) -> ApplicationResult<(StatusCode, Json<ResponseAppointment>)> {
    let doctor_appointments_management_controller = app_state.doctor_appointments_management_controller.lock().await;
    let appointment = doctor_appointments_management_controller.complete_appointment(appointment_id).await?;
    Ok((StatusCode::OK, Json::from(appointment)))
}
