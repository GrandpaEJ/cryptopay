# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-30

### Added
- Initial release of Etherscan payment gateway library
- Full Etherscan API client with rate limiting (5 req/s default)
- API key rotation support for multiple keys
- In-memory LRU cache with configurable TTL
- Account endpoints (balance, transactions, internal transactions)
- Transaction endpoints (get transaction, receipt, confirmations)
- Token endpoints (ERC20 transfers, balances)
- Gas tracker endpoints (oracle, price estimation)
- Payment request models supporting ETH and ERC20 tokens
- Payment verification logic with amount matching
- Payment status tracking (Pending, Detected, Confirmed, Failed, Expired)
- Payment monitor with background polling and callbacks
- Utility functions for amount conversions (wei/ether/gwei)
- Address and transaction hash validation
- Configuration system with builder pattern
- Environment variable support for configuration
- Comprehensive error handling with custom error types
- Three working examples (basic payment, token payment, payment monitor)
- Unit tests for all core functionality
- Complete README with usage examples and API documentation

### Supported
- Ethereum mainnet and Sepolia testnet
- Native ETH payments
- ERC20 token payments (USDT, USDC, DAI, and custom tokens)
- Confirmation tracking with configurable thresholds
- Payment timeouts with expiration handling

## [Unreleased]

### Planned
- PostgreSQL storage implementation
- SQLite storage implementation
- Redis-backed distributed cache
- Webhook delivery system
- Multi-chain support (Polygon, Arbitrum, etc.)

[0.1.0]: https://github.com/yourusername/cryptopay/releases/tag/v0.1.0
