use serde::{Serialize, Deserialize};
use domain::models::Doctor;
use crate::ObjectId;

#[derive(Serialize,Deserialize)]
pub struct ResponseDoctor {
    id: ObjectId,
    name: String
}

impl ResponseDoctor {
    pub fn from_doctor(doctor: Doctor) -> ResponseDoctor{
        ResponseDoctor {
            id: doctor.get_id(),
            name: doctor.get_full_name(),
        }
    }

}

