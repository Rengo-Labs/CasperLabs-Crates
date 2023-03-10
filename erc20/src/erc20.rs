use crate::alloc::string::ToString;
use crate::data::{self, get_package_hash, Allowances, Balances, Nonces};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    system::mint::Error as MintError, ApiError, ContractPackageHash, Key, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

pub enum ERC20Event {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
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
            } => "erc20_transfer",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,536 for (UniswapV2 Core ERC20 EXPIRED)
    UniswapV2CoreERC20EXPIRED = 0,
    /// 65,537 for (UniswapV2 Core ERC20 Signature Verification Failed)
    UniswapV2CoreERC20SignatureVerificationFailed = 1,
    /// 65,538 for (UniswapV2 Core ERC20 OverFlow1)
    UniswapV2CoreERC20OverFlow1 = 2,
    /// 65,539 for (UniswapV2 Core ERC20 OverFlow2)
    UniswapV2CoreERC20OverFlow2 = 3,
    /// 65,540 for (UniswapV2 Core ERC20 OverFlow3)
    UniswapV2CoreERC20OverFlow3 = 4,
    /// 65,541 for (UniswapV2 Core ERC20 OverFlow4)
    UniswapV2CoreERC20OverFlow4 = 5,
    /// 65,542 for (UniswapV2 Core ERC20 UnderFlow1)
    UniswapV2CoreERC20UnderFlow1 = 6,
    /// 65,543 for (UniswapV2 Core ERC20 UnderFlow2)
    UniswapV2CoreERC20UnderFlow2 = 7,
    /// 65,544 for (UniswapV2 Core ERC20 UnderFlow3)
    UniswapV2CoreERC20UnderFlow3 = 8,
    /// 65,545 for (UniswapV2 Core ERC20 UnderFlow4)
    UniswapV2CoreERC20UnderFlow4 = 9,
    /// 65,546 for (UniswapV2 Core ERC20 UnderFlow5)
    UniswapV2CoreERC20UnderFlow5 = 10,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait ERC20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_total_supply(initial_supply);
        data::set_decimals(decimals);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        Nonces::init();
        let nonces = Nonces::instance();
        nonces.set(&Key::from(self.get_caller()), U256::from(0));
        Allowances::init();
        Balances::init();
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn nonce(&mut self, owner: Key) -> U256 {
        Nonces::instance().get(&owner)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self.make_transfer(self.get_caller(), recipient, amount)
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        self._approve(self.get_caller(), spender, amount);
    }

    fn _approve(&mut self, owner: Key, spender: Key, amount: U256) {
        Allowances::instance().set(&owner, &spender, amount);
        self.emit(&ERC20Event::Approval {
            owner: owner,
            spender: spender,
            value: amount,
        });
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn increase_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();
        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);
        let new_allowance: U256 = spender_allowance
            .checked_add(amount)
            .ok_or(Error::UniswapV2CoreERC20OverFlow1)
            .unwrap_or_revert();

        if owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn decrease_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);

        let new_allowance: U256 = spender_allowance
            .checked_sub(amount)
            .ok_or(Error::UniswapV2CoreERC20UnderFlow1)
            .unwrap_or_revert();

        if new_allowance >= 0.into() && new_allowance < spender_allowance && owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        if owner != recipient && amount != 0.into() {
            let ret: Result<(), u32> = self.make_transfer(owner, recipient, amount);
            if ret.is_ok() {
                let allowances = Allowances::instance();
                let spender_allowance: U256 = allowances.get(&owner, &self.get_caller());
                let new_allowance: U256 = spender_allowance
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow2)
                    .unwrap_or_revert();
                if new_allowance >= 0.into()
                    && new_allowance < spender_allowance
                    && owner != self.get_caller()
                {
                    self._approve(owner, self.get_caller(), new_allowance);
                    return Ok(());
                } else {
                    return Err(4);
                }
            }
        }
        Ok(())
    }

    fn mint(&mut self, recipient: Key, amount: U256) {
        let balances: Balances = Balances::instance();
        let balance: U256 = balances.get(&recipient);
        balances.set(
            &recipient,
            balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreERC20OverFlow2)
                .unwrap_or_revert(),
        );
        data::set_total_supply(
            data::total_supply()
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreERC20OverFlow3)
                .unwrap_or_revert(),
        );
        let address_0: Key = Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        self.emit(&ERC20Event::Transfer {
            from: address_0,
            to: recipient,
            value: amount,
        });
    }

    fn burn(&mut self, recipient: Key, amount: U256) {
        let balances: Balances = Balances::instance();
        let balance: U256 = balances.get(&recipient);
        if balance >= amount {
            balances.set(
                &recipient,
                balance
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow3)
                    .unwrap_or_revert(),
            );
            data::set_total_supply(
                data::total_supply()
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow4)
                    .unwrap_or_revert(),
            );
            let address_0: Key = Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap();
            self.emit(&ERC20Event::Transfer {
                from: recipient,
                to: address_0,
                value: amount,
            });
        } else {
            // PosError::InsufficientPaymentForAmountSpent
            runtime::revert(MintError::InsufficientFunds)
        }
    }

    fn set_nonce(&mut self, recipient: Key) {
        let nonces: Nonces = Nonces::instance();
        let nonce: U256 = nonces.get(&recipient);
        nonces.set(&recipient, nonce + U256::from(1));
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        if sender != recipient && amount != 0.into() {
            let balances: Balances = Balances::instance();
            let sender_balance: U256 = balances.get(&sender);
            let recipient_balance: U256 = balances.get(&recipient);
            balances.set(
                &sender,
                sender_balance
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow5)
                    .unwrap_or_revert(),
            );
            balances.set(
                &recipient,
                recipient_balance
                    .checked_add(amount)
                    .ok_or(Error::UniswapV2CoreERC20OverFlow4)
                    .unwrap_or_revert(),
            );
            self.emit(&ERC20Event::Transfer {
                from: sender,
                to: recipient,
                value: amount,
            });
        }

        Ok(())
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }

    fn decimals(&mut self) -> u8 {
        data::decimals()
    }

    fn emit(&mut self, erc20_event: &ERC20Event) {
        match erc20_event {
            ERC20Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
            ERC20Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                storage::new_uref(event);
            }
        };
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
