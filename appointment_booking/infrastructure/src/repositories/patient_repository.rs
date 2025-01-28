use async_trait::async_trait;
use bson::{oid::ObjectId, doc};
use mongodb::{Collection, Database};
use domain::entities::PatientEntity;
use domain::repository_traits::{PatientRepositoryResult, PatientRepositoryTrait};

use crate::data_models::PatientDataModel;
const DEFAULT_SLOTS_COLLECTION_NAME: &str = "patients";


pub struct MongoPatientRepository {
    patients_collection: Collection<PatientDataModel>
}
impl MongoPatientRepository {
    pub fn new(mongo_db: &Database) -> MongoPatientRepository {
        MongoPatientRepository::with_collection_name(mongo_db, DEFAULT_SLOTS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> MongoPatientRepository {
        let patients_collection: Collection<PatientDataModel> = mongo_db.collection(doctors_collection_name);
        MongoPatientRepository {patients_collection }
    }
}
#[async_trait]
impl PatientRepositoryTrait for MongoPatientRepository{
    async fn get_patient_by_id(&self, patient_id: ObjectId) -> PatientRepositoryResult<PatientEntity> {
        let target_patient = self
            .patients_collection
            .find_one(doc! { "_id": patient_id })
            .await?;
        Ok(target_patient.unwrap().to_domain_entity())
    }

    async fn create_patient(&mut self, new_patient: &PatientEntity) -> PatientRepositoryResult<ObjectId> {
        let new_patient_data = PatientDataModel::from(new_patient);
        let patient_create_res = self
            .patients_collection
            .insert_one(new_patient_data).await?;
        Ok(patient_create_res.inserted_id.as_object_id().unwrap())
    }
}