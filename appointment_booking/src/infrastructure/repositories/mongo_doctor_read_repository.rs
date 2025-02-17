use async_trait::async_trait;
use bson::{oid::ObjectId, doc};
use mongodb::{Collection, Database};
use crate::domain::{AppointmentBookingResult, AppointmentBookingError,
                    DoctorEntity, DoctorReadRepositoryTrait};

use crate::infrastructure::data_models::DoctorDataModel;
const DEFAULT_DOCTORS_COLLECTION_NAME: &str = "doctors";

pub struct MongoDoctorReadRepository {
    doctors_collection: Collection<DoctorDataModel>
}
impl MongoDoctorReadRepository {
    pub fn new(mongo_db: &Database) -> MongoDoctorReadRepository {
        MongoDoctorReadRepository::with_collection_name(mongo_db, DEFAULT_DOCTORS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> MongoDoctorReadRepository {
        let doctors_collection: Collection<DoctorDataModel> = mongo_db.collection(doctors_collection_name);
        MongoDoctorReadRepository {doctors_collection }
    }
}
#[async_trait]
impl DoctorReadRepositoryTrait for MongoDoctorReadRepository{
    async fn load(&self, doctor_id: ObjectId) -> AppointmentBookingResult<DoctorEntity> {
        let loaded_doctor = self
            .doctors_collection
            .find_one(doc! { "_id": doctor_id })
            .await
            .map_err(|mongo_err| AppointmentBookingError::InternalBookingError(Box::new(mongo_err)))?;

        let doctor_model = loaded_doctor.ok_or(AppointmentBookingError::DoctorNotFound(doctor_id))?;
        Ok(doctor_model.to_domain_entity())
    }
}