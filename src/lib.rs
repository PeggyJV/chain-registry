//! An API for the [Cosmos chain registry](http://github.com/cosmos/chain-registry)
//!
//! # Examples
//!
//! Populating a config struct with some of the information needed to execute transactions against a chain (doesn't compile)
//!
//! ```ignore
//! use chain_registry::*;
//!
//! #[derive(Clone, Default)]
//! struct BotConfig {
//!     account_prefix: String,
//!     chain_id: String,
//!     rpc_endpoint: String,
//!     default_asset: String,
//!     ibc_paths: HashMap<String, IBCPath>
//! }
//!
//! impl BotConfig {
//!     pub fn set_default_asset(&mut self, asset: String) {
//!         self.default_asset = asset;
//!     }
//!
//!     pub fn add_ibc_path(&mut self, path: IBCPath) {
//!         let key = format!("{}-{}", path.channel_1.chain_name, path.channel_2.chain_name);
//!         self.ibc_paths.insert(&key, path);
//!     }
//! }
//!
//! impl Into<BotConfig> for ChainInfo {
//!     fn into(self) -> BotConfig {
//!         BotConfig {
//!             account_prefix: self.bech32_prefix,
//!             chain_id: self.chain_id,
//!             // realistically you should test for health endpoints before choosing
//!             rpc_endpoint: self.apis.rpc[0].address,
//!             ..Default::default()
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! fn main() {
//!     let registry = Registry::new(None);
//!     let chain = registry.get_chain("osmosis").await.unwrap();
//!     let assets = registry.get_assets("osmosis").await.unwrap();
//!     let osmosis_hub_path = registry.get_path("osmosis", "cosmoshub").await.unwrap();
//!     let mut config: BotConfig = chain.into();
//!
//!     config.set_default_asset(assets[0]);
//!     config.add_ibc_path(osmosis_hub_path);
//!     // ...
//! }
//! ```
//!
//! Front-loading an IBC path cache for use with a REST API or indexer (doesn't compile)
//!
//! ```ignore
//! use chain_registry::*;
//!
//! #[tokio::main]
//! fn main {
//!     // This can take a long time as it has to send an individual request for each path
//!     // in the registry.
//!     let cache = RegistryCache::try_new().await.unwrap();
//!
//!     // Instead of simply querying for an IBCPath, you can filter by Tag.
//!     let use_osmosis = cache.get_paths_filtered(Tag::Dex("osmosis".to_string()));
//! }
//! ```

/// Models for assets.json ser/de
pub mod assets;

/// Models for chain.json ser/de
pub mod chain;

/// A cache type for reading IBC path data into memory for faster and filterable queries
pub mod cache;

pub mod github;
/// API for getting and listing data from the registry Github repo
pub mod registry;

/// Modles for IBC path JSON ser/de
pub mod paths;
