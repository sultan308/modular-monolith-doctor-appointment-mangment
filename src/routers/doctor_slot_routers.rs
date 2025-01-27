use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get};
use mongodb::bson::{oid::ObjectId};
use doctor_availability::payloads::{AddSlotPayload};
use doctor_availability::responses::ResponseDoctorSlot;
use crate::AppState;

pub fn get_doctor_slots_router(app_state: AppState) -> Router {
    let  doctors_router= Router::new()
        .route("/doctors/{doctor_id}/slots",get(get_doctor_slots).post(add_doctor_slot))
        .with_state(app_state);
    doctors_router
}

async fn add_doctor_slot(State(app_state): State<AppState>,
                         Path(doctor_id): Path<ObjectId>,
                         Json(create_doctor_payload): Json<AddSlotPayload>) -> (StatusCode, Json<ResponseDoctorSlot>){

    let mut doctor_slots_controller = app_state.doctor_slots_controller.lock().await;
    let response_doctor_slot = doctor_slots_controller.add_to_doctor(doctor_id,create_doctor_payload).await.unwrap();
    (StatusCode::CREATED, Json::from(response_doctor_slot))
}
async fn get_doctor_slots(State(app_state): State<AppState>,Path(doctor_id): Path<ObjectId>,) -> (StatusCode, Json<Vec<ResponseDoctorSlot>>){
    let doctor_slots_controller = app_state.doctor_slots_controller.lock().await;
    let response_slots = doctor_slots_controller.get_all(doctor_id).await.unwrap();
    (StatusCode::OK, Json::from(response_slots))
}


