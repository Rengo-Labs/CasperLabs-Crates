use crate::data;
use casper_contract::contract_api::runtime;
use casper_types::{ApiError, ContractHash, ContractPackageHash, Key};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casperlabs_ownable::OWNABLE;

#[repr(u16)]
pub enum Error {
    /// 65,540 for (IRewardDistributionRecipient: Caller is not reward distribution)
    NotRewardDistribution = 11601,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait IREWARDDISTRIBUTIONRECIPIENT<Storage: ContractStorage>:
    ContractContext<Storage> + OWNABLE<Storage>
{
    fn init(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        OWNABLE::init(self, contract_hash, package_hash);
        data::set_contract_hash(contract_hash);
        data::set_package_hash(package_hash);
    }
    fn only_reward_distribution(&self) {
        if !(self.get_caller() == data::get_reward_distribution()) {
            runtime::revert(ApiError::from(Error::NotRewardDistribution));
        }
    }
    fn set_reward_distribution(&self, reward_distribution: Key) {
        OWNABLE::only_owner(self);
        data::set_reward_distribution(reward_distribution);
    }
}
