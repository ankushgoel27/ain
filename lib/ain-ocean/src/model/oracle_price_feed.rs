use bitcoin::Txid;
use serde::{Deserialize, Serialize};

use super::BlockContext;

pub type OracleId = (String, String, String, Txid);
pub type OracleKey = (String, String, String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OraclePriceFeed {
    pub id: OracleId,
    pub key: OracleKey,
    pub sort: String,
    pub token: String,
    pub currency: String,
    pub oracle_id: String,
    pub txid: Txid,
    pub time: u64,
    pub amount: i64,
    pub block: BlockContext,
}
