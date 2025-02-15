use axum::extract::{Path, State,rejection::JsonRejection};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get};
use mongodb::bson::{oid::ObjectId};
use doctor_availability::payloads::{AddSlotPayload};
use doctor_availability::responses::ResponseDoctorSlot;
use shared::errors::{ApplicationError, ApplicationResult};

use crate::AppState;

pub fn get_doctor_slots_router(app_state: AppState) -> Router {
    let  doctors_router= Router::new()
        .route("/doctors/{doctor_id}/slots",get(get_doctor_slots).post(add_doctor_slot))
        .with_state(app_state);
    doctors_router
}

async fn add_doctor_slot(State(app_state): State<AppState>,
                         Path(doctor_id): Path<ObjectId>,
                         create_slot_payload: Result<Json<AddSlotPayload>,JsonRejection>) -> ApplicationResult<(StatusCode, Json<ResponseDoctorSlot>)>{

    let Json(create_slot_payload) = create_slot_payload.map_err(|e| ApplicationError::InvalidRequestPayload("Invalid create slot body".to_string(), Box::new(e)))?;
    let mut doctor_slots_controller = app_state.doctor_slots_controller.lock().await;
    let response_doctor_slot = doctor_slots_controller.add_to_doctor(doctor_id,create_slot_payload).await?;
    Ok((StatusCode::CREATED, Json::from(response_doctor_slot)))
}
async fn get_doctor_slots(State(app_state): State<AppState>,Path(doctor_id): Path<ObjectId>,) -> ApplicationResult<(StatusCode, Json<Vec<ResponseDoctorSlot>>)>{
    let doctor_slots_controller = app_state.doctor_slots_controller.lock().await;
    let response_slots = doctor_slots_controller.get_all_slots_by_doctor(doctor_id).await?;
    Ok((StatusCode::OK, Json::from(response_slots)))
}


