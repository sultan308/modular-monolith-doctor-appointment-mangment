mod doctors_repository_trait;
mod doctors_mongo_repository;
mod slots_mongo_repository;
mod slots_repository_trait;
mod repository_error;

pub use doctors_repository_trait::{DoctorsRepository};
pub use slots_repository_trait::{SlotsRepository, SlotsRepositoryFilter};

pub use doctors_mongo_repository::DoctorsMongoRepository;
pub use slots_mongo_repository::SlotsMongoRepository;

pub use repository_error::{RepositoryError,RepositoryResult};