#![no_std]
extern crate alloc;

pub mod data;
mod erc20;

pub use casperlabs_contract_utils;
pub use erc20::{Error, ERC20};
