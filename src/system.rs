use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + Copy + AddAssign;
	type Nonce: Clone + Copy + Zero + One;
}

/// This is the System Pallet
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number
	block_number: T::BlockNumber,
	/// A map from an account to their nonce
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestConfig;

	impl Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		// Arrange
		// Act
		let system = Pallet::<TestConfig>::new();
		// Assert
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		// Arrange
		let mut system = Pallet::<TestConfig>::new();
		// Act
		system.inc_block_number();
		// Assert
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		// Arrange
		let mut system = Pallet::<TestConfig>::new();
		let alice = <TestConfig as Config>::AccountId::from("alice");
		// Act
		system.inc_nonce(&alice);
		// Assert
		assert_eq!(system.get_nonce(&alice), 1);
	}
}
