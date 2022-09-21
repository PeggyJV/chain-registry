#![allow(clippy::derive_partial_eq_without_eq)]
/// Contains models for serializing and deserializing the `chain.json` in a given chain's directory in the registry repository
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// by denying unknown fields we can be more confident that our structs match the
// current configured GIT_REF's schema. errors will occur if the chain.json is
// formatted incorrectly, however.
#[serde(default,)]
pub struct ChainInfo {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub chain_name: String,
    pub status: String,
    pub network_type: String,
    pub pretty_name: String,
    pub chain_id: String,
    pub bech32_prefix: String,
    pub daemon_name: String,
    pub node_home: String,
    pub slip44: u32,
    pub genesis: Genesis,
    pub codebase: Codebase,
    pub peers: Peers,
    pub apis: Apis,
    pub fees: Fees,
    pub staking: Staking,
    pub website: String,
    pub update_link: String,
    pub key_algos: Vec<String>,
    pub explorers: Vec<Explorer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Genesis {
    pub genesis_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Codebase {
    pub git_repo: String,
    pub recommended_version: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub compatible_versions: Vec<String>,
    pub binaries: Binaries,
    pub cosmos_sdk_version: String,
    pub tendermint_version: String,
    pub cosmwasm_version: String,
    pub cosmwasm_enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Binaries {
    #[serde(rename = "linux/amd64")]
    pub linux_amd_64: String,
    #[serde(rename = "linux/arm64")]
    pub linux_arm_64: String,
    #[serde(rename = "darwin/amd64")]
    pub darwin_amd_64: String,
    #[serde(rename = "darwin/arm64")]
    pub darwin_arm_64: String,
    #[serde(rename = "windows/amd64")]
    pub windows_amd_64: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Peers {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub seeds: Vec<Seed>,
    pub persistent_peers: Vec<PersistentPeer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Seed {
    pub id: String,
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PersistentPeer {
    pub id: String,
    pub address: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Apis {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rpc: Vec<Rpc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rest: Vec<Rest>,
    pub grpc: Vec<Grpc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Rpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Rest {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Grpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Fees {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub fee_tokens: Vec<FeeToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct FeeToken {
    pub denom: String,
    pub fixed_min_gas_price: f32,
    pub low_gas_price: f32,
    pub average_gas_price: f32,
    pub high_gas_price: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Staking {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub staking_tokens: Vec<StakingToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct StakingToken {
    pub denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default,)]
pub struct Explorer {
    pub kind: String,
    pub url: String,
    pub tx_page: String,
    pub account_page: String,
}
