# Coreum WASM SDK

[![coreum-wasm-sdk on crates.io](https://img.shields.io/crates/v/coreum-wasm-sdk.svg)](https://crates.io/crates/coreum-wasm-sdk) [![Docs](https://docs.rs/coreum-wasm-sdk/badge.svg)](https://docs.rs/coreum-wasm-sdk)

Coreum WASM SDK contains Rust data types to be used in WASM smart contracts interacting with the Coreum blockchain.
They allow developers to issue and execute on-chain messages managing Smart Tokens.

## Proto types

The proto types are generated using [coreum-rust-protobuf](https://github.com/CoreumFoundation/coreum-rust-protobuf) according to our current version of the chain to make them compatible with tools like our [coreum-test-tube](https://github.com/CoreumFoundation/test-tube) or to interact with our chain using gRPC and Rust, they should not be manually modified. Instructions on how to use the tool are in [coreum-rust-protobuf](https://github.com/CoreumFoundation/coreum-rust-protobuf) and can be used to generate any Rust protobuf types for any version of our chain.

## Useful links

- [Coreum Website](https://coreum.com)
- [Coreum Documentation](https://docs.coreum.dev)
- [Smart contract examples](https://github.com/CoreumFoundation/coreum/tree/master/integration-tests/contracts)
