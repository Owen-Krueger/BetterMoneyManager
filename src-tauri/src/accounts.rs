use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub bank_name: String,
    pub account_type: i32,
    pub balance: f64,
    pub available_balance: f64,
    pub favorite: bool,
    pub date_created: String,
}