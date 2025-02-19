use bson::{oid::ObjectId};
use shared::errors::{ApplicationError,ApplicationResult};

use crate::domain::{PatientRepositoryTrait,AppointmentBookingError};
use crate::infrastructure::{MongoDataBase,MongoPatientRepository};
use crate::usecases::responses::ResponsePatient;

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
    pub async fn by_id(&self, patient_id: ObjectId) -> ApplicationResult<ResponsePatient>{
        let retrieved_patient = self.patients_repo.get_patient_by_id(patient_id)
            .await
            .map_err(|appointment_booking_error:AppointmentBookingError| {
                if let AppointmentBookingError::PatientNotFound(patient_id) = appointment_booking_error {
                    return ApplicationError::RequestNotFound(format!("Patient with requested id ({patient_id}) is not found"), Box::new(appointment_booking_error));
                };
                ApplicationError::InternalServerError(Box::new(appointment_booking_error))
            })?;
        Ok(ResponsePatient::from(retrieved_patient))
    }
}