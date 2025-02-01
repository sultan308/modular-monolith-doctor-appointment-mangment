use anyhow::Result;
use crate::{DateTime,ObjectId};

use crate::payloads::{AddSlotPayload};
use crate::responses::{ResponseDoctorSlot};

use business::models::Slot;
use business::services::SlotsServices;
use business::MongoDataBase;


pub struct SlotsController {
    slots_services: SlotsServices
}

impl SlotsController {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> SlotsController {
        SlotsController {
            slots_services: SlotsServices::with_mongo_db(mongo_data_base)
        }

    }
    pub async fn add_to_doctor (&mut self, doctor_id: ObjectId, add_slot_payload: AddSlotPayload) ->  Result<ResponseDoctorSlot>  {
        let slot_to_add = Slot::new(doctor_id,DateTime::from(add_slot_payload.time), add_slot_payload.duration_in_min, add_slot_payload.cost_cents);
        let added_slot = self.slots_services.add_slot_for_doctor(slot_to_add).await?;
        Ok(ResponseDoctorSlot::from_slot(added_slot))
    }
    pub async fn delete_by_id(&mut self, slot_id: ObjectId, doctor_id : ObjectId) -> Result<()> {
        self.slots_services.delete_doctor_slot(slot_id, doctor_id).await?;
        Ok(())
    }
    pub async fn get_all_slots_by_doctor(&self, doctor_id : ObjectId ) -> Result<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_doctor_slots(doctor_id).await?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_all_slots_by_patient(&self, patient_id : ObjectId ) -> Result<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_patient_reserved_slots(patient_id).await?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_all_bookable_slots(&self) -> Result<Vec<ResponseDoctorSlot>> {
        let found_doctor_slots = self.slots_services.get_all_bookable_slots().await?;
        Ok(found_doctor_slots.into_iter().map(ResponseDoctorSlot::from_slot).collect())
    }
    pub async fn get_by_id(&self, slot_id: ObjectId, _doctor_id : ObjectId) -> Result<ResponseDoctorSlot> {
        let found_slot = self.slots_services.get_slot(slot_id).await?;
        Ok(ResponseDoctorSlot::from_slot(found_slot))
    }
    pub async fn reserve_slot(&mut self, slot_id: ObjectId, patient_id: ObjectId) -> Result<ResponseDoctorSlot> {
        let slot = self.slots_services.reserve_slot(slot_id, patient_id).await?;
        Ok(ResponseDoctorSlot::from_slot(slot))

    }
    pub async fn complete_slot(&mut self, slot_id: ObjectId, _doctor_id : ObjectId) -> Result<ResponseDoctorSlot> {
        let mut slot = self.slots_services.get_slot(slot_id).await?;
        slot.complete();
        self.slots_services.update_slot(&slot).await?;
        Ok(ResponseDoctorSlot::from_slot(slot))

    }
    pub async fn cancel_slot(&mut self, slot_id: ObjectId, _doctor_id : ObjectId) -> Result<ResponseDoctorSlot> {
        let mut slot = self.slots_services.get_slot(slot_id).await?;
        slot.cancel();
        self.slots_services.update_slot(&slot).await?;
        Ok(ResponseDoctorSlot::from_slot(slot))

    }
}



