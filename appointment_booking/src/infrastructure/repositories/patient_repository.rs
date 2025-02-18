use async_trait::async_trait;
use bson::{oid::ObjectId, doc};
use mongodb::{Collection, Database};

use crate::domain::{AppointmentBookingError, AppointmentBookingResult,
                    PatientEntity, PatientRepositoryTrait};
use crate::infrastructure::data_models::PatientDataModel;
const DEFAULT_PATIENTS_COLLECTION_NAME: &str = "patients";

pub struct MongoPatientRepository {
    patients_collection: Collection<PatientDataModel>
}
impl MongoPatientRepository {
    pub fn new(mongo_db: &Database) -> MongoPatientRepository {
        MongoPatientRepository::with_collection_name(mongo_db, DEFAULT_PATIENTS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> MongoPatientRepository {
        let patients_collection: Collection<PatientDataModel> = mongo_db.collection(doctors_collection_name);
        MongoPatientRepository {patients_collection }
    }
}
#[async_trait]
impl PatientRepositoryTrait for MongoPatientRepository{
    async fn get_patient_by_id(&self, patient_id: ObjectId) -> AppointmentBookingResult<PatientEntity> {
        let target_patient = self
            .patients_collection
            .find_one(doc! { "_id": patient_id })
            .await
            .map_err(|mongo_err| AppointmentBookingError::InternalBookingError(Box::new(mongo_err)))?;
        let patient_model = target_patient.ok_or(AppointmentBookingError::PatientNotFound(patient_id))?;
        Ok(patient_model.to_domain_entity())
    }

    async fn create_patient(&mut self, new_patient: &PatientEntity) -> AppointmentBookingResult<ObjectId> {
        let new_patient_data = PatientDataModel::from(new_patient);
        self.patients_collection
            .insert_one(new_patient_data)
            .await
            .map_err(|mongo_err| AppointmentBookingError::InternalBookingError(Box::new(mongo_err)))?;
        Ok(new_patient.get_id())
    }
}