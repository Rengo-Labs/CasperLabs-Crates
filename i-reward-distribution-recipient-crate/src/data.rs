use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key};
use casperlabs_contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_CONTRACT_PACKAGE_HASH: &str = "self_contract_package_hash";
pub const REWARDDISTRIBUTION: &str = "reward_distribution";
pub const RESULT: &str = "result";

pub fn set_reward_distribution(reward_distribution: Key) {
    set_key(REWARDDISTRIBUTION, reward_distribution);
}
pub fn get_reward_distribution() -> Key {
    get_key(REWARDDISTRIBUTION).unwrap_or_revert()
}
pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}
pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
