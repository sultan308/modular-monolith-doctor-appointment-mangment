use anyhow::Result;
use bson::oid::ObjectId;
use crate::core::appointment_entity::Appointment;
use crate::core::appointments_repository_trait::AppointmentsRepositoryTrait;

pub struct AppointmentManagementService {
    appointments_repository: Box<dyn AppointmentsRepositoryTrait>
}

impl AppointmentManagementService {
    pub fn new(appointments_repository: Box<dyn AppointmentsRepositoryTrait>) -> AppointmentManagementService {
        AppointmentManagementService { appointments_repository }
    }
}

impl AppointmentManagementService {
    async fn cancel_appointment(&self, appointment_id: ObjectId) -> Result<Appointment> {
        let mut appointment = self.appointments_repository.get_appointment(appointment_id).await?;
        appointment.canceled();
        self.appointments_repository.save_appointment_status(&appointment).await?;
        Ok(appointment)
    }
    async fn complete_appointment(&self, appointment_id: ObjectId) -> Result<Appointment> {
        let mut appointment = self.appointments_repository.get_appointment(appointment_id).await?;
        appointment.complete();
        self.appointments_repository.save_appointment_status(&appointment).await?;
        Ok(appointment)
    }

    async fn get_all_upcoming_appointment(&self, doctor_id: ObjectId) -> Result<Vec<Appointment>> {
        let appointment = self.appointments_repository.get_doctor_appointments(doctor_id).await?;
         Ok(appointment)
    }
}