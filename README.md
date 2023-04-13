This repository shows how any contract can override the [ink_env_config](https://github.com/xgreenx/ink-env-config) with default environment. 

The `ink_env_config` crate contains types and constants from the [`ink::env::Environment`](https://github.com/paritytech/ink/blob/6fb975d77c61bd08c662e2111c4f35b0e50a3d37/crates/env/src/types.rs#L110) trait.

The [ink_env](ink_env) simulates the [`ink_env`](https://github.com/paritytech/ink/tree/6fb975d77c61bd08c662e2111c4f35b0e50a3d37/crates/env) crate. It uses `ink_env_config` as a dependency.

The [contract](contract) simulates some contract that uses ink.
The developer of the contract wants to use its `ink_env_config` with custom types.
The developer created his local version of the [custom-ink-env-config](contract%2Fcustom-ink-env-config) with updated custom types.

```rust
// Modified from `[u8; 32]`.
/// The account id type.
pub type AccountId = [u8; 16];

// Modified from `u128`.
/// The type of balances.
pub type Balance = u32;

// Modified from `[u8; 32]`.
/// The type of hash.
pub type Hash = [u8; 64];
```

The developer patched the dependencies:

```shell
[patch."https://github.com/xgreenx/ink-env-config.git"]
ink-env-config = { path = "custom-ink-env-config" }

# If the crate is published on the `crates.io` we can use the following syntax:
# [patch.crates-io]
# ink_env_config = { path = "custom_ink_env_config" }
```

And after started to use a new types in the contract:

```rust
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
```

And compiled the contract:
```shell
cargo build --manifest-path contract/Cargo.toml
```