use serde::{Serialize, Deserialize};

use bson::{oid::ObjectId};
use chrono::{DateTime, Utc};
use crate::domain::PatientAppointmentEntity;
#[derive(Serialize,Deserialize)]
pub struct PatientAppointmentResponse {
    pub id: ObjectId,
    pub slot_id: ObjectId,
    pub patient_id: ObjectId,
    pub patient_name: String,
    pub reserved_at: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub is_completed: bool,
    pub is_canceled: bool
}

impl PatientAppointmentResponse{
    pub fn from(patient_appointment_entity: PatientAppointmentEntity) -> PatientAppointmentResponse{
        PatientAppointmentResponse {
            id: patient_appointment_entity.get_id(),
            slot_id: patient_appointment_entity.get_slot_id(),
            patient_id: patient_appointment_entity.get_patient_id(),
            patient_name: String::from(patient_appointment_entity.get_patient_name()),
            reserved_at: patient_appointment_entity.get_appointment_reserve_time().to_chrono(),
            time: patient_appointment_entity.get_appointment_start_time().to_chrono(),
            is_completed: patient_appointment_entity.is_completed(),
            is_canceled: patient_appointment_entity.is_canceled(),
        }
    }
}