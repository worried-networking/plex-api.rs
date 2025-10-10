# Changelog

## [0.0.4](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-cli-v0.0.3...plex-cli-v0.0.4) (2025-10-10)


### Features

* **wait:** add error details ([b7cd005](https://github.com/andrey-yantsen/plex-api.rs/commit/b7cd005b6fe4cf47235b45dc8056e8a9c2b94d1f))


### Miscellaneous

* **deps:** update indicatif requirement from ^0.17.3 to ^0.18.0 ([ecd27bf](https://github.com/andrey-yantsen/plex-api.rs/commit/ecd27bf10a4a964c47ee56f9782ca2c098558c1d))

## [0.0.3](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-cli-v0.0.2...plex-cli-v0.0.3) (2023-05-19)


### ⚠ BREAKING CHANGES

* make rating key to be string
* deny unreachable_pub and rename some exports

### Features

* add basic media downloading to plex-cli ([6b12426](https://github.com/andrey-yantsen/plex-api.rs/commit/6b124262b2212eeac262f3af10734b48bc3ad663))
* **wait:** add an argument for extended waiting ([19838c4](https://github.com/andrey-yantsen/plex-api.rs/commit/19838c4b8a5723c08bb9d8b3bbc7e53eb7594971))
* **wait:** add verbose option to display failure details ([343eb17](https://github.com/andrey-yantsen/plex-api.rs/commit/343eb179fc7f151f9cf3cc2ce21a9d337cfcfda7))
* **wait:** do not wait for certificate for unclaimed servers ([9e22783](https://github.com/andrey-yantsen/plex-api.rs/commit/9e227832a04e0e0ef2ca4b40d06f6e644b8da1dc))
* **wait:** fix and add more checks for --full waiting ([4622166](https://github.com/andrey-yantsen/plex-api.rs/commit/46221667aa314bfc8e86424a9a0d45b4d497d66d))
* **wait:** remove waiting for the certificate in full mode ([244b6a5](https://github.com/andrey-yantsen/plex-api.rs/commit/244b6a5be9e066239702249d9207095ed88923b5))
* **wait:** wait for the server to map with MyPlex in full mode ([9c72e26](https://github.com/andrey-yantsen/plex-api.rs/commit/9c72e26b3a87ec19ac9e6ef1160e03ea42224911))


### Documentation

* **wait:** document default timeout ([6d6da65](https://github.com/andrey-yantsen/plex-api.rs/commit/6d6da65a9c34409441d72c2ca4a07d3788c3e77a))


### Code Refactoring

* deny unreachable_pub and rename some exports ([04ecbc0](https://github.com/andrey-yantsen/plex-api.rs/commit/04ecbc08d1c0cb9e9f2e4be26d65900665855f16)), closes [#509](https://github.com/andrey-yantsen/plex-api.rs/issues/509)
* make rating key to be string ([1748233](https://github.com/andrey-yantsen/plex-api.rs/commit/174823354bf2c6704ced25c3ebf8f99d2c501d8d))


### Miscellaneous

* **deps:** update plex-api to 0.0.10 ([faaa8f5](https://github.com/andrey-yantsen/plex-api.rs/commit/faaa8f57396b93a800f5a12125e9b88180faf204))
* **deps:** update plex-api to 0.0.6 ([086afba](https://github.com/andrey-yantsen/plex-api.rs/commit/086afba31830640616234ea049f53c3b34f31ff4))
* **deps:** use looser limits for plex-api dependency inside plex-cli ([f46e056](https://github.com/andrey-yantsen/plex-api.rs/commit/f46e056b87cce39229859c3a0f348b3e17df8269))

## 0.0.2 (2023-01-06)


### Features

* add `wait` command for plex-cli ([89332b5](https://github.com/andrey-yantsen/plex-api.rs/commit/89332b576f2b43d3d2a1c7dc56513a6a40ec3d81))
* add subcommand for controling settings ([dfdef7a](https://github.com/andrey-yantsen/plex-api.rs/commit/dfdef7a7e4b7e543b88ee0461e45b40d5d4d23c1))


### Bug Fixes

* **plex-cli/preferences:** fix typo in the argument name ([787df7a](https://github.com/andrey-yantsen/plex-api.rs/commit/787df7ab16e51bb4285a660d1ba9c70ff62c3324))


### Miscellaneous

* change MSRV to 1.56 and edition to 2021 ([7cb77b0](https://github.com/andrey-yantsen/plex-api.rs/commit/7cb77b00befcc5265c81e76e74bc8e157a2f0ff5))
* **deps:** adapt the code for the new xflags ([c980ae8](https://github.com/andrey-yantsen/plex-api.rs/commit/c980ae86e99caf613911a64b03668e5982435c60))
* **deps:** bump tokio from 1.22.0 to 1.23.0 ([1f06ffd](https://github.com/andrey-yantsen/plex-api.rs/commit/1f06ffd6c5b0ac0f1dcd0201a4ad2383eda50d35))
* **deps:** bump xflags from 0.2.4 to 0.3.0 ([d700794](https://github.com/andrey-yantsen/plex-api.rs/commit/d700794ada5bf69a3890cc938c84a8d36d4547f9))
* plex-cli 0.0.1 ([5e163d7](https://github.com/andrey-yantsen/plex-api.rs/commit/5e163d741f9ba304658317b2f8d42679af87888d))
