# Changelog

All notable changes to rusty-bybit will be documented in this file.

## [Unreleased] - 2026-01-16

### Breaking Changes
- **ServerTime**: Changed field types from `i64` to `String` to match Bybit v5 API response format
  - `time_second` renamed to `timeSecond` (now returns string from API)
  - Added `timeNano` (string) field
  - Impact: Any code accessing these fields will need to handle string parsing
- **WalletBalance**: Restructured to match Bybit v5 API response format
  - Wrapped in `WalletBalance { list: Vec<AccountBalance> }`
  - Added `AccountBalance` struct with additional fields: `total_margin_balance`, `total_available_balance`, etc.
  - Impact: Balance access pattern changes from `balance.total_equity` to `balance.list[0].total_equity`
- **Ticker response**: Changed from `Vec<Ticker>` to `TickerList { list: Vec<Ticker>, nextPageCursor: Option<String> }`
  - Impact: Access pattern changes from `tickers` to `tickers.list`
- **InstrumentInfo response**: Changed from `Vec<InstrumentInfo>` to `InstrumentList { list: Vec<InstrumentInfo>, nextPageCursor: Option<String> }`
  - Impact: Access pattern changes from `instruments` to `instruments.list`
- **Enums**: Converted string-based fields to strong-typed enums
  - `side` now uses `Side::Buy | Side::Sell` instead of `"Buy" | "Sell"`
  - `order_type` now uses `OrderType::Market | OrderType::Limit` instead of `"Market" | "Limit"`
  - `time_in_force` now uses `TimeInForce::*` enum variants
  - `status` now uses `OrderStatus::*` enum variants
  - Impact: Enum construction and pattern matching syntax changes

### Added
- Crate-level documentation with quick start guide
- Module documentation for all modules
- Comprehensive struct and function documentation
- Builder pattern for `CreateOrderRequest` for fluent API
- Enhanced error types: `RateLimitExceeded`, `AuthenticationError`, `InvalidParameter`, `TimestampError`, `InvalidEnumValue`
- API error code mappings from Bybit v5 API
- Comprehensive examples in `examples/` directory
- README.md with installation and usage guide

### Fixed
- Corrected ServerTime response structure to match Bybit v5 API
- Fixed WalletBalance response structure to match Bybit v5 API
- Fixed Ticker and InstrumentInfo response structures (wrapper objects)
- Corrected authentication signature generation (matches Bybit spec)
- Added missing optional fields to `CreateOrderRequest` and `Order` types

### Changed
- Updated `get_tickers()` return type from `Vec<Ticker>` to `TickerList`
- Updated `get_instruments()` return type from `Vec<InstrumentInfo>` to `InstrumentList`
- Improved error messages with more context
- Made all numeric fields in response structs use `String` type (as per Bybit v5 API)
- Enhanced type safety throughout the codebase

## [0.1.0] - Initial Release
- Initial implementation of Bybit v5 API SDK
- Basic market data endpoints
- Basic order management endpoints
- Basic account and position endpoints
- HMAC-SHA256 authentication
