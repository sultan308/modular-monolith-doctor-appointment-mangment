use anyhow::Result;
use std::sync::Arc;
use futures::lock::Mutex;

use doctor_availability::controllers::SlotsController;

use crate::domain::{PatientAppointmentRepositoryTrait};
use crate::infrastructure::DoctorAvailabilityPatientAppointmentRepository;
use crate::usecases::responses::AvailableSlotResponse;

pub struct GetAvailableSlotsUseCase{
    patient_appointment_repository: Box<dyn PatientAppointmentRepositoryTrait>
}

impl GetAvailableSlotsUseCase {
    pub fn with_slots_controller(slots_controller: Arc<Mutex<SlotsController>>) -> GetAvailableSlotsUseCase {
        GetAvailableSlotsUseCase{
            patient_appointment_repository: Box::new(DoctorAvailabilityPatientAppointmentRepository::new(slots_controller))
        }
    }
}

impl GetAvailableSlotsUseCase{
    pub async fn invoke(&self) -> Result<Vec<AvailableSlotResponse>>{
        let available_slots = self.patient_appointment_repository.get_all_bookable_slots().await?;
        Ok(available_slots.into_iter().map(AvailableSlotResponse::from).collect())
    }
}