/// The size of the buffer for encoding and decoding.
pub const BUFFER_SIZE: usize = 1 << 14; // 16 kB

/// The maximum number of supported event topics provided by the runtime.
///
/// The value must match the maximum number of supported event topics of the used
/// runtime.
pub const MAX_EVENT_TOPICS: usize = 4;

// Modified from `[u8; 32]`.
/// The account id type.
pub type AccountId = [u8; 16];

// Modified from `u128`.
/// The type of balances.
pub type Balance = u32;

// Modified from `[u8; 32]`.
/// The type of hash.
pub type Hash = [u8; 64];

/// The type of a timestamp.
pub type Timestamp = u32;

/// The type of block number.
pub type BlockNumber = u64;