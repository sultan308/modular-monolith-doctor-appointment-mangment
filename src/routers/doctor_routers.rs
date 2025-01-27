use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get};
use mongodb::bson::{oid::ObjectId};
use doctor_availability::payloads::{CreateDoctorPayload, UpdateDoctorPayload};
use doctor_availability::responses::ResponseDoctor;
use crate::AppState;
pub fn get_doctors_router(app_state: AppState) -> Router {
    let  doctors_router= Router::new()
        .route("/", get(|| async { "Hello, World!" }))

        .route("/doctors", get(get_doctors)
            .post(create_doctor_handler))

        .route("/doctors/{doctor_id}", get(get_doctor_handler)
            .put(update_doctor_handler)
            .delete(delete_doctor_handler))
        .with_state(app_state);
    doctors_router
}

async fn create_doctor_handler(State(app_state): State<AppState>,
                               Json(create_doctor_payload): Json<CreateDoctorPayload>) -> (StatusCode, Json<ResponseDoctor>){
    let mut doctors_controller = app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.create_doctor(create_doctor_payload).await.unwrap();
    (StatusCode::CREATED, Json::from(response_doctor))
}

async fn update_doctor_handler(State(app_state): State<AppState>,
                               Path(doctor_id): Path<ObjectId>,
                               Json(update_doctor_payload): Json<UpdateDoctorPayload>) -> (StatusCode, Json<ResponseDoctor>){
    let mut doctors_controller = app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.update_doctor(doctor_id , update_doctor_payload).await.unwrap();
    (StatusCode::OK, Json::from(response_doctor))
}

async fn get_doctor_handler(State(app_state): State<AppState>,
                            Path(doctor_id): Path<ObjectId>) -> (StatusCode, Json<ResponseDoctor>){
    let doctors_controller = &app_state.doctors_controller.lock().await;
    let response_doctor = doctors_controller.get_by_id(doctor_id).await.unwrap();
    (StatusCode::OK, Json::from(response_doctor))
}

async fn get_doctors(State(app_state): State<AppState>) -> (StatusCode, Json<Vec<ResponseDoctor>>){
    let doctors_controller = &app_state.doctors_controller.lock().await;
    let response_doctors = doctors_controller.get_all().await.unwrap();
    (StatusCode::OK, Json::from(response_doctors))
}

async fn delete_doctor_handler(State(app_state): State<AppState>,
                               Path(doctor_id): Path<ObjectId>) -> StatusCode{
    let mut doctors_controller = app_state.doctors_controller.lock().await;
    doctors_controller.delete_by_id(doctor_id).await.unwrap();
    StatusCode::NO_CONTENT
}
