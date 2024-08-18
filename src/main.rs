use rust_state_machine::{
	balances, proof_of_existence,
	support::{self, Header},
	types, Runtime, RuntimeCall,
};

fn main() {
	let mut runtime = Runtime::instace();

	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");

	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: Header { block_number: 1 },
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

	runtime.execute(block_1).expect("Block handling error");

	let block_2 = types::Block {
		header: Header { block_number: 2 },
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

	runtime.execute(block_2).expect("Block handling error");

	println!("{:#?}", runtime);
}
