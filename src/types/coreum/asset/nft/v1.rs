use osmosis_std_derive::CosmwasmExt;

// ClassDefinition defines the non-fungible token class settings to store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.ClassDefinition")]
pub struct ClassDefinition {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(enumeration = "ClassFeature", repeated, tag = "3")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// royalty_rate is a number between 0 and 1,which will be used in coreum native Dex.
    /// whenever an NFT this class is traded on the Dex, the traded amount will be multiplied by this value
    /// that will be transferred to the issuer of the NFT.
    #[prost(string, tag = "4")]
    pub royalty_rate: ::prost::alloc::string::String,
}

/// Class is a full representation of the non-fungible token class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.Class")]
pub struct Class {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub data: ::core::option::Option<crate::shim::Any>,
    #[prost(enumeration = "ClassFeature", repeated, tag = "9")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// royalty_rate is a number between 0 and 1,which will be used in coreum native Dex.
    /// whenever an NFT this class is traded on the Dex, the traded amount will be multiplied by this value
    /// that will be transferred to the issuer of the NFT.
    #[prost(string, tag = "10")]
    pub royalty_rate: ::prost::alloc::string::String,
}
/// ClassFeature defines possible features of non-fungible token class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClassFeature {
    Burning = 0,
    Freezing = 1,
    Whitelisting = 2,
    DisableSending = 3,
}

impl ClassFeature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClassFeature::Burning => "burning",
            ClassFeature::Freezing => "freezing",
            ClassFeature::Whitelisting => "whitelisting",
            ClassFeature::DisableSending => "disable_sending",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "burning" => Some(Self::Burning),
            "freezing" => Some(Self::Freezing),
            "whitelisting" => Some(Self::Whitelisting),
            "disable_sending" => Some(Self::DisableSending),
            _ => None,
        }
    }
}

/// Params store gov manageable parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.Params")]
pub struct Params {
    // mint_fee is the fee burnt each time new NFT is minted
    #[prost(message, optional, tag = "1")]
    pub mint_fee: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}

/// MsgIssueClass defines message for the IssueClass method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgIssueClass")]
pub struct MsgIssueClass {
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub data: ::core::option::Option<crate::shim::Any>,
    #[prost(enumeration = "ClassFeature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "9")]
    pub royalty_rate: ::prost::alloc::string::String,
}

/// MsgMint defines message for the Mint method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<crate::shim::Any>,
}

/// MsgBurn defines message for the Burn method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgBurn")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgFreeze")]
pub struct MsgFreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgUnfreeze")]
pub struct MsgUnfreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgAddToWhitelist")]
pub struct MsgAddToWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgRemoveFromWhitelist")]
pub struct MsgRemoveFromWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EmptyResponse")]
pub struct EmptyResponse {}

// QueryParamsRequest defines the request type for querying x/asset/nft parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryParamsRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}

// QueryParamsResponse defines the response type for querying x/asset/nft parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}

// QueryClassRequest is request type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Class",
    response_type = QueryClassResponse
)]
pub struct QueryClassRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}

// QueryClassResponse is response type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassResponse")]
pub struct QueryClassResponse {
    #[prost(message, optional, tag = "1")]
    pub class: ::core::option::Option<Class>,
}

// QueryClassesRequest is request type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassesRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Classes",
    response_type = QueryClassesResponse
)]
pub struct QueryClassesRequest {
    // pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
}

// QueryClassesResponse is response type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassesResponse")]
pub struct QueryClassesResponse {
    // pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryFrozenRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Frozen",
    response_type = QueryFrozenResponse
)]
pub struct QueryFrozenRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryFrozenResponse")]
pub struct QueryFrozenResponse {
    #[prost(bool, tag = "1")]
    pub frozen: bool,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Whitelisted",
    response_type = QueryWhitelistedResponse
)]
pub struct QueryWhitelistedRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedResponse")]
pub struct QueryWhitelistedResponse {
    #[prost(bool, tag = "1")]
    pub whitelisted: bool,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedAccountsForNFTRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/WhitelistedAccountsForNFT",
    response_type = QueryWhitelistedAccountsForNFTResponse
)]
pub struct QueryWhitelistedAccountsForNFTRequest {
    // pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub class_id: ::prost::alloc::string::String,
}

// QueryClassesResponse is response type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedAccountsForNFTResponse")]
pub struct QueryWhitelistedAccountsForNFTResponse {
    // pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/BurntNFT",
    response_type = QueryBurntNFTResponse
)]
pub struct QueryBurntNFTRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTResponse")]
pub struct QueryBurntNFTResponse {
    #[prost(bool, tag = "1")]
    pub burnt: bool,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTsInClassRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/BurntNFTsInClass",
    response_type = QueryBurntNFTsInClassResponse
)]
pub struct QueryBurntNFTsInClassRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTsInClassResponse")]
pub struct QueryBurntNFTsInClassResponse {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub nft_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
