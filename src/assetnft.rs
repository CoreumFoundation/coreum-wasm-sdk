use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin};

use crate::pagination::{PageRequest, PageResponse};

#[cw_serde]
pub struct Params {
    pub issue_fee: Coin,
}

#[cw_serde]
pub struct ParamsResponse {
    pub params: Params,
}

#[cw_serde]
pub struct Class {
    pub id: String,
    pub issuer: String,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub data: Option<Binary>,
    pub features: Option<Vec<u32>>,
    pub royalty_rate: Option<String>,
}

#[cw_serde]
pub struct ClassResponse {
    pub class: Class,
}

#[cw_serde]
pub struct ClassesResponse {
    pub pagination: PageResponse,
    pub classes: Vec<Class>,
}

#[cw_serde]
pub struct FrozenResponse {
    pub frozen: bool,
}

#[cw_serde]
pub struct WhitelistedResponse {
    pub whitelisted: bool,
}

#[cw_serde]
pub struct WhitelistedAccountsForNFTResponse {
    pub pagination: PageResponse,
    pub accounts: Vec<String>,
}

#[cw_serde]
pub enum Msg {
    IssueClass {
        name: String,
        symbol: String,
        description: Option<String>,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
        features: Option<Vec<u32>>,
        royalty_rate: Option<String>,
    },
    Mint {
        class_id: String,
        id: String,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
    },
    Burn {
        class_id: String,
        id: String,
    },
    Freeze {
        class_id: String,
        id: String,
    },
    Unfreeze {
        class_id: String,
        id: String,
    },
    AddToWhitelist {
        class_id: String,
        id: String,
        account: String,
    },
    RemoveFromWhitelist {
        class_id: String,
        id: String,
        account: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Query {
    #[returns(ParamsResponse)]
    Params {},

    #[returns(ClassResponse)]
    Class { id: String },

    #[returns(ClassesResponse)]
    Classes {
        pagination: PageRequest,
        issuer: String,
    },

    #[returns(FrozenResponse)]
    Frozen { id: String, class_id: String },

    #[returns(WhitelistedResponse)]
    Whitelisted {
        id: String,
        class_id: String,
        account: String,
    },

    #[returns(WhitelistedAccountsForNFTResponse)]
    WhitelistedAccountsForNFT {
        pagination: PageRequest,
        id: String,
        class_id: String,
    },
}
