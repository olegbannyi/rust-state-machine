use support::Dispatch;

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
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
	system: system::Pallet<Runtime>,
	balances: balances::Pallet<Runtime>,
	proof_of_existence: proof_of_existence::Pallet<Runtime>,
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

fn main() {
	let mut runtime = Runtime::new();

	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");

	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: charlie.clone(),
					amount: 20,
				}),
			},
		],
	};

	runtime.execute_block(block_1).expect("Block handling error");

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Alice's document",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Bob's document",
				}),
			},
		],
	};

	runtime.execute_block(block_2).expect("Block handling error");

	println!("{:#?}", runtime);
}
