use anyhow::Result;
use async_trait::async_trait;
use bson::{doc, Document, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::core::{Appointment, AppointmentsFilter,
                  AppointmentsRepositoryResult, AppointmentsRepositoryTrait};

mod slot_mongo_document;
use slot_mongo_document::SlotMongoDocument;
mod patient_mongo_document;
use patient_mongo_document::PatientMongoDocument;
use shared::types::ContactData;

const DEFAULT_SLOTS_COLLECTION_NAME: &str = "slots";
const DEFAULT_PATIENTS_COLLECTION_NAME: &str = "patients";


pub struct MongoAppointmentsRepository {
    slots_collection: Collection<SlotMongoDocument>,
    patients_collection: Collection<PatientMongoDocument>,
}

impl MongoAppointmentsRepository {
    pub fn new(mongo_db: &Database) -> MongoAppointmentsRepository {
        MongoAppointmentsRepository::with_collection_name(mongo_db,
                                                          DEFAULT_SLOTS_COLLECTION_NAME,
                                                          DEFAULT_PATIENTS_COLLECTION_NAME )
    }
    pub fn with_collection_name(mongo_db: &Database, slots_collection_name: &str, patients_collection_name: &str) -> MongoAppointmentsRepository {
        let slots_collection: Collection<SlotMongoDocument> = mongo_db.collection(slots_collection_name);
        let patients_collection: Collection<PatientMongoDocument> = mongo_db.collection(patients_collection_name);
        MongoAppointmentsRepository { slots_collection, patients_collection }
    }
}

// Private methods
impl MongoAppointmentsRepository {
    fn appointment_factory (slot: SlotMongoDocument, patient: &PatientMongoDocument) -> Appointment {
        let patient_contacts = ContactData::build(&patient.name,&patient.email);
        match (slot.canceled_at, slot.completed_at) {
            (None, None) => Appointment::build(slot._id,patient_contacts, slot.time),
            (None, Some(completed_at)) => Appointment::build_canceled_appointment(slot._id,patient_contacts, slot.time,completed_at),
            (Some(canceled_at), None) => Appointment::build_completed_appointment(slot._id,patient_contacts, slot.time,canceled_at),
            _ => panic!("Trying to load invalid appointment data")
        }
    }

    fn appointments_filter_to_mongo_filter(appointments_filter: AppointmentsFilter) -> Document {
        match appointments_filter {
            AppointmentsFilter { doctor_id, from: Some(min), to: Some(max) } => {
                doc! {
                    "reserving_patient_id": {"$exists": true},
                    "doctor_id" : doctor_id, "time": {"$gte": min, "$lte": max}}
            },
            AppointmentsFilter { doctor_id, from: Some(min), to: None } => {
                doc! {
                    "reserving_patient_id": {"$exists": true},
                    "doctor_id": doctor_id,"time": {"$gte": min}}
            },
            AppointmentsFilter { doctor_id, from: None, to: Some(max) } => {
                doc! {
                    "reserving_patient_id": {"$exists": true},
                    "doctor_id": doctor_id, "time": {"$lte": max}}
            },
            AppointmentsFilter { doctor_id, from: None, to: None } => {
                doc! {"reserving_patient_id": {"$exists": true}, "doctor_id": doctor_id}
            },
        }
    }

    async fn get_patient(&self, patient_id: ObjectId) -> Result<PatientMongoDocument>{
        let patient_document = self
            .patients_collection
            .find_one(doc! { "_id": patient_id })
            .await?;
        if patient_document.is_none(){ panic!("Couldn't find appointment patient")}
        Ok(patient_document.unwrap())
    }
}
#[async_trait]
impl AppointmentsRepositoryTrait for MongoAppointmentsRepository{
    async fn get_appointment(&self, appointment_id: ObjectId) -> AppointmentsRepositoryResult<Appointment> {
        let slot_document = self
            .slots_collection
            .find_one(doc! { "_id": appointment_id, "reserving_patient_id": {"$exists": true}})
            .await?.unwrap();
        let patient_document = self.get_patient(slot_document.reserving_patient_id).await?;
        let appointment = MongoAppointmentsRepository::appointment_factory(slot_document, &patient_document);
        Ok(appointment)

    }

    async fn get_doctor_appointments(&self, filter: AppointmentsFilter) -> AppointmentsRepositoryResult<Vec<Appointment>> {
        let mut patients: std::collections::HashMap<ObjectId,PatientMongoDocument> =  std::collections::HashMap::new();
        let mongo_slots_filter = MongoAppointmentsRepository::appointments_filter_to_mongo_filter(filter);
        let cursor = self.slots_collection.find(mongo_slots_filter).await?;
        let slot_documents: Vec<SlotMongoDocument> = cursor.try_collect().await?;
        let mut appointments: Vec<Appointment> = Vec::new();

        for slot_document in slot_documents {
            let slot_id = slot_document._id;
            match patients.get(&slot_id) {
                Some(patient_mongo_document) => {
                    appointments.push( MongoAppointmentsRepository::appointment_factory(slot_document, &patient_mongo_document));
                }
                None => {
                    let patient_document = self.get_patient(slot_document.reserving_patient_id).await?;
                    appointments.push( MongoAppointmentsRepository::appointment_factory(slot_document, &patient_document));
                    patients.insert(slot_id, patient_document);
                }
            };
        }

        Ok(appointments)
    }

    async fn save_appointment_status(&self, appointment: &Appointment) -> AppointmentsRepositoryResult<()> {
        let update_doc = match (appointment.completed_at(),appointment.canceled_at()) {
            (Some(_), Some(_)) => panic!("Trying to save invalid appointment data"),
            (Some(completed_at), None) => doc! { "completed_at": completed_at, "canceled_at": bson::Bson::Null },
            (None, Some(canceled_at)) => doc! { "canceled_at": canceled_at,  "completed_at": bson::Bson::Null},
            (None,None) => doc! { "canceled_at": bson::Bson::Null,  "completed_at": bson::Bson::Null},
        };

        let filter = doc! { "_id": appointment.get_id(), "reserving_patient_id": {"$exists": true}};
        let update = doc! { "$set": update_doc};
        self.slots_collection.update_one(filter, update).await?;
        Ok(())

    }
}

