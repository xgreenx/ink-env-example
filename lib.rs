#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[brush::contract]
pub mod ownership {
    use psp721::traits::*;
    use access_control::traits::*;
    use brush::modifiers;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, PSP721Storage, AccessControlStorage)]
    pub struct Ownership {
        #[PSP721StorageField]
        land: PSP721Data,
        #[AccessControlStorageField]
        access: AccessControlData
    }

    impl IPSP721 for Ownership {}
    
    impl AccessControl for Ownership {}
    
    impl Ownership {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

    }
}
