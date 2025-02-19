use futures::lock::Mutex;
use mongodb::Database;
use shared::errors::{ApplicationError, ApplicationResult};
use std::sync::Arc;

use doctor_availability::controllers::SlotsController;

use crate::domain::{AppointmentBookingError, PatientAppointmentRepositoryTrait};
use crate::infrastructure::DoctorAvailabilityPatientAppointmentRepository;
use crate::usecases::responses::AvailableSlotResponse;

pub struct GetAvailableSlotsUseCase{
    patient_appointment_repository: Box<dyn PatientAppointmentRepositoryTrait>
}

impl GetAvailableSlotsUseCase {
    pub fn with_slots_controller(slots_controller: Arc<Mutex<SlotsController>>, mongodb: &Database) -> GetAvailableSlotsUseCase {
        GetAvailableSlotsUseCase{
            patient_appointment_repository: Box::new(DoctorAvailabilityPatientAppointmentRepository::new(slots_controller, mongodb))
        }
    }
}

impl GetAvailableSlotsUseCase{
    pub async fn invoke(&self) -> ApplicationResult<Vec<AvailableSlotResponse>>{
        let available_slots = self.patient_appointment_repository
            .get_all_bookable_slots()
            .await
            .map_err(|appointment_booking_error: AppointmentBookingError| ApplicationError::InternalServerError(Box::new(appointment_booking_error)))?;;
        Ok(available_slots.into_iter().map(AvailableSlotResponse::from).collect())
    }
}