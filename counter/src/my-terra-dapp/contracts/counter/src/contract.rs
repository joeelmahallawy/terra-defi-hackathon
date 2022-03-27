#[cfg(not(feature = "library"))]
use std::collections::HashMap;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        table: Table::new(),
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

pub struct Testtype
{
    name: String,
    pass: String,
    age: u64,
}

pub struct Table<T>
{
    fields: HashMap<u64, T>,
}

impl<T> Table<T>
{
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: u64, value: T) {
        self.fields.insert(key, value);
    }
}

pub fn test(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {

        let next = Testtype {
            name: String::from("aaaa"),
            pass: String::from("aaaa"),
            age: 25
        };
        state.table.insert(state.count, next);
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
