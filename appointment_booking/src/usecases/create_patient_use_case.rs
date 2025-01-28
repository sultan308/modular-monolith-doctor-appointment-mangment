use anyhow::Result;

use crate::domain::{PatientEntity,PatientRepositoryTrait};
use crate::infrastructure::{MongoDataBase,MongoPatientRepository};
use crate::usecases::responses::ResponsePatient;
pub struct CreatePatientUseCaseInterface {
    pub name: String,
    pub email: String
}

pub struct CreatePatientUseCase{
    patients_repo : Box<dyn PatientRepositoryTrait>
}

impl CreatePatientUseCase {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> CreatePatientUseCase {
        CreatePatientUseCase{
            patients_repo: Box::new(MongoPatientRepository::new(mongo_data_base))
        }
    }
}

impl CreatePatientUseCase{
    pub async fn invoke(&mut self, create_patient_use_case_interface: CreatePatientUseCaseInterface) -> Result<ResponsePatient>{
        let patient_entity = PatientEntity::new(
            &create_patient_use_case_interface.name,
            &create_patient_use_case_interface.email
        );
        self.patients_repo.create_patient(&patient_entity).await?;
        Ok(ResponsePatient::from(patient_entity))
    }
}