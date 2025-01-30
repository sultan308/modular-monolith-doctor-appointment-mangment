use anyhow::Result;
use bson::oid::ObjectId;
use futures::lock::Mutex;
use std::sync::Arc;

use doctor_availability::controllers::SlotsController;
use crate::infrastructure::MongoDataBase;
use crate::usecases::{GetAvailableSlotsUseCase,
                      BookPatientAppointmentUseCase,
                      PatientAppointmentResponse,
                      AvailableSlotResponse};

pub struct AppointmentsController {
    get_available_slots_use_case: GetAvailableSlotsUseCase,
    book_patient_appointment_use_case: BookPatientAppointmentUseCase

}
impl AppointmentsController {
    pub fn with_slots_controller_and_mongo_db(slots_controller: Arc<Mutex<SlotsController>>, mongo_data_base: &MongoDataBase) -> AppointmentsController {
        AppointmentsController {
            book_patient_appointment_use_case:BookPatientAppointmentUseCase::with_slots_controller_and_mongo_db(slots_controller.clone(), mongo_data_base),
            get_available_slots_use_case: GetAvailableSlotsUseCase::with_slots_controller(slots_controller)
        }
    }
}
impl AppointmentsController {
    pub async fn get_available_appointments (&self, _patient_id: ObjectId) ->  Result<Vec<AvailableSlotResponse>>  {
        let available_appointments = self.get_available_slots_use_case.invoke().await?;
        Ok(available_appointments)
    }
    pub async fn get_patient_booked_appointments (&self, patient_id: ObjectId) ->  Result<Vec<AvailableSlotResponse>>  {
        let available_appointments = self.get_available_slots_use_case.invoke().await?;
        Ok(available_appointments)
    }
    pub async fn book_appointment (&mut self, patient_id: ObjectId, slot_id: ObjectId ) ->  Result<PatientAppointmentResponse>  {
        let patient_appointment = self.book_patient_appointment_use_case.invoke(patient_id,slot_id).await?;
        Ok(patient_appointment)
    }
}