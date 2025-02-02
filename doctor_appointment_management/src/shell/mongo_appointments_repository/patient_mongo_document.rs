use bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PatientMongoDocument {
    pub _id: ObjectId,
    pub reserving_patient_id: ObjectId,
    pub canceled_at: Option<bson::DateTime>,
    pub completed_at: Option<bson::DateTime>
}