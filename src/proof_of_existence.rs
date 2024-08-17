use std::{collections::BTreeMap, fmt::Debug};

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which repersents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that descision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		match self.get_claim(&claim) {
			Some(_) => Err("Claim already exists"),
			None => {
				self.claims.insert(claim, caller);
				Ok(())
			},
		}
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

		if claim_owner != &caller {
			return Err("Caller is not the owner of the claim");
		}

		self.claims.remove(&claim);
		Ok(())
	}
}
impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;

		type BlockNumber = u32;

		type Nonce = u32;
	}

	impl Config for TestConfig {
		type Content = &'static str;
	}

	#[test]
	fn return_none_on_non_exisiting_content() {
		// Arrange
		let (poe, _, _) = setup();
		let content = "Lorem ipsum";

		// Act
		let res = poe.get_claim(&content);

		// Assert
		assert_eq!(res, None);
	}

	#[test]
	fn create_claim_success() {
		// Arrange
		let (mut poe, alice, _) = setup();
		let content = "Lorem ipsum";

		// Act
		let res = poe.create_claim(alice, content);
		let claim_owner = poe.get_claim(&content);

		// Assert
		assert_eq!(res, Ok(()));
		assert_eq!(claim_owner, Some(&alice));
	}

	#[test]
	fn create_claim_failure_on_existant_for_the_same_owner() {
		// Arrange
		let (mut poe, alice, _) = setup();
		let content = "Lorem ipsum";
		let _ = poe.create_claim(alice, content);

		// Act
		let res = poe.create_claim(alice, content);

		// Assert
		assert_eq!(res, Err("Claim already exists"));
	}

	#[test]
	fn create_claim_failure_on_existant_for_the_another_owner() {
		// Arrange
		let (mut poe, alice, bob) = setup();
		let content = "Lorem ipsum";
		let _ = poe.create_claim(bob, content);

		// Act
		let res = poe.create_claim(alice, content);

		// Assert
		assert_eq!(res, Err("Claim already exists"));
	}

	#[test]
	fn revoke_claim_success() {
		// Arrange
		let (mut poe, alice, _) = setup();
		let content = "Lorem ipsum";
		let _ = poe.create_claim(alice, content);

		// Act
		let res = poe.revoke_claim(alice, content);
		let claim_owner = poe.get_claim(&content);

		// Assert
		assert_eq!(res, Ok(()));
		assert_eq!(claim_owner, None);
	}

	#[test]
	fn revoke_claim_error_does_not_exist() {
		// Arrange
		let (mut poe, alice, _) = setup();
		let content_1 = "Lorem ipsum";
		let content_2 = "Foo bar";
		let _ = poe.create_claim(alice, content_1);

		// Act
		let res = poe.revoke_claim(alice, content_2);

		// Assert
		assert_eq!(res, Err("Claim does not exist"));
	}

	#[test]
	fn revoke_claim_error_caller_not_owner() {
		// Arrange
		let (mut poe, alice, bob) = setup();
		let content = "Lorem ipsum";
		let _ = poe.create_claim(alice, content);

		// Act
		let res = poe.revoke_claim(bob, content);

		// Assert
		assert_eq!(res, Err("Caller is not the owner of the claim"));
	}

	fn setup() -> (Pallet<TestConfig>, &'static str, &'static str) {
		(Pallet::new(), "alice", "bob")
	}
}
