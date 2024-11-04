# Resolvers

This directory contains custom resolvers for fetching Honkai: Star Rail redemption codes from various sources.

## Overview

Each resolver is responsible for:
- Fetching HTML/JSON content from a specific website
- Parsing the content to extract valid redemption codes and their rewards
- Returning standardized `RedemptionCode` objects

## Structure

When adding a resolver for a new service, create a new directory here named after the service (e.g., `eurogamer`, `hoyolab`). Each resolver directory should contain:

- `mod.rs`: Main implementation file containing the resolver struct and its implementation
- `tests.rs`: Unit tests for the resolver

## Adding a New Resolver

1. Create a new directory for your service
2. Implement the `CodeResolver` trait in your `mod.rs`:
   ```rust
   #[async_trait]
   impl CodeResolver for YourResolver {
       fn name(&self) -> String;
       fn base_url(&self) -> String;
       async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>>;
   }
   ```
3. Add tests in `tests.rs`
4. Register your resolver in `src/resolvers/mod.rs`

## Current Resolvers

- **Eurogamer**: Parses codes from Eurogamer's HSR guides
- **Fandom**: Extracts codes from the HSR Fandom wiki
- **Game8**: Fetches codes from Game8's HSR database
- **Hoyolab**: Official source, fetches codes from Hoyolab API
- **Polygon**: Parses codes from Polygon's HSR guides
- **Prydwen**: Extracts codes from Prydwen.gg

## Common Patterns

Each resolver typically:
1. Makes an HTTP request to fetch the source content
2. Parses the content using appropriate tools (e.g., `scraper` for HTML)
3. Extracts codes and rewards into the standard format
4. Performs validation on the extracted codes
5. Returns a vector of valid `RedemptionCode` objects

## Testing

Each resolver includes tests to verify:
- Successful fetching from live sites
- Proper parsing of HTML/JSON content
- Code validation and formatting
- Error handling

Run tests with:
```bash
cargo test --package starrail-api --lib resolvers
```

To run tests for a specific resolver:
```bash
cargo test --package starrail-api --lib resolvers::{resolver_name}
```

For example:
```bash
cargo test --package starrail-api --lib resolvers::hoyolab
