use osmosis_std_derive::CosmwasmExt;
/// Definition defines the fungible token settings to store.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.Definition")]
pub struct Definition {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(enumeration = "Feature", repeated, tag = "3")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// burn_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// burn_amount. This value will be burnt on top of the send amount.
    #[prost(string, tag = "4")]
    pub burn_rate: ::prost::alloc::string::String,
    /// send_commission_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// amount sent to the token issuer account.
    #[prost(string, tag = "5")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u32,
}
/// Token is a full representation of the fungible token.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.Token")]
pub struct Token {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub subunit: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub precision: u32,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub globally_frozen: bool,
    #[prost(enumeration = "Feature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// burn_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// burn_amount. This value will be burnt on top of the send amount.
    #[prost(string, tag = "9")]
    pub burn_rate: ::prost::alloc::string::String,
    /// send_commission_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// amount sent to the token issuer account.
    #[prost(string, tag = "10")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(uint32, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u32,
}
/// DelayedTokenUpgradeV1 is executed by the delay module when it's time to enable IBC.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.DelayedTokenUpgradeV1")]
pub struct DelayedTokenUpgradeV1 {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// TokenUpgradeV1Status defines the current status of the v1 token migration.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.TokenUpgradeV1Status")]
pub struct TokenUpgradeV1Status {
    #[prost(bool, tag = "1")]
    pub ibc_enabled: bool,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// TokenUpgradeStatuses defines all statuses of the token migrations.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.TokenUpgradeStatuses")]
pub struct TokenUpgradeStatuses {
    #[prost(message, optional, tag = "1")]
    pub v1: ::core::option::Option<TokenUpgradeV1Status>,
}
/// Feature defines possible features of fungible token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Feature {
    Minting = 0,
    Burning = 1,
    Freezing = 2,
    Whitelisting = 3,
    Ibc = 4,
}
impl Feature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Feature::Minting => "minting",
            Feature::Burning => "burning",
            Feature::Freezing => "freezing",
            Feature::Whitelisting => "whitelisting",
            Feature::Ibc => "ibc",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "minting" => Some(Self::Minting),
            "burning" => Some(Self::Burning),
            "freezing" => Some(Self::Freezing),
            "whitelisting" => Some(Self::Whitelisting),
            "ibc" => Some(Self::Ibc),
            _ => None,
        }
    }
}
/// EventIssued is emitted on MsgIssue.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventIssued")]
pub struct EventIssued {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub subunit: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub precision: u32,
    #[prost(string, tag = "6")]
    pub initial_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "Feature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "9")]
    pub burn_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub send_commission_rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventFrozenAmountChanged")]
pub struct EventFrozenAmountChanged {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub previous_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub current_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventWhitelistedAmountChanged")]
pub struct EventWhitelistedAmountChanged {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub previous_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub current_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.Params")]
pub struct Params {
    /// issue_fee is the fee burnt each time new token is issued.
    #[prost(message, optional, tag = "1")]
    pub issue_fee: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// token_upgrade_decision_timeout defines the end of the decision period for upgrading the token.
    #[prost(message, optional, tag = "2")]
    pub token_upgrade_decision_timeout: ::core::option::Option<crate::shim::Timestamp>,
    /// token_upgrade_grace_period the period after which the token upgrade is executed effectively.
    #[prost(message, optional, tag = "3")]
    pub token_upgrade_grace_period: ::core::option::Option<crate::shim::Duration>,
}
/// GenesisState defines the module genesis state.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// tokens keep the fungible token state
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
    /// frozen_balances contains the frozen balances on all of the accounts
    #[prost(message, repeated, tag = "3")]
    pub frozen_balances: ::prost::alloc::vec::Vec<Balance>,
    /// whitelisted_balances contains the whitelisted balances on all of the accounts
    #[prost(message, repeated, tag = "4")]
    pub whitelisted_balances: ::prost::alloc::vec::Vec<Balance>,
    /// pending_token_upgrades contains pending token upgrades.
    #[prost(message, repeated, tag = "5")]
    pub pending_token_upgrades: ::prost::alloc::vec::Vec<PendingTokenUpgrade>,
}
/// Balance defines an account address and balance pair used module genesis genesis state.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.Balance")]
pub struct Balance {
    /// address is the address of the balance holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// coins defines the different coins this balance holds.
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// PendingTokenUpgrade stores the version of pending token upgrade.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.PendingTokenUpgrade")]
pub struct PendingTokenUpgrade {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u32,
}
/// QueryParamsRequest defines the request type for querying x/asset/ft parameters.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryParamsRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/asset/ft parameters.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/Token",
    response_type = QueryTokenResponse
)]
pub struct QueryTokenRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenResponse")]
pub struct QueryTokenResponse {
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<Token>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenUpgradeStatusesRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/TokenUpgradeStatuses",
    response_type = QueryTokenUpgradeStatusesResponse
)]
pub struct QueryTokenUpgradeStatusesRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenUpgradeStatusesResponse")]
pub struct QueryTokenUpgradeStatusesResponse {
    #[prost(message, optional, tag = "1")]
    pub statuses: ::core::option::Option<TokenUpgradeStatuses>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokensRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/Tokens",
    response_type = QueryTokensResponse
)]
pub struct QueryTokensRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokensResponse")]
pub struct QueryTokensResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryBalanceRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// account specifies the account onto which we query balances
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// denom specifies balances on a specific denom
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// balance contains the balance with the queried account and denom
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
    /// whitelisted is the whitelisted amount of the denom on the account.
    #[prost(string, tag = "2")]
    pub whitelisted: ::prost::alloc::string::String,
    /// frozen is the frozen amount of the denom on the account.
    #[prost(string, tag = "3")]
    pub frozen: ::prost::alloc::string::String,
    /// locked is the balance locked by vesting.
    #[prost(string, tag = "4")]
    pub locked: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryFrozenBalancesRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/FrozenBalances",
    response_type = QueryFrozenBalancesResponse
)]
pub struct QueryFrozenBalancesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// account specifies the account onto which we query frozen balances
    #[prost(string, tag = "2")]
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryFrozenBalancesResponse")]
pub struct QueryFrozenBalancesResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// balances contains the frozen balances on the queried account
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryFrozenBalanceRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/FrozenBalance",
    response_type = QueryFrozenBalanceResponse
)]
pub struct QueryFrozenBalanceRequest {
    /// account specifies the account onto which we query frozen balances
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// denom specifies frozen balances on a specific denom
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryFrozenBalanceResponse")]
pub struct QueryFrozenBalanceResponse {
    /// balance contains the frozen balance with the queried account and denom
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryWhitelistedBalancesRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/WhitelistedBalances",
    response_type = QueryWhitelistedBalancesResponse
)]
pub struct QueryWhitelistedBalancesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// account specifies the account onto which we query whitelisted balances
    #[prost(string, tag = "2")]
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryWhitelistedBalancesResponse")]
pub struct QueryWhitelistedBalancesResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// balances contains the whitelisted balances on the queried account
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryWhitelistedBalanceRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/WhitelistedBalance",
    response_type = QueryWhitelistedBalanceResponse
)]
pub struct QueryWhitelistedBalanceRequest {
    /// account specifies the account onto which we query whitelisted balances
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// denom specifies whitelisted balances on a specific denom
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryWhitelistedBalanceResponse")]
pub struct QueryWhitelistedBalanceResponse {
    /// balance contains the whitelisted balance with the queried account and denom
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgIssue defines message to issue new fungible token.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgIssue")]
pub struct MsgIssue {
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub subunit: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub precision: u32,
    #[prost(string, tag = "5")]
    pub initial_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "Feature", repeated, tag = "7")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// burn_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// burn_amount. This value will be burnt on top of the send amount.
    #[prost(string, tag = "8")]
    pub burn_rate: ::prost::alloc::string::String,
    /// send_commission_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// amount sent to the token issuer account.
    #[prost(string, tag = "9")]
    pub send_commission_rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub coin: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgBurn")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub coin: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgFreeze")]
pub struct MsgFreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub coin: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgUnfreeze")]
pub struct MsgUnfreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub coin: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgGloballyFreeze")]
pub struct MsgGloballyFreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgGloballyUnfreeze")]
pub struct MsgGloballyUnfreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgSetWhitelistedLimit")]
pub struct MsgSetWhitelistedLimit {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub coin: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgUpgradeTokenV1 is the message upgrading token to V1.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgUpgradeTokenV1")]
pub struct MsgUpgradeTokenV1 {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub ibc_enabled: bool,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgUpdateParams")]
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EmptyResponse")]
pub struct EmptyResponse {}
