use bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PatientMongoDocument {
    pub _id: ObjectId,
    pub name: String,
    pub email: String
}