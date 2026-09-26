[![crates](https://img.shields.io/crates/v/plex-api.svg)](https://crates.io/crates/plex-api)
[![tests status](https://github.com/andrey-yantsen/plex-api.rs/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/andrey-yantsen/plex-api.rs/actions/workflows/test.yml?query=branch%3Amain)
[![codecov](https://codecov.io/gh/andrey-yantsen/plex-api.rs/branch/main/graph/badge.svg?token=hrpICjrn5q)](https://codecov.io/gh/andrey-yantsen/plex-api.rs)
[![Join the chat at https://gitter.im/plex-api-rs/community](https://badges.gitter.im/plex-api-rs/community.svg)](https://gitter.im/plex-api-rs/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

# Work in progress

At the moment library provides some basic functionality and can be used with
caution. Breaking changes are expected with every release at this stage.

Huge thanks to @Mossop for all his contributions.

My goal (not sure, when I would be able to achieve it, or even if I'll achieve it) is to create an API, similar to
[python-plexapi](https://github.com/pkkid/python-plexapi). Actually, my final goal is to rewrite my
[plexiglas](https://github.com/andrey-yantsen/plexiglas) project into Rust from Python :) And to "simplify" the task
I've decided to have a separate project with the API.

Any help is welcome. And just in case: I'm far from being an expert in Rust, so if you know how to do things in proper,
Rust-way — feel free to create an issue (or, better, pull-request) to correct the code.

# Library status

* [x] MyPlex access
  * [x] Authentication using Token, Login+Password or Login+Password+OTP
  * [x] Listing known devices and connecting to them via MyPlex object
  * [x] Requesting Claim Token
  * [x] Changing privacy settings
  * [x] Controling webhooks
  * [x] [Link Codes](https://support.plex.tv/articles/203395277-connect-app-to-your-plex-account/) support
  * [x] Sharing: accepting/rejecting friendship, inviting somebody, sharing a server
  * [x] Working with Plex announcements
  * [x] Switching to another Plex Home user
  * [ ] Watchlist
  * [ ] Creating/changing managed users
  * [ ] ???
* [ ] Player access
  * [x] Connecting to the player (directly or via server)
  * [ ] Doing anything useful
* [x] Basic server access
  * [x] Navigating the libraries
  * [x] Downloading media
  * [x] Transcoding
  * [x] Managing the server preferences
* [ ] Advanced server access
  * [ ] Managing the libraries
  * [ ] Changing items' metadata
  * [ ] Changing items' preferences (e.g. metadata language)
  * [ ] Listening for the server's events/alers
  * [ ] Server stats reading
  * [ ] Managing optimized versions of media
  * [ ] ???

# Supported plex versions

The codebase is regularly tested against three different PMS versions. The
exact list of the releases is generated automatically on a daily basis, to keep
up with all the recent PMS versions. We take every other minor release from
the latest 5 minor releases. At the moment the following versions are
tested:

<!-- plex releases list start -->
* 1.43.4.10903-e5521bd8c
* 1.41.6.9685-d301f511a
* 1.32.8.7639-fb6452ebf
<!-- plex releases list end -->

# License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

You can find the contribution documentation in the [CONTRIBUTING.md](./CONTRIBUTING.md) file.
