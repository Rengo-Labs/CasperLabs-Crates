use std::collections::BTreeMap;

use crate::data;
use crate::event::*;
use alloc::string::String;
use casper_erc20_crate::{Address, Error, ERC20 as CasperErc20};
use casper_types::ContractHash;
use casper_types::Key;
use casper_types::{ContractPackageHash, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

pub trait ERC20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        data::set_contract_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    fn name(&self) -> String {
        CasperErc20::default().name()
    }

    fn symbol(&self) -> String {
        CasperErc20::default().symbol()
    }

    fn decimals(&self) -> u8 {
        CasperErc20::default().decimals()
    }

    fn total_supply(&self) -> U256 {
        CasperErc20::default().total_supply()
    }

    fn balance_of(&self, owner: Address) -> U256 {
        CasperErc20::default().balance_of(owner)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        CasperErc20::default().allowance(owner, spender)
    }

    fn increase_allowance(&self, spender: Address, amount: U256) -> Result<(), Error> {
        CasperErc20::default().increase_allowance(spender, amount)
    }

    fn decrease_allowance(&self, spender: Address, amount: U256) -> Result<(), Error> {
        CasperErc20::default().decrease_allowance(spender, amount)
    }

    fn transfer(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        let ret = CasperErc20::default().transfer(recipient, amount);
        if ret.is_ok() {
            emit(&ERC20Event::Transfer {
                from: Address::from(self.get_caller()),
                to: recipient,
                value: amount,
            });
        }
        ret
    }

    fn _approve(&self, owner: Address, spender: Address, amount: U256) -> Result<(), Error> {
        let ret = CasperErc20::default()._approve(owner, spender, amount);
        if ret.is_ok() {
            emit(&ERC20Event::Approval {
                owner,
                spender,
                value: amount,
            });
        }
        ret
    }

    fn approve(&self, spender: Address, amount: U256) -> Result<(), Error> {
        let ret = CasperErc20::default().approve(spender, amount);
        if ret.is_ok() {
            emit(&ERC20Event::Approval {
                owner: Address::from(self.get_caller()),
                spender,
                value: amount,
            });
        }
        ret
    }

    fn transfer_from(&self, owner: Address, recipient: Address, amount: U256) -> Result<(), Error> {
        let ret = CasperErc20::default().transfer_from(owner, recipient, amount);
        if ret.is_ok() {
            emit(&ERC20Event::Transfer {
                from: owner,
                to: recipient,
                value: amount,
            });
        }
        ret
    }

    fn mint(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        CasperErc20::default().mint(recipient, amount)
    }

    fn burn(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        CasperErc20::default().burn(recipient, amount)
    }

    fn named_keys(
        &self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
    ) -> Result<BTreeMap<String, Key>, Error> {
        CasperErc20::default().named_keys(name, symbol, decimals, initial_supply)
    }
}
