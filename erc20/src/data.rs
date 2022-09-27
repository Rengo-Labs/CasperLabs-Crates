use alloc::string::String;
use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};

const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";

pub const NAME: &str = "name";
// pub const META: &str = "meta";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const DOMAIN_SEPARATOR: &str = "domain_separator";
pub const PERMIT_TYPE_HASH: &str = "permit_type_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct Nonces {
    dict: Dict,
}

impl Nonces {
    pub fn instance() -> Nonces {
        Nonces {
            dict: Dict::instance(NONCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(NONCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}
