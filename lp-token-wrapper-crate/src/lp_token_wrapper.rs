use crate::data::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256};
// use common::errors::*;
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casper_types::ApiError;
#[repr(u16)]
pub enum Error {
    /// 65,540 for (Lp Token Wrapper Addition Error 1)
    LpTokenWrapperAdditionError1 = 11901,
    /// 65,540 for (Lp Token Wrapper Addition Error 2)
    LpTokenWrapperAdditionError2 = 11902,
    /// 65,540 for (Lp Token Wrapper Subtraction Error 1)
    LpTokenWrapperSubtractionError1 = 11903,
    /// 65,540 for (Lp Token Wrapper Subtraction Error 2)
    LpTokenWrapperSubtractionError2 = 11904,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait LPTOKENWRAPPER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, uni: Key, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        set_uni(uni);
        set_hash(contract_hash);
        set_package_hash(package_hash);
        Balances::init();
    }
    fn total_supply(&self) -> U256 {
        return get_total_supply();
    }
    fn balance_of(&self, account: Key) -> U256 {
        return Balances::instance().get(&account);
    }
    fn stake(&mut self, amount: U256) {
        set_total_supply(
            get_total_supply()
                .checked_add(amount)
                .unwrap_or_revert_with(Error::LpTokenWrapperAdditionError1),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_add(amount)
                .unwrap_or_revert_with(Error::LpTokenWrapperAdditionError2),
        );
        let _ret: Result<(), u32> = runtime::call_versioned_contract(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => Key::from(get_package_hash()),
                "amount" => amount
            },
        );
    }
    fn withdraw(&mut self, amount: U256) {
        set_total_supply(
            get_total_supply()
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::LpTokenWrapperSubtractionError1),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::LpTokenWrapperSubtractionError2),
        );
        let _ret: Result<(), u32> = runtime::call_versioned_contract(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => self.get_caller(),
                "amount" => amount
            },
        );
    }
}
