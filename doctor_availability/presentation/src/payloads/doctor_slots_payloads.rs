use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct AddSlotPayload {
    pub time: DateTime<Utc>, // The type should include a timezone (like Utc)
    pub duration_in_min: u16,
    pub cost_cents: usize,
}

#[derive(Deserialize)]
pub struct RescheduleSlotToPayload {
    pub time: DateTime<Utc>
}

