use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::business::Doctor;
#[derive(Serialize,Deserialize)]
pub struct ResponseDoctor {
    id: ObjectId,
    name: String,
    email: String
}

impl ResponseDoctor {
    pub fn from_doctor(doctor: Doctor) -> ResponseDoctor{
        ResponseDoctor {
            id: doctor.get_id(),
            name: doctor.get_full_name(),
            email: doctor.get_email()
        }
    }

}

