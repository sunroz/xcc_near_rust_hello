use near_sdk::{ext_contract};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

// Interface of this contract, for callbacks
#[ext_contract(this_contract)]
trait Callbacks {
  fn query_greeting_callback(&mut self) -> String;
  fn change_greeting_callback(&mut self) -> bool;
  fn query_get_signer_account_id_callback(&mut self) -> String;
  fn query_get_current_account_id_callback(&mut self) -> String;
  fn query_get_predecessor_account_id_callback(&mut self) -> String;
}

// Validator interface, for cross-contract calls
#[ext_contract(near_rust_hello)]
trait NearRustHello {
  fn get_greeting(&self) -> String;
  fn set_greeting(&mut self, message: String);
  fn get_signer_account_id(&mut self) -> String;
  fn get_current_account_id(&mut self) -> String;
  fn get_predecessor_account_id(&mut self) -> String;
}
