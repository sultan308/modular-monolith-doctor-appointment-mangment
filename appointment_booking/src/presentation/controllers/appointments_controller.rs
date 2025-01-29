use anyhow::Result;
use bson::oid::ObjectId;
use futures::lock::Mutex;
use std::sync::Arc;

use doctor_availability::controllers::SlotsController;

use crate::usecases::{GetAvailableSlotsUseCase,AvailableSlotResponse};
use crate::presentation::payloads::patient_payloads::CreatePatientPayload;

pub struct AppointmentsController {
    get_available_slots_use_case: GetAvailableSlotsUseCase,
}
impl AppointmentsController {
    pub fn with_slots_controller(slots_controller: Arc<Mutex<SlotsController>>) -> AppointmentsController {
        AppointmentsController {
            get_available_slots_use_case: GetAvailableSlotsUseCase::with_slots_controller(slots_controller),
        }
    }
}
impl AppointmentsController {
    pub async fn get_available_appointments (&self, _patient_id: ObjectId) ->  Result<Vec<AvailableSlotResponse>>  {
        let available_appointments = self.get_available_slots_use_case.invoke().await?;
        Ok(available_appointments)
    }
}