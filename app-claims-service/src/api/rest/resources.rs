use serde::{Deserialize, Serialize};
use crate::model::Party;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClaim {
    pub involved: Party,
}