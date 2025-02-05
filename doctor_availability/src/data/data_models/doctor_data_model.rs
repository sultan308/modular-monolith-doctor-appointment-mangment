use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::bson;



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DoctorDataModel {
    pub _id: ObjectId,
    pub name: String,
    pub email: String
}

impl PartialEq for DoctorDataModel {
    fn eq(&self, other: &DoctorDataModel) -> bool {
        self._id == other._id &&
        self.name == other.name &&
        self.email == other.email
    }
}
