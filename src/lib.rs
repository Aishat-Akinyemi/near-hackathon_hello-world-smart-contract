//! This contract implements simple hello world on the near blockchain.
//!
//! The contract provides a method that takes a {name} parameter and returns “Hello {name}!”
//!
//! [say_hello]: struct.HelloWorld.html#method.say_hello


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;


near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {  
    name_records: HashMap<AccountId, String>
}

#[near_bindgen]
impl HelloWorld {   
    pub fn say_hello(&mut self, name: String) -> String {        
        let account_id = env::signer_account_id();
        let log_message = format!("{} said hello world with name: {}", account_id, name);
        env::log(log_message.as_bytes());
        self.name_records.insert(account_id, name.to_string());
        let hello = String::from("hello ");
        return hello+ &name.to_string();
    } 
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package HelloWorld-- --nocapture
 * Note: 'rust-HelloWorld-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::{get_logs};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]   
    fn say_hello(){
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable 
        let mut contract = HelloWorld::default();
        assert_eq!("hello aishat", contract.say_hello("aishat".to_string()));
    }

    #[test]  
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HelloWorld::default();
        contract.say_hello("tundun".to_string());
        assert_eq!(get_logs(), vec!["bob.testnet said hello world with name: tundun"])
    }
  
}