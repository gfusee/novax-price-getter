# Novax xExchange Price Getter Example

Welcome to a hands-on demonstration of how to interact with contracts using the [Novax library](https://github.com/gfusee/novax). This repository not only showcases the potential of Novax but also provides a practical solution to fetch token prices on xExchange.

## Key Features

1. **Token Price Retrieval**: Extract the price of any fungible token listed on xExchange, provided it has a trading pair against WEGLD.
2. **Dual-Use**: This project can be utilized both as a crate for integration into other Rust projects or as a standalone binary for direct execution.
3. **Mock Blockchain for Testing**: Discover how Novax can be employed to craft integration tests by simulating a blockchain environment, ensuring that your applications run as expected.


## Integration Tests with Mock Blockchain

Novax provides an elegant way to mock the blockchain for integration testing. Delve into the `tests/` directory to see how this is achieved and how you can emulate a blockchain environment to validate your contract interactions.