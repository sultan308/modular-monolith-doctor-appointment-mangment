use bson::oid::ObjectId;
use shared::errors::{ApplicationError, ApplicationResult};

use crate::infrastructure::MongoDataBase;
use crate::usecases::{CreatePatientUseCase,
                      CreatePatientUseCaseInterface, GetPatientUseCase,
                      ResponsePatient};


use crate::presentation::payloads::patient_payloads::CreatePatientPayload;

pub struct PatientController {
    get_patient_use_case: GetPatientUseCase,
    create_patient_use_case: CreatePatientUseCase
}
impl PatientController {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> PatientController {
        PatientController {
            get_patient_use_case: GetPatientUseCase::with_mongo_db(mongo_data_base),
            create_patient_use_case: CreatePatientUseCase::with_mongo_db(mongo_data_base),
        }
    }
}
impl PatientController {
    pub async fn create_patient (&mut self, create_patient_payload: CreatePatientPayload) ->  ApplicationResult<ResponsePatient>  {
        let create_patient_interface = CreatePatientUseCaseInterface{
            name:create_patient_payload.name,
            email:create_patient_payload.email
        };
        let created_patient = self.create_patient_use_case.invoke(create_patient_interface).await?;
        Ok(created_patient)
    }
    pub async fn get_patient (&mut self, patient_id: ObjectId) ->  ApplicationResult<ResponsePatient>  {
        let found_patient = self.get_patient_use_case.by_id(patient_id).await?;
        Ok(found_patient)
    }

}