use anyhow::Result;
use bson::DateTime;
use bson::oid::ObjectId;
use crate::core::appointment_entity::Appointment;
use crate::core::appointments_repository_trait::{AppointmentsFilter, AppointmentsRepositoryTrait};

pub struct AppointmentManagementService {
    appointments_repository: Box<dyn AppointmentsRepositoryTrait>
}

impl AppointmentManagementService {
    pub fn new(appointments_repository: Box<dyn AppointmentsRepositoryTrait>) -> AppointmentManagementService {
        AppointmentManagementService { appointments_repository }
    }
}

impl AppointmentManagementService {
    pub async fn cancel_appointment(&self, appointment_id: ObjectId) -> Result<Appointment> {
        let mut appointment = self.appointments_repository.get_appointment(appointment_id).await?;
        appointment.canceled();
        self.appointments_repository.save_appointment_status(&appointment).await?;
        Ok(appointment)
    }
    pub async fn complete_appointment(&self, appointment_id: ObjectId) -> Result<Appointment> {
        let mut appointment = self.appointments_repository.get_appointment(appointment_id).await?;
        appointment.complete();
        self.appointments_repository.save_appointment_status(&appointment).await?;
        Ok(appointment)
    }

    pub async fn get_all_doctor_appointments_in_range(&self, doctor_id: ObjectId, from: Option<DateTime>, to: Option<DateTime>) -> Result<Vec<Appointment>> {
        let appointments_filter = AppointmentsFilter { doctor_id, from, to };
        let appointment = self.appointments_repository.get_doctor_appointments(appointments_filter).await?;
         Ok(appointment)
    }
}