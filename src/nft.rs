use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::pagination::{PageRequest, PageResponse};

#[cw_serde]
pub struct NFT {
    pub class_id: String,
    pub id: String,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub data: Option<Binary>,
}

#[cw_serde]
pub struct BalanceResponse {
    pub amount: u64,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct SupplyResponse {
    pub amount: u64,
}

#[cw_serde]
pub struct NFTsResponse {
    pub nft: NFT,
}

#[cw_serde]
pub struct NFTResponse {
    pub nfts: Vec<NFT>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct Class {
    pub id: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub data: Option<Binary>,
}

#[cw_serde]
pub struct ClassResponse {
    pub class: Class,
}

#[cw_serde]
pub struct ClassesResponse {
    pub classes: Vec<Class>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub enum Msg {
    Send {
        class_id: String,
        id: String,
        receiver: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Query {
    #[returns(BalanceResponse)]
    Balance { class_id: String, owner: String },

    #[returns(OwnerResponse)]
    Owner { class_id: String, id: String },

    #[returns(SupplyResponse)]
    Supply { class_id: String },

    #[returns(NFTResponse)]
    NFT { class_id: String, id: String },

    #[returns(NFTsResponse)]
    NFTs {
        class_id: String,
        owner: String,
        pagination: PageRequest,
    },

    #[returns(ClassResponse)]
    Class { class_id: String },

    #[returns(ClassesResponse)]
    Classes { pagination: PageRequest },
}
