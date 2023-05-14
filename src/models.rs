use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub error_code: u32,
    pub error_description: String,
    #[serde(rename = "token_expire_ts")]
    pub token_expire_ts: u32,
    pub server_time: u32,
    pub active_sessions: Vec<Value>,
    pub login: LoginInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub username: String,
    pub external_id: String,
    pub language: String,
    pub token: String,
    pub offline_allowed: bool,
    pub manage_others: bool,
    pub administrator: bool,
    pub view_invoice: bool,
    pub view_transaction_history: bool,
    pub view_products: bool,
    pub api_message_permission: bool,
    pub correction_allowed: bool,
    pub installer: bool,
    pub start_multiple: bool,
    pub start_for_others: bool,
    pub time_for_review: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReserveStopResponse {
    pub error_code: u32,
    pub error_description: String,
    #[serde(rename = "token_expire_ts")]
    pub token_expire_ts: u32,
    pub server_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub error_code: u32,
    pub error_description: String,
    #[serde(rename = "token_expire_ts")]
    pub token_expire_ts: u32,
    pub server_time: u32,
    pub account_id: String,
    pub currency: String,
    pub balance_cents: u32,
    pub balance_date_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentPriceObject {
    pub full_price_string: String,
    pub price_string: String,
    pub cost_cents: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PricingInfo {
    pub service_type: String,
    pub component_price_objects: Vec<ComponentPriceObject>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub error_code: usize,
    pub error_description: String,
    #[serde(rename = "token_expire_ts")]
    pub token_expire_ts: usize,
    pub server_time: usize,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MachineState {
    Available,
    Reserved,
    Occupied,
    Stoppable,
    Faulted,
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineState::Available => write!(f, "Available"),
            MachineState::Reserved => write!(f, "Reserved"),
            MachineState::Occupied => write!(f, "Occupied"),
            MachineState::Stoppable => write!(f, "Stoppable"),
            MachineState::Faulted => write!(f, "Faulted"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MachineData {
    pub external_id: String,
    pub location_id: String,
    pub location: String,
    pub location_top_level_name: String,
    pub service_type: String,
    pub service_name: String,
    pub unit: String,
    pub state: MachineState,
    pub state_description: String,
    pub last_session_start: Option<i64>,
    pub required_fields: Vec<Value>,
    pub free_form_question_int: Vec<Value>,
    pub pricing: Vec<PricingInfo>,
    pub tariff_set_name: String,
    pub gps: Value,
    pub reservable: String,
    pub reservations: Vec<Value>,
    pub block_time_seconds: u32,
    pub time_of_arrival_seconds: u32,
    pub checkout_time_seconds: u32,
    pub start_with_predetermined_usage: bool,
    pub optional_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub mutation_timestamp: u32,
    pub currency: String,
    pub mutation_cents: i32,
    pub balance_cents_before: u32,
    pub balance_cents_after: u32,
    pub mutation_description: String,
    pub service_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationTypeObject {
    #[serde(rename(deserialize = "type", serialize = "_type"))]
    pub _type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocationInfo {
    pub name: String,
    pub external_id: String,
    pub gps: HashMap<String, f64>,
    pub location_type_v2: String,
    pub location_type_object: LocationTypeObject,
    pub location_status: String,
    pub duration_required: bool,
    pub known_communication_issues: bool,
    pub services: Vec<Value>,
    pub pricing: Vec<Value>,
    pub products: Vec<Value>,
    pub child_locations: Vec<Value>,
    pub max_days_in_advance: u32,
    pub reserved_type: String,
    pub service_types: Vec<Value>,
}
