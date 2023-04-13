#![allow(dead_code)]

pub mod contract {
    use ink_env::{AccountId, Balance, Hash};

    pub struct Storage {
        total_balance: Balance,
        hash_of_child_contract: Hash,
        account_id: AccountId,
    }
    
    impl Storage {
        pub fn new() -> Self {
           Self {
               // With default `ink-env-config`
               // total_balance: 123u128,
               // hash_of_child_contract: [1; 32],
               // account_id: [2; 32],
               total_balance: 123u32,
               hash_of_child_contract: [1; 64],
               account_id: [2; 16],
           }
        }
    }
}