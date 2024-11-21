use coreum_std_derive::CosmwasmExt;
/// EventOrderPlaced is emitted when a new order is placed and new sequence is generated for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.EventOrderPlaced")]
pub struct EventOrderPlaced {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// id is unique order ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sequence is unique order sequence.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
/// EventOrderReduced is emitted when the order is reduced during the matching.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.EventOrderReduced")]
pub struct EventOrderReduced {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// id is unique order ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sequence is unique order sequence.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    /// sent_coin is coin sent during matching.
    #[prost(string, tag = "4")]
    pub sent_coin: ::prost::alloc::string::String,
    /// received_coin is coin received during matching.
    #[prost(string, tag = "5")]
    pub received_coin: ::prost::alloc::string::String,
}
/// EventOrderCreated is emitted when the limit order is saved to the order book.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.EventOrderCreated")]
pub struct EventOrderCreated {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// id is unique order ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sequence is unique order sequence.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    /// remaining_quantity is remaining filling quantity sell/buy.
    #[prost(string, tag = "4")]
    pub remaining_quantity: ::prost::alloc::string::String,
    /// remaining_balance is remaining order balance.
    #[prost(string, tag = "5")]
    pub remaining_balance: ::prost::alloc::string::String,
}
/// EventOrderClosed is emitted when the order is closed during matching or manually, and removed from the order book.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.EventOrderClosed")]
pub struct EventOrderClosed {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// id is unique order ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sequence is unique order sequence.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    /// remaining_quantity is remaining filling quantity sell/buy.
    #[prost(string, tag = "4")]
    pub remaining_quantity: ::prost::alloc::string::String,
    /// remaining_balance is remaining order balance.
    #[prost(string, tag = "5")]
    pub remaining_balance: ::prost::alloc::string::String,
}
/// GoodTil is a good til order settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.GoodTil")]
pub struct GoodTil {
    /// good_til_block_height means that order remains active until a specific blockchain block height is reached.
    #[prost(uint64, tag = "1")]
    pub good_til_block_height: u64,
    /// good_til_block_time means that order remains active until a specific blockchain block time is reached.
    #[prost(message, optional, tag = "2")]
    pub good_til_block_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// CancelGoodTil is a cancel good til message for the delay router.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.CancelGoodTil")]
pub struct CancelGoodTil {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// order_sequence is order sequence.
    #[prost(uint64, tag = "2")]
    pub order_sequence: u64,
}
/// Order is a DEX order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.Order")]
pub struct Order {
    /// creator is order creator address.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// type is order type.
    #[prost(enumeration = "OrderType", tag = "2")]
    pub r#type: i32,
    /// id is unique order ID.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// sequence is unique order sequence generated at the time of the order placement.
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
    /// base_denom is base order denom.
    #[prost(string, tag = "5")]
    pub base_denom: ::prost::alloc::string::String,
    /// quote_denom is quote order denom
    #[prost(string, tag = "6")]
    pub quote_denom: ::prost::alloc::string::String,
    /// price is value of one unit of the base_denom expressed in terms of the quote_denom.
    #[prost(string, tag = "7")]
    pub price: ::prost::alloc::string::String,
    /// quantity is amount of the base base_denom being traded.
    #[prost(string, tag = "8")]
    pub quantity: ::prost::alloc::string::String,
    /// side is order side.
    #[prost(enumeration = "Side", tag = "9")]
    pub side: i32,
    /// remaining_quantity is remaining filling quantity sell/buy.
    #[prost(string, tag = "10")]
    pub remaining_quantity: ::prost::alloc::string::String,
    /// remaining_balance is remaining order balance.
    #[prost(string, tag = "11")]
    pub remaining_balance: ::prost::alloc::string::String,
    /// good_til is order good til
    #[prost(message, optional, tag = "12")]
    pub good_til: ::core::option::Option<GoodTil>,
    /// time_in_force is order time in force
    #[prost(enumeration = "TimeInForce", tag = "13")]
    pub time_in_force: i32,
    /// reserve is the reserve required to save the order in the order book
    #[prost(message, optional, tag = "14")]
    pub reserve: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// OrderData is a order data used for the store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.OrderData")]
pub struct OrderData {
    /// order ID provided by the creator.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// order_book_id is order book ID.
    #[prost(uint32, tag = "2")]
    pub order_book_id: u32,
    /// price is value of one unit of the base_denom expressed in terms of the quote_denom.
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// quantity is amount of the base base_denom being traded.
    #[prost(string, tag = "4")]
    pub quantity: ::prost::alloc::string::String,
    /// side is order side.
    #[prost(enumeration = "Side", tag = "5")]
    pub side: i32,
    /// good_til is order good til
    #[prost(message, optional, tag = "6")]
    pub good_til: ::core::option::Option<GoodTil>,
    /// reserve is the reserve required to save the order in the order book
    #[prost(message, optional, tag = "7")]
    pub reserve: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// OrderBookData is a order book data used by order for the store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.OrderBookData")]
pub struct OrderBookData {
    /// base_denom is base order book denom.
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    /// quote_denom is quote order book denom
    #[prost(string, tag = "2")]
    pub quote_denom: ::prost::alloc::string::String,
}
/// OrderBookRecord is a single order book record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.OrderBookRecord")]
pub struct OrderBookRecord {
    /// order_book_id is order book ID.
    #[prost(uint32, tag = "1")]
    pub order_book_id: u32,
    /// side is order side.
    #[prost(enumeration = "Side", tag = "2")]
    pub side: i32,
    /// price is order book record price.
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// order_sequence is order sequence.
    #[prost(uint64, tag = "4")]
    pub order_sequence: u64,
    /// order ID provided by the creator.
    #[prost(string, tag = "5")]
    pub order_id: ::prost::alloc::string::String,
    /// account_number is account number which corresponds the order creator.
    #[prost(uint64, tag = "6")]
    pub account_number: u64,
    /// remaining_quantity is remaining filling quantity sell/buy.
    #[prost(string, tag = "7")]
    pub remaining_quantity: ::prost::alloc::string::String,
    /// remaining_balance is remaining order balance.
    #[prost(string, tag = "8")]
    pub remaining_balance: ::prost::alloc::string::String,
}
/// OrderBookRecordData is a single order book record used for the store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.OrderBookRecordData")]
pub struct OrderBookRecordData {
    /// order ID provided by the creator.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// account_number is account number which corresponds the order creator.
    #[prost(uint64, tag = "2")]
    pub account_number: u64,
    /// remaining_quantity is remaining filling quantity sell/buy.
    #[prost(string, tag = "3")]
    pub remaining_quantity: ::prost::alloc::string::String,
    /// remaining_balance is remaining order balance.
    #[prost(string, tag = "4")]
    pub remaining_balance: ::prost::alloc::string::String,
}
/// Side is order side.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Side {
    /// SIDE_UNSPECIFIED reserves the default value, to protect against unexpected settings.
    Unspecified = 0,
    /// SIDE_BUY means that the order is to buy base_denom quantity with the price.
    Buy = 1,
    /// SIDE_SELL means that the order is to sell base_denom quantity with the price.
    Sell = 2,
}
impl Side {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Side::Unspecified => "SIDE_UNSPECIFIED",
            Side::Buy => "SIDE_BUY",
            Side::Sell => "SIDE_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIDE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIDE_BUY" => Some(Self::Buy),
            "SIDE_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Type is order type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OrderType {
    /// order_type_unspecified reserves the default value, to protect against unexpected settings.
    Unspecified = 0,
    /// order_type_limit means that the order is limit order.
    Limit = 1,
    /// limit order_type_market that the order is market order.
    Market = 2,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_TYPE_LIMIT" => Some(Self::Limit),
            "ORDER_TYPE_MARKET" => Some(Self::Market),
            _ => None,
        }
    }
}
/// TimeInForce is order time in force.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum TimeInForce {
    /// time_in_force_unspecified reserves the default value, to protect against unexpected settings.
    Unspecified = 0,
    /// time_in_force_gtc means that the order remains active until it is fully executed or manually canceled.
    Gtc = 1,
    /// time_in_force_ioc  means that order must be executed immediately, either in full or partially. Any portion of the
    ///   order that cannot be filled immediately is canceled.
    Ioc = 2,
    /// time_in_force_fok means that order must be fully executed or canceled.
    Fok = 3,
}
impl TimeInForce {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TimeInForce::Unspecified => "TIME_IN_FORCE_UNSPECIFIED",
            TimeInForce::Gtc => "TIME_IN_FORCE_GTC",
            TimeInForce::Ioc => "TIME_IN_FORCE_IOC",
            TimeInForce::Fok => "TIME_IN_FORCE_FOK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIME_IN_FORCE_UNSPECIFIED" => Some(Self::Unspecified),
            "TIME_IN_FORCE_GTC" => Some(Self::Gtc),
            "TIME_IN_FORCE_IOC" => Some(Self::Ioc),
            "TIME_IN_FORCE_FOK" => Some(Self::Fok),
            _ => None,
        }
    }
}
/// Params keeps gov manageable parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.Params")]
pub struct Params {
    /// default_unified_ref_amount is the default approximate amount you need to by 1USD, used to for tokens without custom value
    #[prost(string, tag = "1")]
    pub default_unified_ref_amount: ::prost::alloc::string::String,
    /// price_tick_exponent is the exponent used for the price tick calculation
    #[prost(int32, tag = "2")]
    pub price_tick_exponent: i32,
    /// max_orders_per_denom is the maximum number of orders per denom the user can have
    #[prost(uint64, tag = "3")]
    pub max_orders_per_denom: u64,
    /// order_reserve is the reserve required to save the order in the order book
    #[prost(message, optional, tag = "4")]
    pub order_reserve: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// GenesisState defines the module genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub order_books: ::prost::alloc::vec::Vec<OrderBookDataWithId>,
    #[prost(message, repeated, tag = "3")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// order_sequence is current order sequence;
    #[prost(uint64, tag = "4")]
    pub order_sequence: u64,
    #[prost(message, repeated, tag = "5")]
    pub accounts_denoms_orders_counts: ::prost::alloc::vec::Vec<AccountDenomOrdersCount>,
}
/// OrderBookDataWithID is a order book data with it's corresponding ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.OrderBookDataWithID")]
pub struct OrderBookDataWithId {
    /// id is order book ID.
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// data is order book data.
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<OrderBookData>,
}
/// AccountDenomOrderCount is a count of orders per account and denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.AccountDenomOrdersCount")]
pub struct AccountDenomOrdersCount {
    #[prost(uint64, tag = "1")]
    pub account_number: u64,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub orders_count: u64,
}
/// QueryParamsRequest defines the request type for querying x/dex parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryParamsRequest")]
#[proto_query(path = "/coreum.dex.v1.Query/Params", response_type = QueryParamsResponse)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/dex parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryOrderRequest defines the request type for the `Order` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderRequest")]
#[proto_query(path = "/coreum.dex.v1.Query/Order", response_type = QueryOrderResponse)]
pub struct QueryOrderRequest {
    /// creator is order creator's account.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// id is order ID.
    ///
    /// we don't use the gogoproto.customname here since the google.api.http ignores it and generates invalid code.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryOrderRequestResponse defines the response type for the `Order` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderResponse")]
pub struct QueryOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// QueryOrdersRequest defines the request type for the `Orders` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrdersRequest")]
#[proto_query(path = "/coreum.dex.v1.Query/Orders", response_type = QueryOrdersResponse)]
pub struct QueryOrdersRequest {
    /// creator is order creator's account.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryOrdersRequestResponse defines the response type for the `Order` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrdersResponse")]
pub struct QueryOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryOrderBooksRequest defines the request type for the `OrderBooks` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderBooksRequest")]
#[proto_query(
    path = "/coreum.dex.v1.Query/OrderBooks",
    response_type = QueryOrderBooksResponse
)]
pub struct QueryOrderBooksRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryOrderBooksResponse defines the response type for the `OrderBooks` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderBooksResponse")]
pub struct QueryOrderBooksResponse {
    #[prost(message, repeated, tag = "1")]
    pub order_books: ::prost::alloc::vec::Vec<OrderBookData>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryOrderBookOrdersRequest defines the request type for the `OrderBookOrders` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderBookOrdersRequest")]
#[proto_query(
    path = "/coreum.dex.v1.Query/OrderBookOrders",
    response_type = QueryOrderBookOrdersResponse
)]
pub struct QueryOrderBookOrdersRequest {
    /// base_denom is base order denom.
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    /// quote_denom is quote order denom
    #[prost(string, tag = "2")]
    pub quote_denom: ::prost::alloc::string::String,
    /// side is order side.
    #[prost(enumeration = "Side", tag = "3")]
    pub side: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryOrderBookOrdersResponse defines the response type for the `OrderBookOrders` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryOrderBookOrdersResponse")]
pub struct QueryOrderBookOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryAccountDenomOrdersCountRequest defines the request type for the `AccountDenomOrdersCount` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryAccountDenomOrdersCountRequest")]
#[proto_query(
    path = "/coreum.dex.v1.Query/AccountDenomOrdersCount",
    response_type = QueryAccountDenomOrdersCountResponse
)]
pub struct QueryAccountDenomOrdersCountRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryAccountDenomOrdersCountResponse defines the response type for the `AccountDenomOrdersCount` query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.QueryAccountDenomOrdersCountResponse")]
pub struct QueryAccountDenomOrdersCountResponse {
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgPlaceOrder defines message to place an order on orderbook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.MsgPlaceOrder")]
pub struct MsgPlaceOrder {
    /// sender is order creator address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// type is order type.
    #[prost(enumeration = "OrderType", tag = "2")]
    pub r#type: i32,
    /// id is unique order ID.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// base_denom is base order denom.
    #[prost(string, tag = "4")]
    pub base_denom: ::prost::alloc::string::String,
    /// quote_denom is quote order denom
    #[prost(string, tag = "5")]
    pub quote_denom: ::prost::alloc::string::String,
    /// price is value of one unit of the base_denom expressed in terms of the quote_denom.
    #[prost(string, tag = "6")]
    pub price: ::prost::alloc::string::String,
    /// quantity is amount of the base base_denom being traded.
    #[prost(string, tag = "7")]
    pub quantity: ::prost::alloc::string::String,
    /// side is order side.
    #[prost(enumeration = "Side", tag = "8")]
    pub side: i32,
    /// good_til is order good til
    #[prost(message, optional, tag = "9")]
    pub good_til: ::core::option::Option<GoodTil>,
    /// time_in_force is order time in force
    #[prost(enumeration = "TimeInForce", tag = "10")]
    pub time_in_force: i32,
}
/// MsgCancelOrder defines message to cancel the order in the orderbook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.MsgCancelOrder")]
pub struct MsgCancelOrder {
    /// sender is order creator address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// id is unique order ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// MsgCancelOrdersByDenom defines message to cancel all orders by denom and account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.MsgCancelOrdersByDenom")]
pub struct MsgCancelOrdersByDenom {
    /// sender is order creator address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// account is order creator address.
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// denom is orders denom.
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/coreum.dex.v1.EmptyResponse")]
pub struct EmptyResponse {}
