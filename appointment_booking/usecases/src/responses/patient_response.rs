use serde::{Serialize, Deserialize};

use infrastructure::ObjectId;
use domain::entities::PatientEntity;
#[derive(Serialize,Deserialize)]
pub struct ResponsePatient {
    pub id: ObjectId,
    pub name: String,
    pub email: String
}

impl ResponsePatient{
    pub fn from(patient_entity: PatientEntity) -> ResponsePatient{
        ResponsePatient {
            id: patient_entity.get_id(),
            name: patient_entity.get_name().to_string(),
            email: patient_entity.get_email().to_string(),
        }
    }
}