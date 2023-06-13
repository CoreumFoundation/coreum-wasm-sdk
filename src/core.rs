use crate::{assetft, assetnft, nft};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, CustomQuery};

#[cw_serde]
pub enum CoreumMsg {
    AssetFT(assetft::Msg),
    AssetNFT(assetnft::Msg),
    NFT(nft::Msg),
}

impl From<CoreumMsg> for CosmosMsg<CoreumMsg> {
    fn from(val: CoreumMsg) -> Self {
        CosmosMsg::Custom(val)
    }
}

impl CustomMsg for CoreumMsg {}

#[cw_serde]
pub enum CoreumQueries {
    AssetFT(assetft::Query),
    AssetNFT(assetnft::Query),
    NFT(nft::Query),
}

impl CustomQuery for CoreumQueries {}
