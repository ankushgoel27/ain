use std::sync::Arc;

use ain_db::LedgerColumn;
use ain_macros::Repository;

use super::RepositoryOps;
use crate::{
    model::{OracleHistory, OracleHistoryId},
    storage::{columns, ocean_store::OceanStore},
    Result,
};

#[derive(Repository)]
#[repository(K = "OracleHistoryId", V = "OracleHistory")]
pub struct OracleHistoryRepository {
    pub store: Arc<OceanStore>,
    col: LedgerColumn<columns::OracleHistory>,
}

impl OracleHistoryRepository {
    pub fn new(store: Arc<OceanStore>) -> Self {
        Self {
            col: store.column(),
            store,
        }
    }
}
