use chrono::{DateTime, Utc};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddSlotPayload {
    pub time: DateTime<Utc>,
    pub duration_in_min: u16,
    pub cost_cents: usize,
}

#[derive(Deserialize)]
pub struct RescheduleSlotToPayload {
    pub time: DateTime<Utc>
}

