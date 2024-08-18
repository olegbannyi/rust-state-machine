use support::{Dispatch, DispatchResult};

pub mod balances;
pub mod proof_of_existence;
pub mod support;
pub mod system;

pub mod types {
	use crate::{support, RuntimeCall};

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	pub system: system::Pallet<Runtime>,
	pub balances: balances::Pallet<Runtime>,
	pub proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

impl Runtime {
	pub fn instace() -> Self {
		Self::new()
	}

	pub fn execute(&mut self, block: types::Block) -> DispatchResult {
		self.execute_block(block)
	}
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_instance() {
        // Act
        let runtime = Runtime::instace();

        // Assert
        assert!(runtime.system.block_number() == 0);
    }
}
