mod balances;
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
}

enum RuntimeCall {
	
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Runtime>,
	balances: balances::Pallet<Runtime>,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
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

fn main() {
	let mut runtime = Runtime::new();

	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");

	runtime.balances.set_balance(&alice, 100);
	runtime.system.inc_block_number();

	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| eprintln!("{}", e));

	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}
