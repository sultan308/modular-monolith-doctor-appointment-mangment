use bson::{DateTime, oid::ObjectId};
use shared::types::ContactData;

enum AppointmentStatus {
    Booked,
    Completed(bson::DateTime),
    Canceled(bson::DateTime)
}

pub struct Appointment {
    id: ObjectId,
    patient_contact_details: ContactData,
    status: AppointmentStatus,
    time: bson::DateTime,
}
impl Appointment {
    pub fn build( id: ObjectId,
                  patient_contact_details: ContactData,
                  time: bson::DateTime ) -> Appointment {
        Appointment {
            id,
            patient_contact_details,
            time,
            status: AppointmentStatus::Booked
        }

    }
    pub fn build_canceled_appointment( id: ObjectId,
                                       patient_contact_details: ContactData,
                                       time: bson::DateTime,
                                       canceled_at: bson::DateTime ) -> Appointment {
        Appointment {
            id,
            patient_contact_details,
            time,
            status: AppointmentStatus::Canceled(canceled_at)
        }

    }
    pub fn build_completed_appointment( id: ObjectId,
                                        patient_contact_details: ContactData,
                                        time: bson::DateTime,
                                        completed_at: bson::DateTime) -> Appointment {
        Appointment {
            id,
            patient_contact_details,
            time,
            status: AppointmentStatus::Completed(completed_at)
        }

    }
}
impl Appointment {
    pub fn get_id(&self) -> ObjectId {self.id}
    pub fn get_patient_name(&self) -> &str {self.patient_contact_details.get_name()}
    pub fn get_patient_email(&self) -> &str{self.patient_contact_details.get_email()}
    pub fn get_appointment_time(&self) -> DateTime {self.time}
    pub fn completed_at(&self) -> Option<bson::DateTime> {
        match self.status {
            AppointmentStatus::Completed(at) => Some(at),
            _ => None
        }
    }
    pub fn canceled_at(&self) -> Option<bson::DateTime> {
        match self.status {
            AppointmentStatus::Canceled(at) => Some(at),
            _ => None
        }
    }
}

impl Appointment {
    pub fn complete(&mut self) {
        if self.canceled_at().is_some() { panic!("Cannot complete a canceled appointment"); };
        if self.completed_at().is_some() { panic!("Appointment already completed")};
        self.status = AppointmentStatus::Completed(bson::DateTime::now());
    }

    pub fn canceled(&mut self) {
        if self.completed_at().is_some(){ panic!("Cannot cancel a completed appointment"); };
        if self.canceled_at().is_some(){ panic!("Appointment already canceled")};
        self.status = AppointmentStatus::Canceled(bson::DateTime::now());
    }
}