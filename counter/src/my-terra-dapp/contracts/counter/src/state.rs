use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
 
use super::contract;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {

    /* 
     * the developer can change Testtype to be whatever they want the type of 
     * the table's columns to be
     */
    // pub table: contract::DatabaseMod::Table<contract::DatabaseMod::Testtype>,
    pub table: contract::Table<contract::Testtype>,
    pub count: u64,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
