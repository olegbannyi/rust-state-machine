use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

/// This is the System Pallet
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number
	block_number: BlockNumber,
	/// A map from an account to their nonce
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	/// Create a new instance of the System Pallet
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn get_nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&0)
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number.checked_add(1).unwrap();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let current_nonce = self.nonce.get(who).unwrap_or(&0);
		let new_nonce = current_nonce.checked_add(1).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn init_system() {
		// Arrange
		// Act
		let system = Pallet::new();
		// Assert
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		// Arrange
		let mut system = Pallet::new();
		// Act
		system.inc_block_number();
		// Assert
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		// Arrange
		let mut system = Pallet::new();
		let alice = AccountId::from("alice");
		// Act
		system.inc_nonce(&alice);
		// Assert
		assert_eq!(system.get_nonce(&alice), 1);
	}
}
