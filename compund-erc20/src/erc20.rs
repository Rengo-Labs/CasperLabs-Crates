use crate::data;
use crate::data::{account_zero_address, zero_address, Error as Err};
use crate::event::*;
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::ContractHash;
use casper_types::Key;
use casper_types::{ContractPackageHash, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use compound_casper_erc20::{Address, Error, ERC20 as CasperErc20};
use std::collections::BTreeMap;

pub trait ERC20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        data::set_contract_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    fn set_balance(&self, owner: Address, amount: U256) {
        CasperErc20::default().write_balance(owner, amount);
    }

    fn set_allowance(&self, owner: Address, spender: Address, amount: U256) {
        CasperErc20::default().write_allowance(owner, spender, amount);
    }

    fn set_total_supply(&self, total_supply: U256) {
        CasperErc20::default().write_total_supply(total_supply);
    }

    fn name(&self) -> String {
        CasperErc20::default().name()
    }

    fn set_name(&self, name: String) {
        CasperErc20::default().set_name(name);
    }

    fn symbol(&self) -> String {
        CasperErc20::default().symbol()
    }

    fn set_symbol(&self, symbol: String) {
        CasperErc20::default().set_symbol(symbol);
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
        if recipient == Address::from(zero_address())
            || recipient == Address::from(account_zero_address())
        {
            runtime::revert(Err::CompoundErc20ZeroAddress1);
        }
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
        if recipient == Address::from(zero_address())
            || recipient == Address::from(account_zero_address())
        {
            runtime::revert(Err::CompoundErc20ZeroAddress2);
        }
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
