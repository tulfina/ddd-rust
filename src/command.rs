use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct Command<Payload> {
    pub id: String,
    pub _type: String,
    pub aggregate_id: String,
    pub payload: Payload,
    pub date_time: Option<DateTime<Utc>>,
    pub aggregate_type: Option<String>,
    pub status: Option<String>,
    pub additional_information: Option<Value>,
    pub headers: Option<Value>,
    pub correlation_id: Option<String>,
}

impl Command<Value> {
    pub fn new(id: String, _type: String, aggregate_id: String, payload: Value) -> Self {
        Self {
            id,
            _type,
            aggregate_id,
            payload,
            date_time: Some(Utc::now()),
            aggregate_type: None,
            status: None,
            additional_information: None,
            headers: None,
            correlation_id: None,
        }
    }
}
