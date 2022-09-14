/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, PanicOnDefault, Promise, Gas, env};

pub mod external;
pub use crate::external::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub xcc_contract_address: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(xcc_contract_address: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { xcc_contract_address}
    }

     // Public - query external greeting
    pub fn query_greeting(&self) -> Promise {
        // Create a promise to call NearRustHello.get_greeting()
        let promise = near_rust_hello::ext(self.xcc_contract_address.clone())
        .with_static_gas(Gas(9*TGAS))
        .get_greeting();

        return promise.then( // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(9*TGAS))
            .query_greeting_callback()
        )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_greeting_callback(&self, #[callback_result] call_result: Result<String, near_sdk::PromiseError>) -> String {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting NearRustHello");
            return "".to_string();
        }

        // Return the greeting
        let greeting: String = call_result.unwrap();
        greeting
    }

    // Public - change external greeting
    pub fn change_greeting(&mut self, new_greeting: String) -> Promise {
        // Create a promise to call HelloNEAR.set_greeting(message:string)
        near_rust_hello::ext(self.xcc_contract_address.clone())
        .with_static_gas(Gas(9*TGAS))
        .set_greeting(new_greeting)
        .then( // Create a callback change_greeting_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(9*TGAS))
            .change_greeting_callback()
        )
    }

    #[private]
    pub fn change_greeting_callback(&mut self, #[callback_result] call_result: Result<(), near_sdk::PromiseError>) -> bool {
        // Return whether or not the promise succeeded using the method outlined in external.rs
        if call_result.is_err() {
            env::log_str("set_greeting failed...");
            false
        } else {
            env::log_str("set_greeting was successful!");
            true
        }
    }

    pub fn query_get_signer_account_id(&mut self) -> Promise {
        // Create a promise to call NearRustHello.get_signer_account_id()
        let promise = near_rust_hello::ext(self.xcc_contract_address.clone())
        .with_static_gas(Gas(9*TGAS))
        .get_signer_account_id();

        return promise.then(  // Create a promise to callback query_get_signer_account_id_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(9*TGAS))
            .query_get_signer_account_id_callback()
        )
    }

    pub fn query_get_signer_account_id_callback(&mut self, #[callback_result] call_result: Result<String, near_sdk::PromiseError>) -> String {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting NearRustHello");
            return "".to_string();
        }

        // Return the signer_account_id
        let signer_account_id: String = call_result.unwrap();
        signer_account_id
    }

    pub fn query_get_current_account_id(&mut self) -> Promise {
        // Create a promise to call NearRustHello.get_current_account_id()
        let promise = near_rust_hello::ext(self.xcc_contract_address.clone())
        .with_static_gas(Gas(9*TGAS))
        .get_current_account_id();

        return promise.then(  // Create a promise to callback query_get_current_account_id_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(9*TGAS))
            .query_get_current_account_id_callback()
        )
    }

    pub fn query_get_current_account_id_callback(&mut self, #[callback_result] call_result: Result<String, near_sdk::PromiseError>) -> String {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting NearRustHello");
            return "".to_string();
        }

        // Return the current_account_id
        let current_account_id: String = call_result.unwrap();
        current_account_id
    }

    pub fn query_get_predecessor_account_id(&mut self) -> Promise {
        // Create a promise to call NearRustHello.get_predecessor_account_id()
        let promise = near_rust_hello::ext(self.xcc_contract_address.clone())
        .with_static_gas(Gas(9*TGAS))
        .get_predecessor_account_id();

        return promise.then(  // Create a promise to callback query_get_predecessor_account_id_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(9*TGAS))
            .query_get_predecessor_account_id_callback()
        )
    }

    pub fn query_get_predecessor_account_id_callback(&mut self, #[callback_result] call_result: Result<String, near_sdk::PromiseError>) -> String {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting NearRustHello");
            return "".to_string();
        }

        // Return the predecessor_account_id
        let predecessor_account_id: String = call_result.unwrap();
        predecessor_account_id
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
