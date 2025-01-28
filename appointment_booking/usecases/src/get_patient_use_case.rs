use anyhow::Result;

use domain::repository_traits::PatientRepositoryTrait;
use infrastructure::{MongoDataBase, ObjectId};
use infrastructure::repositories::MongoPatientRepository;

use crate::responses::ResponsePatient;

pub struct GetPatientUseCase{
    patients_repo : Box<dyn PatientRepositoryTrait>
}

impl GetPatientUseCase {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> GetPatientUseCase {
        GetPatientUseCase{
            patients_repo: Box::new(MongoPatientRepository::new(mongo_data_base))
        }
    }
}

impl GetPatientUseCase{
    pub async fn by_id(&mut self, patient_id: ObjectId) -> Result<ResponsePatient>{
        let retrieved_patient = self.patients_repo.get_patient_by_id(patient_id).await?;
        Ok(ResponsePatient::from(retrieved_patient))
    }
}