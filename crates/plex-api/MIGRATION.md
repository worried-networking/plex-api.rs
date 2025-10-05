# Migration Guide

## Version 0.0.12 - HTTP Client Abstraction

### Breaking Changes

The HTTP client implementation has been refactored to use an abstraction layer instead of directly depending on `isahc`. This change allows users to choose between different HTTP client implementations.

### What Changed

1. **HTTP Client Features**: The library no longer includes an HTTP client by default. You must now explicitly choose an HTTP client implementation by enabling one of the following features:
   - `http-client-isahc`: Use the `isahc` HTTP client (previously the default)
   - `http-client-reqwest`: Use the `reqwest` HTTP client

2. **Dependency Updates**: 
   - `isahc` is no longer a direct dependency
   - `reqwest` is no longer a direct dependency
   - `http-adapter` v0.3.0 is used as the abstraction layer
   - `http-adapter-isahc` v0.3.0 provides isahc support
   - `http-adapter-reqwest` v0.3.0 provides reqwest support

### How to Migrate

#### For users who were using the library without explicit HTTP client configuration:

**Before:**
```toml
[dependencies]
plex-api = "0.0.11"
```

**After:**
```toml
[dependencies]
plex-api = { version = "0.0.12", features = ["http-client-isahc"] }
```

#### For users who want to switch to `reqwest`:

```toml
[dependencies]
plex-api = { version = "0.0.12", features = ["http-client-reqwest"] }
```

### Code Changes

The public API remains largely unchanged. The `HttpClient` and `HttpClientBuilder` continue to work the same way from a user perspective.

**Before:**
```rust
use plex_api::{HttpClientBuilder, MyPlexBuilder};

let client = HttpClientBuilder::default()
    .set_x_plex_client_identifier("unique-client-id")
    .build()?;

let myplex = MyPlexBuilder::default()
    .set_client(client)
    .build()?;
```

**After:**
```rust
// No changes needed! The same code works with either http-client-isahc or http-client-reqwest feature
use plex_api::{HttpClientBuilder, MyPlexBuilder};

let client = HttpClientBuilder::default()
    .set_x_plex_client_identifier("unique-client-id")
    .build()?;

let myplex = MyPlexBuilder::default()
    .set_client(client)
    .build()?;
```

### Advanced: Custom HTTP Client

If you want to use a custom HTTP client implementation, you can provide your own implementation of the `http_adapter::HttpClient` trait:

```rust
use plex_api::HttpClientBuilder;
use http_adapter::HttpClient as AdapterHttpClient;

// Assuming you have a custom client that implements http_adapter::HttpClient
let custom_client: Box<dyn AdapterHttpClient> = Box::new(MyCustomClient::new());

let client = HttpClientBuilder::default()
    .set_http_client(custom_client)
    .build()?;
```

### Why This Change?

This change provides several benefits:

1. **Flexibility**: Users can now choose the HTTP client that best fits their needs and existing dependencies
2. **Reduced Dependencies**: Projects can avoid unnecessary dependencies by selecting only the HTTP client they need
3. **Future-Proofing**: The abstraction layer makes it easier to add support for additional HTTP clients in the future
4. **Better Testing**: The abstraction makes it easier to mock HTTP clients for testing purposes

### Troubleshooting

#### Error: "At least one HTTP client feature must be enabled"

This error occurs when you try to build the library without enabling any HTTP client feature. Make sure to add either `http-client-isahc` or `http-client-reqwest` to your feature list.

#### Build Errors Related to OpenSSL

If you're using the `http-client-isahc` feature and encounter OpenSSL-related build errors:

1. Install OpenSSL development packages:
   - On Ubuntu/Debian: `sudo apt-get install libssl-dev pkg-config`
   - On Fedora/RHEL: `sudo dnf install openssl-devel`
   - On macOS: `brew install openssl`

2. Alternatively, switch to `http-client-reqwest` with rustls which doesn't require OpenSSL

#### Performance Considerations

Both `isahc` and `reqwest` are high-performance HTTP clients. The choice between them should be based on your specific requirements:

- `isahc`: Based on libcurl, good for compatibility and stability
- `reqwest`: Pure Rust implementation with `rustls` option (no OpenSSL dependency)

### Getting Help

If you encounter issues during migration, please:

1. Check the [examples](examples/) directory for updated examples
2. Review the [API documentation](https://docs.rs/plex-api)
3. Open an issue on [GitHub](https://github.com/andrey-yantsen/plex-api.rs/issues) if you need assistance