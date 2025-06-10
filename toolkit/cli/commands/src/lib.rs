//! # CLI Commands for Partner Chains
//!
//! This crate provides command structures and execution logic for Partner Chains operations.
//! These commands are integrated into the `partner-chains-demo-node` binary to provide
//! cryptographic operations, signature generation, and blockchain interaction capabilities.
//!
//! ## Core Functionality
//!
//! - **Registration Signatures**: Generate cryptographic signatures for validator registration
//! - **Address Association**: Create signatures linking Cardano and Partner Chain addresses
//! - **Block Producer Metadata**: Sign metadata for block producer operations
//! - **Genesis UTXO Retrieval**: Query the genesis UTXO from on-chain storage
//! - **Key Parameter Handling**: Parse and validate various cryptographic key formats
//!
//! ## Integration
//!
//! These commands are exposed through the Partner Chains node CLI via the
//! `PartnerChainsSubcommand` enum in the `partner-chains-node-commands` crate.
//! Each command implements the clap `Parser` trait for argument parsing and provides
//! an `execute` method for performing the required operations.
//!
//! ## Architecture
//!
//! Commands follow a consistent pattern:
//! - Struct fields represent command parameters with clap annotations
//! - `execute` methods perform the core logic and output results
//! - Key parameter types provide secure parsing and validation
//! - Output structures serialize results as JSON for consumption
//! 
//! ## Usage Examples
//!
//! ### Address Association Signature
//!
//! ```bash
//! cargo run --bin partner-chains-demo-node -- sign-address-association \
//!   --genesis-utxo 59104061ffa0d66f9ba0135d6fc6a884a395b10f8ae9cb276fc2c3bfdfedc260#1 \
//!   --partnerchain-address d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d \
//!   --signing-key d75c630516c33a66b11b3444a70b65083aeb21353bd919cc5e3daa02c9732a84
//! ```
//!
//! ### Block Producer Metadata Signature
//!
//! ```bash
//! cargo run --bin partner-chains-demo-node -- sign-block-producer-metadata \
//!   --genesis-utxo 0101010101010101010101010101010101010101010101010101010101010101#0 \
//!   --metadata-file metadata.json \
//!   --cross-chain-signing-key cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854
//! ```
//!
//! ### Validator Registration Signatures
//!
//! ```bash
//! cargo run --bin partner-chains-demo-node -- registration-signatures \
//!   --genesis-utxo e41c9b57841e582c207bb68d5e9736fb48c7af5f1ec29ade00692fa5e0e47efa#4 \
//!   --mainchain-signing-key 2bebcb7fbc74a6e0fd6e00a311698b047b7b659f0e047ff5349dbd984aefc52c \
//!   --sidechain-signing-key 02dbfc8b66c22f931a6647fd86db2fc073dd564b99837226a1bdfe7a99578854ec \
//!   --registration-utxo 8ea10040249ad3033ae7c4d4b69e0b2e2b50a90741b783491cb5ddf8ced0d861#4
//! ```
//! 
//! ### Genesis UTXO Retrieval
//!
//! ```bash
//! cargo run --bin partner-chains-demo-node -- sidechain-params \
//!   --dev
//! ```

pub mod address_association_signatures;
pub mod block_producer_metadata_signatures;
pub mod get_genesis_utxo;
pub mod key_params;
pub mod registration_signatures;
