mod doctors_repository_trait;
mod doctors_mongo_repository;
mod slots_mongo_repository;
mod slots_repository_trait;
mod repository_error;

pub use doctors_repository_trait::{DoctorsRepository, DoctorsRepositoryResult};
pub use slots_repository_trait::{SlotsRepository, SlotsRepositoryResult, SlotsRepositoryFilter};

pub use doctors_mongo_repository::{DoctorsMongoRepository};
pub use slots_mongo_repository::{SlotsMongoRepository};
