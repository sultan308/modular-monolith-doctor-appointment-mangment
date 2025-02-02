use bson::oid::ObjectId;
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
                                       canceled_at: bson::DateTime,
                                       time: bson::DateTime ) -> Appointment {
        Appointment {
            id,
            patient_contact_details,
            time,
            status: AppointmentStatus::Canceled(canceled_at)
        }

    }
    pub fn build_completed_appointment( id: ObjectId,
                                        patient_contact_details: ContactData,
                                        completed_at: bson::DateTime,
                                        time: bson::DateTime ) -> Appointment {
        Appointment {
            id,
            patient_contact_details,
            time,
            status: AppointmentStatus::Completed(completed_at)
        }

    }
}
impl Appointment {
    pub fn is_completed(&self) -> bool{
        match self.status {
            AppointmentStatus::Completed(_) => true,
            _ => false
        }
    }

    pub fn is_canceled(&self) -> bool{
        match self.status {
            AppointmentStatus::Canceled(_) => true,
            _ => false
        }
    }
}

impl Appointment {
    pub fn complete(&mut self) {
        if self.is_canceled() { panic!("Cannot complete a canceled appointment"); };
        if self.is_completed() { panic!("Appointment already completed")};
        self.status = AppointmentStatus::Completed(bson::DateTime::now());
    }

    pub fn canceled(&mut self) {
        if self.is_completed() { panic!("Cannot cancel a completed appointment"); };
        if self.is_canceled() { panic!("Appointment already canceled")};
        self.status = AppointmentStatus::Canceled(bson::DateTime::now());
    }
}