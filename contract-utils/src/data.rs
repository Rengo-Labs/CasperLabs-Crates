use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, Key, RuntimeArgs, URef, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use core::convert::TryInto;

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct Dict {
    uref: URef,
}

impl Dict {
    pub fn instance(name: &str) -> Dict {
        let key = runtime::get_key(name).unwrap_or_revert();
        let uref = *key.as_uref().unwrap_or_revert();
        Dict { uref }
    }

    pub fn init(name: &str) {
        storage::new_dictionary(name).unwrap_or_revert();
    }

    pub fn at(uref: URef) -> Dict {
        Dict { uref }
    }

    pub fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T> {
        storage::dictionary_get(self.uref, key)
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    pub fn get_by_key<T: CLTyped + FromBytes>(&self, key: &Key) -> Option<T> {
        self.get(&key_to_str(key))
    }

    pub fn get_by_keys<T: CLTyped + FromBytes, U: CLTyped + ToBytes>(
        &self,
        keys: (&U, &Key),
    ) -> Option<T> {
        self.get(&keys_to_str(keys.0, keys.1))
    }

    pub fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T) {
        storage::dictionary_put(self.uref, key, Some(value));
    }

    pub fn set_by_key<T: CLTyped + ToBytes>(&self, key: &Key, value: T) {
        self.set(&key_to_str(key), value);
    }

    pub fn set_by_keys<T: CLTyped + ToBytes, U: CLTyped + ToBytes>(
        &self,
        keys: (&U, &Key),
        value: T,
    ) {
        self.set(&keys_to_str(keys.0, keys.1), value)
    }
    pub fn set_by_values<T: CLTyped + ToBytes, U: CLTyped + ToBytes, V: CLTyped + ToBytes>(
        &self,
        keys: (&T, &U),
        value: V,
    ) {
        self.set(&values_to_str(keys.0, keys.1), value);
    }

    pub fn get_by_values<T: CLTyped + ToBytes, U: CLTyped + ToBytes, R: CLTyped + FromBytes>(
        &self,
        keys: (&T, &U),
    ) -> Option<R> {
        self.get(&values_to_str(keys.0, keys.1))
    }

    pub fn remove<T: CLTyped + ToBytes>(&self, key: &str) {
        storage::dictionary_put(self.uref, key, Option::<T>::None);
    }

    pub fn remove_by_key<T: CLTyped + ToBytes>(&self, key: &Key) {
        self.remove::<T>(&key_to_str(key));
    }

    pub fn remove_by_vec_of_keys<T: CLTyped + ToBytes>(&self, keys: (&Key, &Key)) {
        self.remove::<T>(&keys_to_str(keys.0, keys.1))
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    }
}

pub fn keys_to_str<U: CLTyped + ToBytes, V: CLTyped + ToBytes>(key_a: &U, key_b: &V) -> String {
    let mut bytes_a = key_a.to_bytes().unwrap_or_revert();
    let mut bytes_b = key_b.to_bytes().unwrap_or_revert();
    bytes_a.append(&mut bytes_b);
    let bytes = runtime::blake2b(bytes_a);
    hex::encode(bytes)
}

pub fn values_to_str<T: CLTyped + ToBytes, U: CLTyped + ToBytes>(
    value_a: &T,
    value_b: &U,
) -> String {
    let mut bytes_a = value_a.to_bytes().unwrap_or_revert();
    let mut bytes_b = value_b.to_bytes().unwrap_or_revert();

    bytes_a.append(&mut bytes_b);

    let bytes = runtime::blake2b(bytes_a);
    hex::encode(bytes)
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut bytes_a = key.to_bytes().unwrap_or_revert();
    let mut bytes_b = value.to_bytes().unwrap_or_revert();

    bytes_a.append(&mut bytes_b);

    let bytes = runtime::blake2b(bytes_a);
    hex::encode(bytes)
}

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn call_function(target: Key, function_name: String, function_args: RuntimeArgs) -> String {
    if function_name == "get_gauge_weight"
        || function_name == "gauge_relative_weight"
        || function_name == "inflation_rate"
        || function_name == "working_supply"
        || function_name == "balance_of"
        || function_name == "duration"
        || function_name == "reward_rate"
        || function_name == "balances"
        || function_name == "total_supply"
        || function_name == "get_balance"
        || function_name == "earned"
        || function_name == "allowance"
        || function_name == "locked_end"
        || function_name == "vested_of"
        || function_name == "locked_of"
        || function_name == "initial_locked"
        || function_name == "start_time"
        || function_name == "end_time"
        || function_name == "total_claimed"
        || function_name == "working_balances"
        || function_name == "vested_supply"
        || function_name == "user_reward_per_token_paid"
    {
        let ret: U256 = runtime::call_versioned_contract(
            target.into_hash().unwrap_or_revert().into(),
            None,
            &function_name,
            function_args,
        );
        let data: String = ret.to_string() + ":U256";
        return data;
    }
    if function_name == "gauges" || function_name == "lp_token" {
        let ret: Key = runtime::call_versioned_contract(
            target.into_hash().unwrap_or_revert().into(),
            None,
            &function_name,
            function_args,
        );
        let data: String = ret.to_string() + ":Key";
        return data;
    }
    if function_name == "gauge_type_names" {
        let ret: String = runtime::call_versioned_contract(
            target.into_hash().unwrap_or_revert().into(),
            None,
            &function_name,
            function_args,
        );
        let data: String = ret.to_string() + ":String";
        return data;
    }
    if function_name == "gauge_types"
        || function_name == "n_gauge_types"
        || function_name == "n_gauges"
    {
        let ret: (bool, U128) = runtime::call_versioned_contract(
            target.into_hash().unwrap_or_revert().into(),
            None,
            &function_name,
            function_args,
        );
        let data: String =
            "(".to_string() + &ret.0.to_string() + "," + &ret.1.to_string() + ")" + ":(bool,U128)";
        return data;
    }
    "".to_string()
}
