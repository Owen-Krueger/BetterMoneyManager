use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub payee: String,
    pub amount: f64,
    pub date: String,
    pub number: Option<i32>,
    pub category: Option<String>,
    pub memo: Option<String>,
}