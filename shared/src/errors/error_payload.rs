use serde::Serialize;

#[derive(Serialize,Clone,Debug)]
pub struct ErrorPayload{
    pub message: String
}