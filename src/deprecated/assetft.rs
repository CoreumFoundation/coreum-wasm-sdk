use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::deprecated::pagination::{PageRequest, PageResponse};

pub const MINTING: u32 = 0;
pub const BURNING: u32 = 1;
pub const FREEZING: u32 = 2;
pub const WHITELISTING: u32 = 3;
pub const IBC: u32 = 4;
pub const BLOCK_SMART_CONTRACTS: u32 = 5;
pub const CLAWBACK: u32 = 6;
pub const EXTENSION: u32 = 7;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub issue_fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Token {
    pub denom: String,
    pub issuer: String,
    pub symbol: String,
    pub subunit: String,
    pub precision: u32,
    pub description: Option<String>,
    pub globally_frozen: Option<bool>,
    pub features: Option<Vec<u32>>,
    pub burn_rate: String,
    pub send_commission_rate: String,
    pub version: u32,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub extension_cw_address: Option<String>,
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokensResponse {
    pub pagination: PageResponse,
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenResponse {
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BalanceResponse {
    pub balance: String,
    pub whitelisted: String,
    pub frozen: String,
    pub locked: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FrozenBalancesResponse {
    pub pagination: PageResponse,
    pub balances: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FrozenBalanceResponse {
    pub balance: Coin,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistedBalancesResponse {
    pub pagination: PageResponse,
    pub balances: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistedBalanceResponse {
    pub balance: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Msg {
    Issue {
        symbol: String,
        subunit: String,
        precision: u32,
        initial_amount: Uint128,
        description: Option<String>,
        features: Option<Vec<u32>>,
        burn_rate: String,
        send_commission_rate: String,
        uri: Option<String>,
        uri_hash: Option<String>,
    },
    Mint {
        coin: Coin,
        recipient: Option<String>,
    },
    Burn {
        coin: Coin,
    },
    Freeze {
        account: String,
        coin: Coin,
    },
    Unfreeze {
        account: String,
        coin: Coin,
    },
    SetFrozen {
        account: String,
        coin: Coin,
    },
    GloballyFreeze {
        denom: String,
    },
    GloballyUnfreeze {
        denom: String,
    },
    SetWhitelistedLimit {
        account: String,
        coin: Coin,
    },
    UpgradeTokenV1 {
        denom: String,
        ibc_enabled: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
pub enum Query {
    #[returns(ParamsResponse)]
    Params {},

    #[returns(TokensResponse)]
    Tokens {
        pagination: Option<PageRequest>,
        issuer: String,
    },

    #[returns(TokenResponse)]
    Token { denom: String },

    #[returns(BalanceResponse)]
    Balance { account: String, denom: String },

    #[returns(FrozenBalancesResponse)]
    FrozenBalances {
        pagination: Option<PageRequest>,
        account: String,
    },

    #[returns(FrozenBalanceResponse)]
    FrozenBalance { account: String, denom: String },

    #[returns(WhitelistedBalancesResponse)]
    WhitelistedBalances {
        pagination: Option<PageRequest>,
        account: String,
    },

    #[returns(WhitelistedBalanceResponse)]
    WhitelistedBalance { account: String, denom: String },
}
