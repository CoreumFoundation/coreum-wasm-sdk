use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct Params {
    pub issue_fee: Coin,
}

#[cw_serde]
pub struct ParamsResponse {
    pub params: Params,
}

#[cw_serde]
pub struct Token {
    pub denom: String,
    pub issuer: String,
    pub symbol: String,
    pub subunit: String,
    pub precision: u32,
    pub description: Option<String>,
    pub features: Option<Vec<u32>>,
    pub burn_rate: String,
    pub send_commission_rate: String,
}

#[cw_serde]
pub struct TokenResponse {
    pub token: Token,
}

#[cw_serde]
pub struct FrozenBalanceResponse {
    pub balance: Coin,
}

#[cw_serde]
pub struct WhitelistedBalanceResponse {
    pub balance: Coin,
}

#[cw_serde]
pub enum Msg {
    Issue {
        symbol: String,
        subunit: String,
        precision: u32,
        initial_amount: Uint128,
        description: Option<String>,
        features: Option<Vec<u32>>,
        burn_rate: Option<String>,
        send_commission_rate: Option<String>,
    },
    Mint {
        coin: Coin,
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
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Query {
    #[returns(ParamsResponse)]
    Params {},

    #[returns(TokenResponse)]
    Token { denom: String },

    #[returns(FrozenBalanceResponse)]
    FrozenBalance { account: String, denom: String },

    #[returns(WhitelistedBalanceResponse)]
    WhitelistedBalance { account: String, denom: String },
}
