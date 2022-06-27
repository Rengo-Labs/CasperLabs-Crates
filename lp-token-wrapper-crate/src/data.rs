use casper_types::{bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, set_key, Dict};


pub const UNI: &str = "uni";
pub const BALANCES: &str = "balances";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_CONTRACT_PACKAGE_HASH: &str = "self_contract_package_hash";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const RESULT: &str = "result";

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES),
        }
    }

    pub fn init() {
        Dict::init(BALANCES)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value)
    }
}
pub fn set_uni(uni: Key) {
    set_key(UNI, uni);
}
pub fn get_uni() -> Key {
    get_key(UNI).unwrap_or(zero_address())
}
pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}
pub fn get_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}
pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
