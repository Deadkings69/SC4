use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage, require};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;
pub mod dis_treasury_protocol;
pub mod dis_community;
pub mod dis_chess;
pub mod dis_founders;
pub mod mint;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON:&str="data:image/png;base64,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";
/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// total tokens for all time.
    pub total_tokens:u128,
    /// contract owner
    pub owner:AccountId,
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,

    //  TREASURY PROTOCOL
    /// Wallet Account Id for Treasury protocol grants
    pub treasury_protocol_ac: AccountId,
    /// granted months
    pub treasury_months:u8,
    /// treasury timestamp
    pub t_timestamp:u64,
    /// treasury protocol minted tokens
    pub treasury_minted_tokens:u128,

    //  COMMUNITY
    /// Wallet Account Id for community grants
    pub community_ac: AccountId,
    /// granted months
    pub community_months:u8,
    /// treasury timestamp
    pub com_timestamp:u64,
    /// community minted tokens
    pub community_minted_tokens:u128,

    //  CHESS TOURNMENT
    /// Wallet Account Id for chess tournment grants
    pub chess_ac: AccountId,
    /// granted months
    pub chess_months:u8,
    /// treasury timestamp
    pub chess_timestamp:u64,
    /// chess minted tokens
    pub chess_minted_tokens:u128,

    //  FOUNDERS
    /// Wallet Account Id for founders grants
    pub founders_ac: AccountId,
    /// granted months
    pub founders_months:u8,
    /// treasury timestamp
    pub founders_timestamp:u64,
    /// founders minted tokens
    pub founders_minted_tokens:u128,

    // PUBLIC MINTING
    /// after initial minted supply. remaining minting for publically available tokens
    /// will counted in this variable.
    pub public_minting:u128

}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Dead Kings".to_string(),
                symbol: "DK".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            //set total token
            total_tokens:1_000_000_000,
            // set owner
            owner:owner_id.clone(),
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            //treasury
            treasury_protocol_ac: "deadkings.near".parse().unwrap(),
            treasury_months:1,
            t_timestamp:env::block_timestamp_ms(),
            treasury_minted_tokens: 0,
            // community
            community_ac: "deadkings.near".parse().unwrap(),
            community_months:1,
            com_timestamp:env::block_timestamp_ms(),
            community_minted_tokens:0,
            // chess tournment
            chess_ac: "deadkings.near".parse().unwrap(),
            chess_months:1,
            chess_timestamp:env::block_timestamp_ms(),
            chess_minted_tokens:0,
            // founders
            founders_ac: "deadkings.near".parse().unwrap(),
            founders_months:1,
            founders_timestamp:env::block_timestamp_ms(),
            founders_minted_tokens:0,
            public_minting:0
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }

}