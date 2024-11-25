use coreum_std_derive::CosmwasmExt;
/// MintAuthorization allows the grantee to mint up to mint_limit coin from
/// the granter's account.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MintAuthorization")]
pub struct MintAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub mint_limit:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// BurnAuthorization allows the grantee to burn up to burn_limit coin from
/// the granter's account.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.BurnAuthorization")]
pub struct BurnAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub burn_limit:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
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
    /// amount sent to the token admin account.
    #[prost(string, tag = "5")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub version: u32,
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub extension_cw_address: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub admin: ::prost::alloc::string::String,
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
    /// amount sent to the token admin account.
    #[prost(string, tag = "10")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(uint32, tag = "11")]
    pub version: u32,
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub extension_cw_address: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub admin: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "16")]
    pub dex_settings: ::core::option::Option<DexSettings>,
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
    Copy,
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
    Copy,
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
/// DEXSettings defines the token settings of the dex.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.DEXSettings")]
pub struct DexSettings {
    /// unified_ref_amount is the approximate amount you need to by 1USD, used to define the price tick size
    #[prost(string, tag = "1")]
    pub unified_ref_amount: ::prost::alloc::string::String,
    /// whitelisted_denoms is the list of denoms to trade with.
    #[prost(string, repeated, tag = "2")]
    pub whitelisted_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    BlockSmartContracts = 5,
    Clawback = 6,
    Extension = 7,
    DexBlock = 8,
    DexWhitelistedDenoms = 9,
    DexOrderCancellation = 10,
    DexUnifiedRefAmountChange = 11,
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
            Feature::BlockSmartContracts => "block_smart_contracts",
            Feature::Clawback => "clawback",
            Feature::Extension => "extension",
            Feature::DexBlock => "dex_block",
            Feature::DexWhitelistedDenoms => "dex_whitelisted_denoms",
            Feature::DexOrderCancellation => "dex_order_cancellation",
            Feature::DexUnifiedRefAmountChange => "dex_unified_ref_amount_change",
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
            "block_smart_contracts" => Some(Self::BlockSmartContracts),
            "clawback" => Some(Self::Clawback),
            "extension" => Some(Self::Extension),
            "dex_block" => Some(Self::DexBlock),
            "dex_whitelisted_denoms" => Some(Self::DexWhitelistedDenoms),
            "dex_order_cancellation" => Some(Self::DexOrderCancellation),
            "dex_unified_ref_amount_change" => Some(Self::DexUnifiedRefAmountChange),
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
    #[prost(string, tag = "11")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub admin: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub dex_settings: ::core::option::Option<DexSettings>,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventAmountClawedBack")]
pub struct EventAmountClawedBack {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventDEXLockedAmountChanged")]
pub struct EventDexLockedAmountChanged {
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventDEXExpectedToReceiveAmountChanged")]
pub struct EventDexExpectedToReceiveAmountChanged {
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventAdminTransferred")]
pub struct EventAdminTransferred {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub previous_admin: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub current_admin: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventAdminCleared")]
pub struct EventAdminCleared {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub previous_admin: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.EventDEXSettingsChanged")]
pub struct EventDexSettingsChanged {
    #[prost(message, optional, tag = "1")]
    pub previous_settings: ::core::option::Option<DexSettings>,
    #[prost(message, optional, tag = "2")]
    pub new_settings: ::core::option::Option<DexSettings>,
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
    /// dex_locked_balances contains the DEX locked balances on all of the accounts
    #[prost(message, repeated, tag = "6")]
    pub dex_locked_balances: ::prost::alloc::vec::Vec<Balance>,
    #[prost(message, repeated, tag = "7")]
    pub dex_expected_to_receive_balances: ::prost::alloc::vec::Vec<Balance>,
    #[prost(message, repeated, tag = "8")]
    pub dex_settings: ::prost::alloc::vec::Vec<DexSettingsWithDenom>,
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
    pub version: u32,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.DEXSettingsWithDenom")]
pub struct DexSettingsWithDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub dex_settings: ::core::option::Option<DexSettings>,
}
/// QueryParamsRequest defines the request type for querying x/asset/ft parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
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
    Copy,
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
    /// locked is the balance locked in vesting and DEX.
    #[prost(string, tag = "4")]
    pub locked: ::prost::alloc::string::String,
    /// locked_in_vesting is the balance locked in bank vesting.
    #[prost(string, tag = "5")]
    pub locked_in_vesting: ::prost::alloc::string::String,
    /// locked_in_dex is the balance locked in DEX.
    #[prost(string, tag = "6")]
    pub locked_in_dex: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub expected_to_receive_in_dex: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryDEXSettingsRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/DEXSettings",
    response_type = QueryDexSettingsResponse
)]
pub struct QueryDexSettingsRequest {
    /// denom specifies the denom onto which we query DEX settings
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
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryDEXSettingsResponse")]
pub struct QueryDexSettingsResponse {
    /// dex_settings contains the DEX settings
    #[prost(message, optional, tag = "1")]
    pub dex_settings: ::core::option::Option<DexSettings>,
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
    /// amount sent to the token admin account.
    #[prost(string, tag = "9")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub uri_hash: ::prost::alloc::string::String,
    /// extension_settings must be provided in case wasm extensions are enabled.
    #[prost(message, optional, tag = "12")]
    pub extension_settings: ::core::option::Option<ExtensionIssueSettings>,
    /// dex_settings allowed to be customized by issuer
    #[prost(message, optional, tag = "13")]
    pub dex_settings: ::core::option::Option<DexSettings>,
}
/// ExtensionIssueSettings are settings that will be used to Instantiate the smart contract which contains
/// the source code for the extension.
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
#[proto_message(type_url = "/coreum.asset.ft.v1.ExtensionIssueSettings")]
pub struct ExtensionIssueSettings {
    /// code_id is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "3")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional json encoded data to pass to WASM on instantiation by the ft issuer
    #[prost(bytes = "vec", tag = "4")]
    pub issuance_msg: ::prost::alloc::vec::Vec<u8>,
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
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgSetFrozen")]
pub struct MsgSetFrozen {
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgClawback")]
pub struct MsgClawback {
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgSetWhitelistedLimit")]
pub struct MsgSetWhitelistedLimit {
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgTransferAdmin")]
pub struct MsgTransferAdmin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgClearAdmin")]
pub struct MsgClearAdmin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgUpdateDEXUnifiedRefAmount")]
pub struct MsgUpdateDexUnifiedRefAmount {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// unified_ref_amount is the approximate amount you need to by 1USD, used to define the price tick size
    #[prost(string, tag = "3")]
    pub unified_ref_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/coreum.asset.ft.v1.MsgUpdateDEXWhitelistedDenoms")]
pub struct MsgUpdateDexWhitelistedDenoms {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// whitelisted_denoms is the list of denoms to trade with.
    #[prost(string, repeated, tag = "3")]
    pub whitelisted_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
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
