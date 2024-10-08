use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::system::Config as SystemConfig;

pub trait Config: SystemConfig {
	type Balance: CheckedSub + CheckedAdd + Zero + Copy;
}

/// This is the Balances Module
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_from_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Fund overflow.")?;

		self.set_balance(&caller, new_from_balance);
		self.set_balance(&to, new_to_balance);
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of balance module
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::system::Config as SystemConfig;

	struct TestConfig;

	impl SystemConfig for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let (mut balances, alice, bob) = setup();

		assert_eq!(balances.balance(&alice), 0);
		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_balance() {
		// That `alice` can successfully transfer funds to `bob`
		// That the balance of `alice` and `bob` is correctly updated.
		// Arrange
		let (mut balances, alice, bob) = setup();
		balances.set_balance(&alice, 100);
		// Act
		let result = balances.transfer(alice.clone(), bob.clone(), 75);
		// Assert
		assert_eq!(result, Ok(()));
		assert_eq!(balances.balance(&alice), 25);
		assert_eq!(balances.balance(&bob), 75);
	}

	#[test]
	fn transfer_balance_insufficient() {
		// Arrange
		let (mut balances, alice, bob) = setup();
		balances.set_balance(&alice, 100);
		// Act
		let result = balances.transfer(alice, bob, 110);
		// Assert
		assert_eq!(result, Err("Not enough funds."));
	}

	#[test]
	fn transfer_balance_overflow() {
		// Arrange
		let (mut balances, alice, bob) = setup();
		balances.set_balance(&alice, 100);
		balances.set_balance(&bob, std::u128::MAX);
		// Act
		let result = balances.transfer(alice, bob, 1);
		// Assert
		assert_eq!(result, Err("Fund overflow."));
	}

	fn setup() -> (
		Pallet<TestConfig>,
		<TestConfig as SystemConfig>::AccountId,
		<TestConfig as SystemConfig>::AccountId,
	) {
		let balances = Pallet::new();
		let alice = String::from("alice");
		let bob = String::from("bob");

		(balances, alice, bob)
	}
}
