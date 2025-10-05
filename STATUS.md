# HTTP Client Abstraction - Work Status

## Completed Work

### 1. Cargo.toml Updates ✅
- Added `http-adapter` v0.3.0 as a dependency
- Made `isahc` an optional dependency with feature `http-client-isahc`
- Made `reqwest` an optional dependency with feature `http-client-reqwest`
- Updated feature configuration with `http-client-isahc` and `http-client-reqwest`
- Updated `plex-cli/Cargo.toml` to use `http-client-isahc` feature
- Fixed rust-version from 1.86.0 (invalid) to 1.75.0

### 2. Migration Guide ✅
- Created comprehensive `MIGRATION.md` in `crates/plex-api/`
- Documented breaking changes
- Provided migration examples
- Added troubleshooting section

### 3. Code Refactoring - Partially Complete ⚠️
- Updated `src/error.rs` to use `http-adapter::Body` and remove isahc-specific errors
- Deleted `src/isahc_compat.rs` (no longer needed)
- Updated `src/lib.rs` to remove isahc_compat module
- Started updating `src/http_client.rs` to use http-adapter abstraction
- Updated several files in `src/myplex/` to remove isahc dependencies:
  - `claim_token.rs`
  - `privacy.rs` 
  - `pin.rs`
  - `webhook.rs`

## Remaining Work

### 1. Fix Compilation Errors ⚠️

The following files still need updates to remove isahc-specific imports and usage:

**Priority Files:**
- `src/myplex/mod.rs` - Remove `isahc_compat::StatusCodeExt` and `isahc::AsyncBody` imports
- `src/myplex/sharing/friend.rs` - Remove isahc dependencies
- `src/server/mod.rs` - Remove `isahc_compat` and `isahc::AsyncReadResponseExt`
- `src/server/library.rs` - Remove isahc dependencies  
- `src/server/transcode.rs` - Remove isahc dependencies

**Test Files:**
- `tests/client.rs`
- `tests/fixtures/offline/mod.rs`
- `tests/fixtures/online/mod.rs`
- `tests/server.rs`
- `tests/transcode.rs`

### 2. Fix http-adapter API Usage

The current `src/http_client.rs` implementation needs corrections:
- Verify correct imports from `http-adapter` crate (currently has compilation errors)
- Fix the `Body` type usage
- Implement proper `Request::send()` method that works with http-adapter
- Fix the conditional compilation for `create_default_http_client()`

### 3. Build Environment Issue

Cannot currently test compilation due to missing OpenSSL development files in the build environment. Options:
1. Install `libssl-dev` and `pkg-config` (requires system admin)
2. Use `reqwest` with `rustls-tls` feature (already configured)
3. Add vendored OpenSSL support

### 4. Update Remaining isahc Usage

Pattern to follow for updates:
- Remove `use isahc::AsyncReadResponseExt;`
- Remove `use isahc_compat::StatusCodeExt;`
- Replace `.as_http_status()` with direct status code access (response.status())
- Replace `.text().await?` with body extraction:
  ```rust
  let body_bytes = response.into_body().into_bytes().await?;
  let body = String::from_utf8(body_bytes)?;
  ```
- Replace `.copy_to(writer).await?` with:
  ```rust
  // Stream body to writer using http_adapter::Body methods
  ```

## Testing Plan

Once compilation is fixed:

1. Run `cargo build --features http-client-isahc`
2. Run `cargo build --features http-client-reqwest`  
3. Run `cargo test --features http-client-isahc`
4. Run `cargo clippy --all-targets --features http-client-isahc -- -D warnings`
5. Run `cargo clippy --all-targets --features http-client-reqwest -- -D warnings`

## Commit Message

```
feat!: replace isahc with http-adapter abstraction layer

BREAKING CHANGE: HTTP client is no longer included by default.
Users must explicitly enable either `http-client-isahc` or 
`http-client-reqwest` feature to use the library.

- Add http-adapter v0.3.0 as abstraction layer
- Make isahc optional behind `http-client-isahc` feature  
- Add reqwest support behind `http-client-reqwest` feature
- Remove direct isahc dependency from public API
- Add comprehensive migration guide in MIGRATION.md

Migration: Add feature flag to Cargo.toml:
```toml
plex-api = { version = "0.0.12", features = ["http-client-isahc"] }
```

Resolves: #<issue-number-if-any>
```

## Next Steps

1. Fix http-adapter API usage in `src/http_client.rs`
2. Update remaining files to remove isahc dependencies
3. Resolve build environment OpenSSL issue
4. Test compilation with both features
5. Run tests
6. Run clippy
7. Commit with breaking change message