use bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::domain::PatientEntity;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PatientDataModel {
    pub _id: ObjectId,
    pub name: String,
    pub email: String
}
impl PatientDataModel{
    pub fn from(patient_entity: &PatientEntity) -> PatientDataModel {
        PatientDataModel {
            _id: patient_entity.get_id(),
            name: String::from(patient_entity.get_name()),
            email: String::from(patient_entity.get_email())
        }
    }
}

impl PatientDataModel{
    pub fn to_domain_entity(self) -> PatientEntity {
        PatientEntity::build(
           self._id,
           &self.name,
           &self.email
        )
    }
}

impl PartialEq for PatientDataModel {
    fn eq(&self, other: &PatientDataModel) -> bool {
        self._id == other._id &&
            self.name == other.name &&
            self.email == other.email
    }
}