use bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::domain::DoctorEntity;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DoctorDataModel {
    pub _id: ObjectId,
    pub name: String,
    pub email: String
}
impl DoctorDataModel{
    pub fn to_domain_entity(self) -> DoctorEntity {
        DoctorEntity::build(
           self._id,
           &self.name,
           &self.email
        )
    }
}

impl PartialEq for DoctorDataModel {
    fn eq(&self, other: &DoctorDataModel) -> bool {
        self._id == other._id &&
            self.name == other.name &&
            self.email == other.email
    }
}