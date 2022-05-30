use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub errorCode: u32,
    pub errorDescription: String,
    pub token_expire_ts: u32,
    pub serverTime: u32,
    pub activeSessions: Vec<Value>,
    pub login: LoginInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo {
    pub email: String,
    pub username: String,
    pub externalId: String,
    pub language: String,
    pub token: String,
    pub offlineAllowed: bool,
    pub manageOthers: bool,
    pub administrator: bool,
    pub viewInvoice: bool,
    pub viewTransactionHistory: bool,
    pub viewProducts: bool,
    pub apiMessagePermission: bool,
    pub correctionAllowed: bool,
    pub installer: bool,
    pub startMultiple: bool,
    pub startForOthers: bool,
    pub timeForReview: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopStartResponse {
    pub errorCode: u32,
    pub errorDescription: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    pub errorCode: u32,
    pub errorDescription: String,
    pub token_expire_ts: u32,
    pub serverTime: u32,
    pub accountId: String,
    pub currency: String,
    pub balanceCents: u32,
    pub balanceDateTime: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentPriceObject {
    pub fullPriceString: String,
    pub priceString: String,
    pub costCents: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricingInfo {
    pub serviceType: String,
    pub componentPriceObjects: Vec<ComponentPriceObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machines {
    pub errorCode: u32,
    pub errorDescription: String,
    pub token_expire_ts: u32,
    pub serverTime: u32,
    pub data: Vec<MachineData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineData {
    pub externalId: String,
    pub locationId: String,
    pub location: String,
    pub locationTopLevelName: String,
    pub serviceType: String,
    pub serviceName: String,
    pub unit: String,
    pub state: String,
    pub stateDescription: String,
    pub lastSessionStart: Option<i64>,
    pub requiredFields: Vec<Value>,
    pub freeFormQuestionInt: Vec<Value>,
    pub pricing: Vec<PricingInfo>,
    pub tariffSetName: String,
    pub gps: Value,
    pub reservable: String,
    pub reservations: Vec<Value>,
    pub blockTimeSeconds: u32,
    pub timeOfArrivalSeconds: u32,
    pub checkoutTimeSeconds: u32,
    pub startWithPredeterminedUsage: bool,
    pub optionalName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseHistory {
    pub mutationTimestamp: u32,
    pub currency: String,
    pub mutationCents: i32,
    pub balanceCentsBefore: u32,
    pub balanceCentsAfter: u32,
    pub mutationDescription: String,
    pub serviceType: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryResponse {
    pub errorCode: u32,
    pub errorDescription: String,
    pub token_expire_ts: u32,
    pub serverTime: u32,
    pub data: Vec<PurchaseHistory>,
}
