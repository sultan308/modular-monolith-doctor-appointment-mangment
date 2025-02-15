use anyhow::Result;
use bson::{DateTime, oid::ObjectId};
use mongodb::Database;
use shared::errors::{ApplicationError,ApplicationResult};


use crate::application::payloads::{AddSlotPayload};
use crate::application::responses::{ResponseDoctorSlot};

use crate::business::{Slot,SlotsServices, DoctorAvailabilityError};



pub struct SlotsController {
    slots_services: SlotsServices
}

impl SlotsController {
    pub fn with_mongo_db(mongo_data_base: &Database) -> SlotsController {
        SlotsController {
            slots_services: SlotsServices::with_mongo_db(mongo_data_base)
        }

    }
    pub async fn add_to_doctor (&mut self, doctor_id: ObjectId, add_slot_payload: AddSlotPayload) ->  ApplicationResult<ResponseDoctorSlot>  {
        let slot_to_add = Slot::new(doctor_id,DateTime::from(add_slot_payload.time), add_slot_payload.duration_in_min, add_slot_payload.cost_cents);
        let added_slot = self.slots_services.add_slot_for_doctor(slot_to_add)
            .await
            .map_err(|application_error| ApplicationError::InternalServerError(Box::new(application_error)))?;
        Ok(ResponseDoctorSlot::from_slot(added_slot))
    }
    pub async fn delete_by_id(&mut self, slot_id: ObjectId, doctor_id : ObjectId) -> ApplicationResult<()> {
        self.slots_services.delete_doctor_slot(slot_id, doctor_id)
            .await
            .map_err(|application_error| ApplicationError::InternalServerError(Box::new(application_error)))?;
        Ok(())
    }
    pub async fn get_all_slots_by_doctor(&self, doctor_id : ObjectId ) -> ApplicationResult<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_doctor_slots(doctor_id)
            .await
            .map_err(|application_error| ApplicationError::InternalServerError(Box::new(application_error)))?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_all_slots_by_patient(&self, patient_id : ObjectId ) -> ApplicationResult<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_patient_reserved_slots(patient_id)
            .await
            .map_err(|application_error| ApplicationError::InternalServerError(Box::new(application_error)))?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_all_bookable_slots(&self) -> ApplicationResult<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_bookable_slots()
            .await
            .map_err(|application_error| ApplicationError::InternalServerError(Box::new(application_error)))?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_by_id(&self, slot_id: ObjectId, _doctor_id : ObjectId) -> ApplicationResult<ResponseDoctorSlot> {
        let found_slot = self.slots_services.get_slot(slot_id)
            .await
            .map_err(|doctor_availability_error| {
                if let DoctorAvailabilityError::SlotNotFound(not_found_slot_id) = doctor_availability_error {
                    return ApplicationError::RequestNotFound(format!("No slot found with the requested id ({not_found_slot_id})"),Box::new(doctor_availability_error))
                };
                ApplicationError::InternalServerError(Box::new(doctor_availability_error))
            })?;
        Ok(ResponseDoctorSlot::from_slot(found_slot))
    }
    pub async fn reserve_slot(&mut self, slot_id: ObjectId, patient_id: ObjectId) -> ApplicationResult<ResponseDoctorSlot> {
        let slot = self.slots_services.reserve_slot(slot_id, patient_id)
            .await
            .map_err(|doctor_availability_error| {
                if let DoctorAvailabilityError::FailedToReserveSlot(slot, _) = &doctor_availability_error {
                    return ApplicationError::InvalidOperation(format!("Failed to reserve slot ({}) as it's already reserved!", slot.get_id()),Box::new(doctor_availability_error))
                };
                ApplicationError::InternalServerError(Box::new(doctor_availability_error))
            })?;
        Ok(ResponseDoctorSlot::from_slot(slot))

    }

}



