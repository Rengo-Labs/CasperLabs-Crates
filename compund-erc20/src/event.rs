use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::contract_api::storage;
use casper_types::{URef, U256};
use compound_casper_erc20::Address;

use crate::data::get_package_hash;

pub enum ERC20Event {
    Approval {
        owner: Address,
        spender: Address,
        value: U256,
    },
    Transfer {
        from: Address,
        to: Address,
        value: U256,
    },
}

impl ERC20Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC20Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            ERC20Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
        }
        .to_string()
    }
}

pub fn emit(erc20_event: &ERC20Event) {
    let mut events = Vec::new();
    let package = get_package_hash();
    match erc20_event {
        ERC20Event::Approval {
            owner,
            spender,
            value,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", erc20_event.type_name());
            event.insert("owner", owner.to_string());
            event.insert("spender", spender.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        ERC20Event::Transfer { from, to, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", erc20_event.type_name());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
