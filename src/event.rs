use crate::command::Command;
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct Event<Payload> {
    pub type_: String,
    pub aggregate_id: String,
    pub correlation_id: Option<String>,
    pub payload: Payload,
    pub id: Option<String>,
    pub date_time: Option<String>,
    pub headers: Option<Value>,
    pub additional_information: Option<Value>,
}

impl Event<Value> {
    pub fn new(id: String, type_: String, command: Command<Value>, payload: Value) -> Self {
        let date_time = command.date_time.unwrap();
        let aggregate_id = command.aggregate_id;
        let correlation_id = command.correlation_id;
        let headers = command.headers;
        let additional_information = command.additional_information;

        Self {
            id: Some(id),
            type_,
            aggregate_id,
            correlation_id,
            payload,
            date_time: Some(date_time.to_rfc3339()),
            headers,
            additional_information,
        }
    }
}