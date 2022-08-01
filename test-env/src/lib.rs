mod test_contract;
mod test_env;
mod utils;
use crate::test_env as other_test_env;

pub use other_test_env::TestEnv;
pub use test_contract::TestContract;
pub use test_contract::{call_contract_with_contract_hash, call_contract_with_package_hash};