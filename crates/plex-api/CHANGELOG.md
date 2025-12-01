# Changelog

## [0.0.13](https://github.com/worried-networking/plex-api.rs/compare/plex-api-v0.0.12...plex-api-v0.0.13) (2025-12-01)


### Miscellaneous

* **deps:** update dash-mpd requirement from ^0.18.4 to ^0.19.0 ([a912df2](https://github.com/worried-networking/plex-api.rs/commit/a912df291fd1ec71dc844e9ad4764f419f83bcbf))
* **deps:** update dash-mpd requirement from ^0.18.4 to ^0.19.0 ([fdf35e8](https://github.com/worried-networking/plex-api.rs/commit/fdf35e873029528ede21bde02ca4166218b0511b))
* update Feature enum ([0a1cdd7](https://github.com/worried-networking/plex-api.rs/commit/0a1cdd7345571ad1c6c6e2ce5190f2361567cd50))
* update Feature enum ([e91c339](https://github.com/worried-networking/plex-api.rs/commit/e91c3399ee62a4ce79a8117ea4097796cb13287c))
* update Feature enum ([76ae0c1](https://github.com/worried-networking/plex-api.rs/commit/76ae0c18ed5aa264ebe2f2cae3ca2c932a5f713f))
* update Feature enum ([8288ce3](https://github.com/worried-networking/plex-api.rs/commit/8288ce3ba158a59363e8a049c5c777c1f66cbdaf))
* update Feature enum ([971f1ae](https://github.com/worried-networking/plex-api.rs/commit/971f1ae647c45eeeebcac8acf464b2d486a4845c))

## [0.0.12](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.11...plex-api-v0.0.12) (2025-10-10)


### ⚠ BREAKING CHANGES

* add a new `content` field to Metadata
* bump MSRV to 1.86.0
* add a few new fields to the Part struct
* bump msrv to 1.70.0

### build

* bump MSRV to 1.86.0 ([021a827](https://github.com/andrey-yantsen/plex-api.rs/commit/021a8272642c2d45d61be566e16e870a277df488))


### Features

* add a new `content` field to Metadata ([34e217d](https://github.com/andrey-yantsen/plex-api.rs/commit/34e217dad7a6a970daa7a3e870e4830445971520))
* add ApiVersion property to Server ([9516d54](https://github.com/andrey-yantsen/plex-api.rs/commit/9516d541c5d597a8439886b1621ecc8b3cc914ab))
* Allow transcoding entire media with server media selection ([dd6d0e5](https://github.com/andrey-yantsen/plex-api.rs/commit/dd6d0e5731b99a7d44fdcd74bec057ed053d3e41))
* Implement DownloadQueue ([3025cb4](https://github.com/andrey-yantsen/plex-api.rs/commit/3025cb45646edb9021f5276ab31749abad3e69bd))


### Bug Fixes

* add `async_std::test` to async tests ([efba3f5](https://github.com/andrey-yantsen/plex-api.rs/commit/efba3f5198cb439662a749816dd23b5a14ca5de3))
* Add missing attributes ([20a0438](https://github.com/andrey-yantsen/plex-api.rs/commit/20a043856c66efeb3bc633d4c88f9e509b4f1c16))
* Fixes test failures by switching to tokio runtime for tests ([449c072](https://github.com/andrey-yantsen/plex-api.rs/commit/449c072de497387243dc0f688064a786149e6bf4))
* make photo_auto_tag field on the server optional ([345effe](https://github.com/andrey-yantsen/plex-api.rs/commit/345effef0cb0f228cc50f1d327a05a824a0aa0b4))
* migrate to the new sysinfo version ([ff70260](https://github.com/andrey-yantsen/plex-api.rs/commit/ff70260e5ee6623ce9ba369661293f548ec685db))
* Safer deserialisation for skip_parent and skip_count fields ([039db5a](https://github.com/andrey-yantsen/plex-api.rs/commit/039db5a728112211783d088ca9950f844b7099eb))


### Miscellaneous

* add a few new fields to the Part struct ([032372d](https://github.com/andrey-yantsen/plex-api.rs/commit/032372d3f05e1c03e5743d805341a38932bea2f7))
* bump msrv to 1.70.0 ([fc7b85f](https://github.com/andrey-yantsen/plex-api.rs/commit/fc7b85f93014a48b1fd2c47ec3cab61cfd49359c))
* **clippy:** fix a new warning ([9561575](https://github.com/andrey-yantsen/plex-api.rs/commit/9561575b99de30e761aa5b183ea1a6a1319a54d7))
* **deps:** update dash-mpd requirement from ^0.10.0 to ^0.11.0 ([f8055c7](https://github.com/andrey-yantsen/plex-api.rs/commit/f8055c77d766c86091831ba0c35259be070e0158))
* **deps:** update dash-mpd requirement from ^0.11.0 to ^0.12.0 ([17a2493](https://github.com/andrey-yantsen/plex-api.rs/commit/17a2493bcf367497dd03499195da1ffa599bc746))
* **deps:** update dash-mpd requirement from ^0.12.0 to ^0.13.0 ([f68a881](https://github.com/andrey-yantsen/plex-api.rs/commit/f68a881f49a79f83a328b5b122acec765c3dc950))
* **deps:** update dash-mpd requirement from ^0.13.0 to ^0.14.0 ([6138dff](https://github.com/andrey-yantsen/plex-api.rs/commit/6138dff08b18096f5308487f107dc2d6232e4810))
* **deps:** update dash-mpd requirement from ^0.14.0 to ^0.15.0 ([3277356](https://github.com/andrey-yantsen/plex-api.rs/commit/3277356c64a22a02a86c867ad7091f293f232b86))
* **deps:** update dash-mpd requirement from ^0.15.0 to ^0.16.0 ([3ae07a6](https://github.com/andrey-yantsen/plex-api.rs/commit/3ae07a63e04e34f03a93e9794f174bf482d4ec17))
* **deps:** update dash-mpd requirement from ^0.16.0 to ^0.18.4 ([ed70fc8](https://github.com/andrey-yantsen/plex-api.rs/commit/ed70fc86941517024467b643a622df213e8642b7))
* **deps:** update dash-mpd requirement from ^0.9.0 to ^0.10.0 ([afe3697](https://github.com/andrey-yantsen/plex-api.rs/commit/afe3697fd27f293d33befe59f8276b26e12072d3))
* **deps:** update hls_m3u8 requirement from ^0.4.1 to ^0.5.1 ([f0d6c7d](https://github.com/andrey-yantsen/plex-api.rs/commit/f0d6c7d8977114155b333cd08fb318a378018767))
* **deps:** update httpmock requirement from ^0.6 to ^0.7 ([c0962c8](https://github.com/andrey-yantsen/plex-api.rs/commit/c0962c88498251f33258c50102199c022e420197))
* **deps:** update httpmock requirement from ^0.7 to ^0.8 ([6223ed5](https://github.com/andrey-yantsen/plex-api.rs/commit/6223ed5a6a3b8ab7a5c9702c23af48519309bbb5))
* **deps:** update image requirement from ^0.24.5 to ^0.25.0 ([0fe9ff6](https://github.com/andrey-yantsen/plex-api.rs/commit/0fe9ff66e8c114454cb2f3e35b4679827233c364))
* **deps:** update monostate requirement from ^0.1.2 to ^1.0.0 ([3f1250a](https://github.com/andrey-yantsen/plex-api.rs/commit/3f1250aae3ac5a7f3c71bc6d03c278c61ab7723c))
* **deps:** update mp3-metadata requirement from ^0.3.4 to ^0.4.0 ([1add8f0](https://github.com/andrey-yantsen/plex-api.rs/commit/1add8f063ac7d52b3b7f1b1e4c94d4fe93f344e3))
* **deps:** update mp4 requirement from ^0.13.0 to ^0.14.0 ([2ce4b35](https://github.com/andrey-yantsen/plex-api.rs/commit/2ce4b35857c437b8824845f651ad85e518e92f69))
* **deps:** update quick-xml requirement from ^0.28 to ^0.29 ([cc1960a](https://github.com/andrey-yantsen/plex-api.rs/commit/cc1960a178aba34e6f720b54c139061a41ad4525))
* **deps:** update quick-xml requirement from ^0.29 to ^0.30 ([4ee3353](https://github.com/andrey-yantsen/plex-api.rs/commit/4ee33537409c434a516b911e19a3c4facfb36ebb))
* **deps:** update quick-xml requirement from ^0.30 to ^0.31 ([7da1603](https://github.com/andrey-yantsen/plex-api.rs/commit/7da1603791e611b9a6cce75120d8f41e90dbbd2b))
* **deps:** update quick-xml requirement from ^0.31 to ^0.36 ([64ccbfe](https://github.com/andrey-yantsen/plex-api.rs/commit/64ccbfe85478e475121e8b54f54340c598519cbd))
* **deps:** update rstest requirement from ^0.17.0 to ^0.18.1 ([6ed9f9e](https://github.com/andrey-yantsen/plex-api.rs/commit/6ed9f9ea0a2b7ca30d4055f8293a853f0ce4a20a))
* **deps:** update rstest requirement from ^0.18.1 to ^0.19.0 ([100d1df](https://github.com/andrey-yantsen/plex-api.rs/commit/100d1dfda7ec1df47388970c07cafc2996e754e4))
* **deps:** update rstest requirement from ^0.19.0 to ^0.21.0 ([a1e2e7b](https://github.com/andrey-yantsen/plex-api.rs/commit/a1e2e7bfad14fc668e6a6aa0dec7ceb7aeaaeffa))
* **deps:** update sysinfo requirement from 0.29.9 to 0.30.1 ([d588b42](https://github.com/andrey-yantsen/plex-api.rs/commit/d588b422bce829f45eddf41c35966ab5b411e73b))
* **deps:** update thiserror requirement from ^1.0 to ^2.0 ([ca112e9](https://github.com/andrey-yantsen/plex-api.rs/commit/ca112e98e3b18455f346ff27182b673b0ad52492))
* **deps:** upgrade http ([752f466](https://github.com/andrey-yantsen/plex-api.rs/commit/752f4668b4fcc12a726076e4651987502bf4ffb3))
* disable default-features for dash-mpd ([18db6fd](https://github.com/andrey-yantsen/plex-api.rs/commit/18db6fd0c96f8a2452153f52bb37aec4ba91a5fc))
* fix a new warning from Clippy ([f73997f](https://github.com/andrey-yantsen/plex-api.rs/commit/f73997fba185bbe13968756f27ff83648038771e))
* fix breaking changes after httpmock update ([6b14a01](https://github.com/andrey-yantsen/plex-api.rs/commit/6b14a01e1f2111dffff58d85bea4f42a8fa2993e))
* make Clippy happy by using clone_from() ([7d6bbff](https://github.com/andrey-yantsen/plex-api.rs/commit/7d6bbffcf47fc8a385e382c525bf9bf635325044))
* remove the unnecessary TryFrom imports ([af1469f](https://github.com/andrey-yantsen/plex-api.rs/commit/af1469f209a262bd671a6cf9784830d1241978d7))
* remove unnecessary Server import from a test ([6d88730](https://github.com/andrey-yantsen/plex-api.rs/commit/6d88730a36d8aa356bc4372f882687f9c5b85fc6))
* replace ToString impl with Display ([1acdcb8](https://github.com/andrey-yantsen/plex-api.rs/commit/1acdcb849b16b8e9cf296e72aacc7fe1ae5b3da0))
* update Feature enum ([4391913](https://github.com/andrey-yantsen/plex-api.rs/commit/43919136e439808e6639a471853efecbb43b3f02))
* update Feature enum ([3f2c338](https://github.com/andrey-yantsen/plex-api.rs/commit/3f2c33837e164ffd0b5f0b3217642ed7346785df))
* update Feature enum ([636f8ed](https://github.com/andrey-yantsen/plex-api.rs/commit/636f8edbde1ad3eacf5b88bf715d76754f1fe6bd))
* update Feature enum ([b3defb1](https://github.com/andrey-yantsen/plex-api.rs/commit/b3defb164c0fca28704cc05de10684f2f5d09a58))
* update Feature enum ([7a09b2e](https://github.com/andrey-yantsen/plex-api.rs/commit/7a09b2e39714cb49b32837fc8b27674e4730aa15))
* update Feature enum ([67d9660](https://github.com/andrey-yantsen/plex-api.rs/commit/67d9660c88e79244cc3d3c4bd0117c27012c665d))
* update Feature enum ([ea2a37f](https://github.com/andrey-yantsen/plex-api.rs/commit/ea2a37f72a1c17df0a4f4131c8a2f8de19217e45))
* update Feature enum ([6fbef90](https://github.com/andrey-yantsen/plex-api.rs/commit/6fbef90873920110d7e7ec0b33b1bb3d0d09f609))
* update Feature enum ([e6f5a91](https://github.com/andrey-yantsen/plex-api.rs/commit/e6f5a910ec19bfb34db0e890a583f1da2413c821))
* update Feature enum ([b84c723](https://github.com/andrey-yantsen/plex-api.rs/commit/b84c723ddec835b744bae1ccc2e5e806c1f7038e))
* update Feature enum ([7fe5ad9](https://github.com/andrey-yantsen/plex-api.rs/commit/7fe5ad94f2d4551f227aea3cf2f7c7f5217559fa))
* update Feature enum ([92bee65](https://github.com/andrey-yantsen/plex-api.rs/commit/92bee6544bada811ada85c81d2baf8a9aeeef431))
* update Feature enum ([62022a0](https://github.com/andrey-yantsen/plex-api.rs/commit/62022a0b13c9683a783079d73682384b94f087cf))
* update Feature enum ([48403a6](https://github.com/andrey-yantsen/plex-api.rs/commit/48403a6c4d73d949533fce3751e25a9ae09fbc82))
* update Feature enum ([396c782](https://github.com/andrey-yantsen/plex-api.rs/commit/396c782de5c6f599de9f505ba97763f63f142d29))
* update Feature enum ([59c374e](https://github.com/andrey-yantsen/plex-api.rs/commit/59c374e392b49b8c89dae031f1d151b1b6b9dfa8))
* update Feature enum ([2f63e14](https://github.com/andrey-yantsen/plex-api.rs/commit/2f63e145ffbdaba2977dd18d4eb8f7cd73148024))
* update Feature enum ([4c4d579](https://github.com/andrey-yantsen/plex-api.rs/commit/4c4d579214556e201e9818c682e28e1a0d4c06f7))
* update Feature enum ([709d49d](https://github.com/andrey-yantsen/plex-api.rs/commit/709d49dde1daf3ee5e6b0cad5f15fd9824c45b00))
* update Feature enum ([5060f35](https://github.com/andrey-yantsen/plex-api.rs/commit/5060f3503fd0a3036a3ac68c24975933c65015d7))
* update Feature enum ([800852b](https://github.com/andrey-yantsen/plex-api.rs/commit/800852b89e0d354be0a679202466d7bdf9e82492))
* update Feature enum ([f3b703c](https://github.com/andrey-yantsen/plex-api.rs/commit/f3b703c65fec7bf42f689ef9fb15ae362c5f22f6))

## [0.0.11](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.10...plex-api-v0.0.11) (2023-06-02)


### ⚠ BREAKING CHANGES

* `Profile#default_subtitle_accessibility` is now an `Enum` `Profile#default_subtitle_forced` is now an `Enum`.

### Bug Fixes

* Profile de-serialization when using new subtitle setting options ([45bab7a](https://github.com/andrey-yantsen/plex-api.rs/commit/45bab7a25f46e9919c5f370f70e5abcd2f692c4d))


### Miscellaneous

* update Feature enum ([106150d](https://github.com/andrey-yantsen/plex-api.rs/commit/106150d4062e49c7589965118dc36c556f7a4ae2))
* update Feature enum ([7f2e9f2](https://github.com/andrey-yantsen/plex-api.rs/commit/7f2e9f2f0770e6cf1a0a28883973f9b0e01da785))

## [0.0.10](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.9...plex-api-v0.0.10) (2023-05-19)


### ⚠ BREAKING CHANGES

* Chapter.Filter is not always present
* a typo in MappingState::Waiting variant
* add more missed types and fields
* add new fields to make it work with local assets
* make Preferences use own HttpClient
* Make rating keys consistent
* make rating key to be string
* add Markers parsing
* make HttpClientBuilder::set_x_plex_features accept slices
* make HttpClientBuilder accept more params types
* merge metadata type fields into one
* add extras and some other new metadata fields
* make Guids to be enum variants

### Features

* add APIs for setting an item's watched status ([3b04bb0](https://github.com/andrey-yantsen/plex-api.rs/commit/3b04bb0ee45474d85a1a098c1c006a12ba83d305))
* add extras and some other new metadata fields ([46b6956](https://github.com/andrey-yantsen/plex-api.rs/commit/46b6956ae6ea419ccd79ecc233ebe41a875f75b7))
* add has_premium_extras to Metadata ([1508142](https://github.com/andrey-yantsen/plex-api.rs/commit/1508142db025e35d8a4b8ef2eb299f54d086f767))
* add labels to metadata ([714fffd](https://github.com/andrey-yantsen/plex-api.rs/commit/714fffd0bdeb13734597b3b2b23b5589c57664a0))
* add Markers parsing ([c901c4b](https://github.com/andrey-yantsen/plex-api.rs/commit/c901c4be9182ab7a12600395925bd7b4e5688b08))
* add missing `thumb` property in Chapter ([cb3eb90](https://github.com/andrey-yantsen/plex-api.rs/commit/cb3eb90eb1a46cac4d78fcea4a1746912004d918))
* add more missed types and fields ([d8fe885](https://github.com/andrey-yantsen/plex-api.rs/commit/d8fe88565af28609753c0c0e02182dc82e5f0a87))
* add new fields to make it work with local assets ([123c542](https://github.com/andrey-yantsen/plex-api.rs/commit/123c542f180c138904c17c3d9d9f160b31edd9b9))
* add new metadata fields ([efe828e](https://github.com/andrey-yantsen/plex-api.rs/commit/efe828ee426068c0a1fe072be6fced0f1d8e28fb))
* add syntax sugar for checking device types ([09f4377](https://github.com/andrey-yantsen/plex-api.rs/commit/09f4377e71690d4e9588dd7652c0f25febd62848))
* add viewing an item by it from Discovery ([a496636](https://github.com/andrey-yantsen/plex-api.rs/commit/a496636c492503d200a5359c130b45af5ee1fcaa))
* expose internal library type ([f2957eb](https://github.com/andrey-yantsen/plex-api.rs/commit/f2957eb5e8982ce2863b9f09d78619c5312b804d))
* load albums via search ([2574ca2](https://github.com/andrey-yantsen/plex-api.rs/commit/2574ca2d704379e108ee13a82ccbd5b4a83172a7))
* make HttpClientBuilder accept more params types ([75ac984](https://github.com/andrey-yantsen/plex-api.rs/commit/75ac9846cef3bcce96fc77e1fcd331fc164adab8))
* pass library_section_id to parsed media ([cf3c868](https://github.com/andrey-yantsen/plex-api.rs/commit/cf3c868ebddd8c285e248ba5d8fdf5804e88bf15))
* print received body on deserialization errors ([4a54359](https://github.com/andrey-yantsen/plex-api.rs/commit/4a5435976ee9aef94ad2e4d00ab5ba1322c34939))


### Bug Fixes

* a typo in MappingState::Waiting variant ([d69a834](https://github.com/andrey-yantsen/plex-api.rs/commit/d69a834102a83fafa40b3accd8e91d31c1112500))
* Allow parsing Live TV libraries ([d8a9140](https://github.com/andrey-yantsen/plex-api.rs/commit/d8a9140b4cf8e1c80ad84ecc5c7cf4c689d16891))
* Chapter.Filter is not always present ([49cab2f](https://github.com/andrey-yantsen/plex-api.rs/commit/49cab2f1cced4f169cf658c7af5c2760eae1cf01))
* Correctly use a 404 response to indicate an unknown transcode session. ([5cd7e27](https://github.com/andrey-yantsen/plex-api.rs/commit/5cd7e27982d312f0cd7b55760a0166e414a8e3fe))
* Generic videos should be transcodable ([d2e07f2](https://github.com/andrey-yantsen/plex-api.rs/commit/d2e07f2b2005c15b4eb45a51d15313663e20e868))
* make errors on unknown guids slightly nicer ([f6c7121](https://github.com/andrey-yantsen/plex-api.rs/commit/f6c712198d4062c8f526289b1f6f204a710790cd))
* make more fields accept strings as numbers ([62bd6b0](https://github.com/andrey-yantsen/plex-api.rs/commit/62bd6b07591e6ab27b7ff94808f47c29f1edc982))
* Report ItemNotFound from lost transcode sessions ([0fb6e42](https://github.com/andrey-yantsen/plex-api.rs/commit/0fb6e42b719f59b52406275d79f77ddde791adbf))
* return appropriate MetadataType if subtype is missing ([6e81477](https://github.com/andrey-yantsen/plex-api.rs/commit/6e81477dd0ec2482b3f0fbffdb206f8cf6a4032f))
* Send a unique X-Plex-Session-Identifier with each transcode request ([0f8d266](https://github.com/andrey-yantsen/plex-api.rs/commit/0f8d266377e58f04f9f8bfb6afe1ad283d8ed36a))


### Code Refactoring

* make Guids to be enum variants ([332d07d](https://github.com/andrey-yantsen/plex-api.rs/commit/332d07dfba661ee9f0dbd767d0f0988f0491bd10))
* make HttpClientBuilder::set_x_plex_features accept slices ([4402e5b](https://github.com/andrey-yantsen/plex-api.rs/commit/4402e5b24945019549419dc1a5a23f0a04ad2fcf))
* make Preferences use own HttpClient ([10d392e](https://github.com/andrey-yantsen/plex-api.rs/commit/10d392e0f3b83eef6aa3fd03ce7e5b84ccb40a57))
* make rating key to be string ([1748233](https://github.com/andrey-yantsen/plex-api.rs/commit/174823354bf2c6704ced25c3ebf8f99d2c501d8d))
* Make rating keys consistent ([8c8df57](https://github.com/andrey-yantsen/plex-api.rs/commit/8c8df57687446b55cfb75d5e3da94f9a6a5d6854))
* merge metadata type fields into one ([5a790d9](https://github.com/andrey-yantsen/plex-api.rs/commit/5a790d9dea949e73d97d603fd0357e7211457aa3))


### Documentation

* add an example that loads all the items ([e35d4e3](https://github.com/andrey-yantsen/plex-api.rs/commit/e35d4e3668961b09c46fe452603bbeb5679d3f69))


### Miscellaneous

* add a script for transcoding DoS ([1afc908](https://github.com/andrey-yantsen/plex-api.rs/commit/1afc908ca3808cc849b72b5b96073b9f783ba29e)), closes [#541](https://github.com/andrey-yantsen/plex-api.rs/issues/541)
* add an example for debugging items ([5c9dc6d](https://github.com/andrey-yantsen/plex-api.rs/commit/5c9dc6d743054278b551cd7ab620deef756c564b))
* allow println! in http_client when testing ([33753c2](https://github.com/andrey-yantsen/plex-api.rs/commit/33753c2619e99a66be58d40c3a9235b8e2520760))
* **deps:** update dash-mpd requirement from ^0.7.0 to ^0.8.0 ([360b627](https://github.com/andrey-yantsen/plex-api.rs/commit/360b62754d1373047342a603be3e1a7bce347a55))
* **deps:** update dash-mpd requirement from ^0.8.0 to ^0.9.0 ([977e26d](https://github.com/andrey-yantsen/plex-api.rs/commit/977e26ddd210b53d49e6abf104981312d0676652))
* **deps:** update serde_with requirement from ^2.1 to ^3.0 ([d5effb2](https://github.com/andrey-yantsen/plex-api.rs/commit/d5effb2a632d14a1fb553113772936f9a85c9594))
* forbid printing debug output ([9f4f496](https://github.com/andrey-yantsen/plex-api.rs/commit/9f4f496965d91e6c0389746096b7f5ff0bae408b))
* remove forgotten dbg!() ([f1fc7c5](https://github.com/andrey-yantsen/plex-api.rs/commit/f1fc7c538cc88b1d79cca013898e4b221ef27d80))
* update Feature enum ([1f1a7a0](https://github.com/andrey-yantsen/plex-api.rs/commit/1f1a7a0635fadf8d2d5d10416149fb1452aabe09))
* update Feature enum ([d415390](https://github.com/andrey-yantsen/plex-api.rs/commit/d415390fb8eecd2b219a5fdd3d08f0eb866911f8))
* update msrv to 1.65 ([0785ade](https://github.com/andrey-yantsen/plex-api.rs/commit/0785adefd7ff997ef84d079afd37900a2615682a))

## [0.0.9](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.8...plex-api-v0.0.9) (2023-03-29)


### ⚠ BREAKING CHANGES

* deny unreachable_pub and rename some exports

### Features

* expose MyPlex device names ([38cfaf5](https://github.com/andrey-yantsen/plex-api.rs/commit/38cfaf5320d655aaf3c5d64bca19f2f61dd10448))


### Bug Fixes

* Make a number of already exposed types publicly usable. ([97810dc](https://github.com/andrey-yantsen/plex-api.rs/commit/97810dc168daade7130674751371b29eee381514))


### Code Refactoring

* deny unreachable_pub and rename some exports ([04ecbc0](https://github.com/andrey-yantsen/plex-api.rs/commit/04ecbc08d1c0cb9e9f2e4be26d65900665855f16)), closes [#509](https://github.com/andrey-yantsen/plex-api.rs/issues/509)

## [0.0.8](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.7...plex-api-v0.0.8) (2023-03-24)


### ⚠ BREAKING CHANGES

* add Unknown variant to few more enums

### Code Refactoring

* add Unknown variant to few more enums ([b16c048](https://github.com/andrey-yantsen/plex-api.rs/commit/b16c04856208af4c89dc72d865ab0d519e3de274))

## [0.0.7](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.6...plex-api-v0.0.7) (2023-03-20)


### ⚠ BREAKING CHANGES

* move transcoding functions from Media to Part
* remove webhook.url getter
* remove `get_` prefix from getters
* allow creating MyPlex without authentication
* add tracing instrumenting for most of the methods
* remove PinManager::default()
* get rid of Arc over HttpClient
* add account info to MyPlex struct
* change MyPlexAccount::id to u64

### Features

* add a stub for sharing-related stuff ([c43c5c3](https://github.com/andrey-yantsen/plex-api.rs/commit/c43c5c3d388f6f9f1af17fabb273f7216664bde5))
* add announcements ([01a2137](https://github.com/andrey-yantsen/plex-api.rs/commit/01a213777d40d43ba2c7ac8352612575aa9f1678))
* add HttpClient::json_body() convenience method ([f1ba0a6](https://github.com/andrey-yantsen/plex-api.rs/commit/f1ba0a6eb28811b43d5c0bc21d15e2ad323d4586))
* add MyPlex::server_info() ([a8deb24](https://github.com/andrey-yantsen/plex-api.rs/commit/a8deb245c7e19fd0a6c8185505fd96001b1743c1))
* add sharing and friends management ([974ecb1](https://github.com/andrey-yantsen/plex-api.rs/commit/974ecb11217873d6ce3d3bb1501c23d6c1dc62eb))
* add support for connecting to shared servers ([0e3a4db](https://github.com/andrey-yantsen/plex-api.rs/commit/0e3a4db2a5c96612e31e00f84e3d290af93b8adf))
* add switching between Plex Home users ([536eab4](https://github.com/andrey-yantsen/plex-api.rs/commit/536eab49e757287ff1be7d436e80c1ca0f3707e8))
* add X-Plex-Model & X-Plex-Features headers ([25dbdec](https://github.com/andrey-yantsen/plex-api.rs/commit/25dbdec4fab9847b65574287ea19bee51f040c57))
* allow creating MyPlex without authentication ([1672639](https://github.com/andrey-yantsen/plex-api.rs/commit/16726398227c4aa5e7465d751d5be1fbf4ca3f15))
* disable timeout when downloading a file ([d17bff3](https://github.com/andrey-yantsen/plex-api.rs/commit/d17bff36770223e355fb1037a36f02a433eaccb3))
* remove PinManager::default() ([5106c37](https://github.com/andrey-yantsen/plex-api.rs/commit/5106c370a574e38b4d3290dff815426f2ef54da0))


### Bug Fixes

* add a missed field to Friend struct ([e886496](https://github.com/andrey-yantsen/plex-api.rs/commit/e886496fd98b68e2cadbaac8ee263c9e8f5be89b))
* add missed fields to Device ([70f09ae](https://github.com/andrey-yantsen/plex-api.rs/commit/70f09aec6aa7274346275cd8d15bdd5cf1491772))
* add the appropriate content-type for sharing request ([f5782af](https://github.com/andrey-yantsen/plex-api.rs/commit/f5782af8fd7237d4ce5b076f9116a8f497dd3e04))
* correct condition for player connection ([d18dc96](https://github.com/andrey-yantsen/plex-api.rs/commit/d18dc96602e3b4b3f1a4edb83070c4b057a45ad8))
* make Players connectable, even through server proxy ([8717cf9](https://github.com/andrey-yantsen/plex-api.rs/commit/8717cf923bc827fc5fb93b91fda4da80ad9aebb5))
* webhooks management is now working! ([376e5d0](https://github.com/andrey-yantsen/plex-api.rs/commit/376e5d0e919f59cf236ae3fe39012d9f0f9ddb72))


### Documentation

* add summary to some methods ([8c4cc08](https://github.com/andrey-yantsen/plex-api.rs/commit/8c4cc08bc06c306696fd00731a2ec3dde82de8f0))


### Miscellaneous

* add a few missed plexpass entities ([4005e0d](https://github.com/andrey-yantsen/plex-api.rs/commit/4005e0d8c5c1335bb9a94a4dde82c22c8c8ad57b))
* add some missed tests_deny_unknown_fields fallbacks ([666f439](https://github.com/andrey-yantsen/plex-api.rs/commit/666f439d2671c1005c14d9ba5af2c7a3923d9c53))
* add tracing instrumenting for most of the methods ([a323dd0](https://github.com/andrey-yantsen/plex-api.rs/commit/a323dd00cea39c4b8b651a69946f353bb15144f9))
* **deps:** update quick-xml requirement from ^0.27 to ^0.28 ([d54753d](https://github.com/andrey-yantsen/plex-api.rs/commit/d54753de4b776b8d05b7c85dd97a033f733e9fa6))
* **deps:** update rstest requirement from ^0.16.0 to ^0.17.0 ([b6d2959](https://github.com/andrey-yantsen/plex-api.rs/commit/b6d2959767d1ad2a8367da3d39d4dc7ae72c9d5e))
* fix lint warnings ([acbcb06](https://github.com/andrey-yantsen/plex-api.rs/commit/acbcb06c86bf2346495a261ea5b6dac16f084dc2))
* mark feature.rs as autogenerated ([2ce714c](https://github.com/andrey-yantsen/plex-api.rs/commit/2ce714caff23d6e3afcbd5d022f788c54ca0a1af))


### Code Refactoring

* add account info to MyPlex struct ([17f03fe](https://github.com/andrey-yantsen/plex-api.rs/commit/17f03fe4425b69f2eaf05d0ac60639b853d6d9ae))
* change MyPlexAccount::id to u64 ([05c8c6d](https://github.com/andrey-yantsen/plex-api.rs/commit/05c8c6d13dcbfd353c2eab68f17ddc7864255c0d))
* get rid of Arc over HttpClient ([928ed7f](https://github.com/andrey-yantsen/plex-api.rs/commit/928ed7ff40753d41a0ed0801cb6ae2d50c00a5c5))
* move transcoding functions from Media to Part ([6f9ad71](https://github.com/andrey-yantsen/plex-api.rs/commit/6f9ad71d13655f4f3405caad5b5ed4118e4f1b01))
* remove `get_` prefix from getters ([26ed77c](https://github.com/andrey-yantsen/plex-api.rs/commit/26ed77cd4f8bb9078dba081ea145de201f7a5c6a))
* remove webhook.url getter ([359f7a5](https://github.com/andrey-yantsen/plex-api.rs/commit/359f7a594df35c731d40b41a3baf3064d4fd5423))

## [0.0.6](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.5...plex-api-v0.0.6) (2023-03-05)


### ⚠ BREAKING CHANGES

* store all the tokens/passwords as SecretString
* decode start_state as enum instead of String
* decode updated_at as DateTime
* Add a way to retrieve specific items.
* Move a lot of the boilerplate from trait implementations into macros.

### Features

* Add a method for downloading media parts. ([72266cf](https://github.com/andrey-yantsen/plex-api.rs/commit/72266cf333fd646e62ddd73c777a08c481c9b68e))
* Add a way to retrieve specific items. ([34298d9](https://github.com/andrey-yantsen/plex-api.rs/commit/34298d95f6c05bab7254e04973354b1baeb10709))
* Add functionality for transcoding media. ([2584ed0](https://github.com/andrey-yantsen/plex-api.rs/commit/2584ed00b81bc8f0bbcd2e635409630783368054))
* Allow resizing item art. ([4a5da5e](https://github.com/andrey-yantsen/plex-api.rs/commit/4a5da5e9d7b2b89120007711c752a6f7d8633d3f))
* decode start_state as enum instead of String ([5fb355a](https://github.com/andrey-yantsen/plex-api.rs/commit/5fb355a179f63e1d6529eee9709e60cc44d6425d))
* decode updated_at as DateTime ([2909443](https://github.com/andrey-yantsen/plex-api.rs/commit/29094433bcd05903cf3623720efdb78f4e773aa6))
* store all the tokens/passwords as SecretString ([ca5e513](https://github.com/andrey-yantsen/plex-api.rs/commit/ca5e513722124c587e61333af6b0d35c7405772d))


### Bug Fixes

* Allow more unknown data to be deserialized without failing. ([8257a46](https://github.com/andrey-yantsen/plex-api.rs/commit/8257a46de9540cb569f2a98caaa626906f88b94e))
* make HttpClient::deletem() use only min headers ([94a2d4a](https://github.com/andrey-yantsen/plex-api.rs/commit/94a2d4aaf4cf6f79406d8f7935a5155dc61c5f92))
* Move a lot of the boilerplate from trait implementations into macros. ([95ddf42](https://github.com/andrey-yantsen/plex-api.rs/commit/95ddf425cddb4f5e20e1d143663e4e07da90d749))
* Only allocate default client if one is not provided. ([bcd9449](https://github.com/andrey-yantsen/plex-api.rs/commit/bcd944977f0a11b2866ead7e9ded9f559c75a006))
* Simplify the common case of requesting JSON data. ([0007c77](https://github.com/andrey-yantsen/plex-api.rs/commit/0007c7763c3e2b10dabd6c830cf01226e5415231))


### Documentation

* Add some API documentation. ([0364392](https://github.com/andrey-yantsen/plex-api.rs/commit/0364392d29146542dfb96d06ad17e402cafb7f56))


### Miscellaneous

* update msrv to 1.63.0 ([3128ebf](https://github.com/andrey-yantsen/plex-api.rs/commit/3128ebf53cdee18c6acf874c688fd46bcf36f36d))

## 0.0.5 (2023-01-06)


### ⚠ BREAKING CHANGES

* rename Client to HttpClient

### Features

* add a method for linking new players ([33d811f](https://github.com/andrey-yantsen/plex-api.rs/commit/33d811fc38b728bd4355b385a8b8d3541a6307a4))
* add a stub for Players (aka Clients) ([769d963](https://github.com/andrey-yantsen/plex-api.rs/commit/769d9631e729abbd75f67347868295011e2caa87))
* add joined_at field to MyPlexAccount ([4b8f4a7](https://github.com/andrey-yantsen/plex-api.rs/commit/4b8f4a7ea091486035f6e9c77d617bdb8a7029f4))
* add structs for receiving webhooks ([cc314e0](https://github.com/andrey-yantsen/plex-api.rs/commit/cc314e071252e55f8ce9daa86bcd66baa965f9d9))
* **api:** Add a high-level API for browsing libraries on a server. ([a315fcd](https://github.com/andrey-yantsen/plex-api.rs/commit/a315fcd978adb6c326d338eb440e0203ae52973e))
* improve working with pins ([bcb9fd0](https://github.com/andrey-yantsen/plex-api.rs/commit/bcb9fd06c54d05acc4cd58a528a15cd9d91ae76a))
* migrate the code to serde_with v2 ([7de8395](https://github.com/andrey-yantsen/plex-api.rs/commit/7de83955b53e39f2847ec10d7146ba39f61b8573))
* **srv:** add method for getting all settings ([fafb427](https://github.com/andrey-yantsen/plex-api.rs/commit/fafb42743412ad7f49961624b48baa9d263d868c))
* **srv:** add new uuid to the Feature enum ([3cd2bae](https://github.com/andrey-yantsen/plex-api.rs/commit/3cd2bae3e91b8b70dfc566cd8c33b88f7ef69b8f))
* **srv:** add preferences management ([7454bc5](https://github.com/andrey-yantsen/plex-api.rs/commit/7454bc57d84a577ec324c48f2117296e86d88cbc))
* update Feature enum ([48f679f](https://github.com/andrey-yantsen/plex-api.rs/commit/48f679f3a839a00d741221b158afabe7c9107e62))
* **webhooks:** make most of the fields optional ([a23fd30](https://github.com/andrey-yantsen/plex-api.rs/commit/a23fd303f126fc3bd1dec0fb050893fe65de2749))


### Bug Fixes

* certificateVersion is not returned from the myplex API ([8d2d8e4](https://github.com/andrey-yantsen/plex-api.rs/commit/8d2d8e46d8869d812a45a41bb2aa20cad7d4721a))
* get rid of the deprecated method usage ([452f7e3](https://github.com/andrey-yantsen/plex-api.rs/commit/452f7e3cde0f88ab36d5271d09f622ecd8edeb71))
* **srv:** older PMS can send numbers as strings ([96bb0bf](https://github.com/andrey-yantsen/plex-api.rs/commit/96bb0bfc6e8d7259bf542f4224e1f79f0b1c5ba3))


### Code Refactoring

* rename Client to HttpClient ([bb4e05e](https://github.com/andrey-yantsen/plex-api.rs/commit/bb4e05ecaa14461c822c9a5b99aeec0b2337750e))


### Miscellaneous

* add examples for signin & signout ([dbc7086](https://github.com/andrey-yantsen/plex-api.rs/commit/dbc7086ba7e82441007d11f3fd5842618469aac2))
* change MSRV to 1.56 and edition to 2021 ([7cb77b0](https://github.com/andrey-yantsen/plex-api.rs/commit/7cb77b00befcc5265c81e76e74bc8e157a2f0ff5))
* change MSRV to 1.61.0 ([d757b27](https://github.com/andrey-yantsen/plex-api.rs/commit/d757b272264a94c71ed03b02f08d8e76b9fe07d1))
* **deps:** bump async-std from 1.10.0 to 1.11.0 ([4aa9016](https://github.com/andrey-yantsen/plex-api.rs/commit/4aa9016c6918d148c07eb76e3100d225721e6524))
* **deps:** bump async-std from 1.11.0 to 1.12.0 ([7ca1b43](https://github.com/andrey-yantsen/plex-api.rs/commit/7ca1b43cb9e979133480038abc61037a9f8e9ae4))
* **deps:** bump futures from 0.3.21 to 0.3.23 ([ef4c726](https://github.com/andrey-yantsen/plex-api.rs/commit/ef4c72629cae00c865a2b9ce22599d89549e6cde))
* **deps:** bump futures from 0.3.23 to 0.3.24 ([7d06e1e](https://github.com/andrey-yantsen/plex-api.rs/commit/7d06e1e0337d422299aa96d5364d6e2f4d03be95))
* **deps:** bump futures from 0.3.24 to 0.3.25 ([356b230](https://github.com/andrey-yantsen/plex-api.rs/commit/356b2301f3e425f3f5f3d28d9712494235ec163f))
* **deps:** bump http from 0.2.6 to 0.2.7 ([25eb8cd](https://github.com/andrey-yantsen/plex-api.rs/commit/25eb8cd524ae1631455fecb0a8a8c5c6bcf846c6))
* **deps:** bump http from 0.2.7 to 0.2.8 ([2bf3813](https://github.com/andrey-yantsen/plex-api.rs/commit/2bf38138b70d64d8ac7a51b9df9c3e1f256dd7cc))
* **deps:** bump isahc from 1.6.0 to 1.7.0 ([51d0c84](https://github.com/andrey-yantsen/plex-api.rs/commit/51d0c84e421e62ca4123821f47ee1ed5c2d7e293))
* **deps:** bump isahc from 1.7.0 to 1.7.1 ([473bbc6](https://github.com/andrey-yantsen/plex-api.rs/commit/473bbc626beaee31cd4c1ca81a17e6fa9b8a38d7))
* **deps:** bump isahc from 1.7.1 to 1.7.2 ([6480b7b](https://github.com/andrey-yantsen/plex-api.rs/commit/6480b7b2fed656fce08a57dcedebf838ce4d6b71))
* **deps:** bump monostate from 0.1.1 to 0.1.2 ([d43ded7](https://github.com/andrey-yantsen/plex-api.rs/commit/d43ded7b2d26fa1fe82e0d6ffb7a4b475ef1e069))
* **deps:** bump quick-xml from 0.22.0 to 0.23.0 ([3242b93](https://github.com/andrey-yantsen/plex-api.rs/commit/3242b93adebbc759b0cb62a2c222b5515e43085b))
* **deps:** bump quick-xml from 0.23.0 to 0.24.0 ([235b116](https://github.com/andrey-yantsen/plex-api.rs/commit/235b116f4fa6843dbd287d9908e82a5cddec51da))
* **deps:** bump quick-xml from 0.24.0 to 0.25.0 ([7a848eb](https://github.com/andrey-yantsen/plex-api.rs/commit/7a848eb6143bfba123103682e0cec29cd0d980e6))
* **deps:** bump quick-xml from 0.25.0 to 0.26.0 ([e1e799f](https://github.com/andrey-yantsen/plex-api.rs/commit/e1e799f61d432f6c1f1a8be5da10fe4414a4dd90))
* **deps:** bump quick-xml from 0.26.0 to 0.27.1 ([8be7d4f](https://github.com/andrey-yantsen/plex-api.rs/commit/8be7d4fdc83e4cd22be5563434d19b0be36d3cd6))
* **deps:** bump regex from 1.5.6 to 1.6.0 ([8c843ea](https://github.com/andrey-yantsen/plex-api.rs/commit/8c843ea7600f79b5389140482c8d53a4e7565536))
* **deps:** bump regex from 1.6.0 to 1.7.0 ([b2bf01d](https://github.com/andrey-yantsen/plex-api.rs/commit/b2bf01d0d6d760cc004ef7fcd20173c9dfb103d9))
* **deps:** bump rpassword from 5.0.1 to 6.0.1 ([25c1c3f](https://github.com/andrey-yantsen/plex-api.rs/commit/25c1c3fd9504b2f91ed97aab8caa44d9a5bfc932))
* **deps:** bump rpassword from 6.0.1 to 7.0.0 ([20db373](https://github.com/andrey-yantsen/plex-api.rs/commit/20db37398d0fa84727af1330ff0b421bbe22c225))
* **deps:** bump rpassword from 7.0.0 to 7.1.0 ([be47b33](https://github.com/andrey-yantsen/plex-api.rs/commit/be47b331dfb6aaffd29e77fe9c4a9466bcd5bacb))
* **deps:** bump rpassword from 7.1.0 to 7.2.0 ([cf0fc1b](https://github.com/andrey-yantsen/plex-api.rs/commit/cf0fc1b746737fb6be3eabd31af29cf1b77d9c04))
* **deps:** bump rstest from 0.12.0 to 0.13.0 ([f01f31a](https://github.com/andrey-yantsen/plex-api.rs/commit/f01f31afbf4557eeb777a7ec413f9268d2ac6787))
* **deps:** bump rstest from 0.13.0 to 0.14.0 ([a117993](https://github.com/andrey-yantsen/plex-api.rs/commit/a11799366a01688e0c7f7a1be92b41942665ac80))
* **deps:** bump rstest from 0.14.0 to 0.15.0 ([2381b64](https://github.com/andrey-yantsen/plex-api.rs/commit/2381b642cb8bfcd9f57a5861605b14bd35e8d7f1))
* **deps:** bump rstest from 0.15.0 to 0.16.0 ([a044057](https://github.com/andrey-yantsen/plex-api.rs/commit/a0440574b0f6af003c4b585c6bbc47d955fbedb8))
* **deps:** bump serde_plain from 1.0.0 to 1.0.1 ([a365424](https://github.com/andrey-yantsen/plex-api.rs/commit/a3654240e8929d6e70dc1b9660d7bf77eb58b606))
* **deps:** bump serde_with from 1.11.0 to 1.12.0 ([f25394a](https://github.com/andrey-yantsen/plex-api.rs/commit/f25394a0f995cada7d1599ccde033d5c344dcfc6))
* **deps:** bump serde_with from 1.12.1 to 1.13.0 ([ea86bfc](https://github.com/andrey-yantsen/plex-api.rs/commit/ea86bfcfb677fdcb27fb06aa07a65c39373afbc4))
* **deps:** bump serde_with from 1.13.0 to 1.14.0 ([54b514f](https://github.com/andrey-yantsen/plex-api.rs/commit/54b514fc8e35ee1359178e44b5a8d2d90c0f8ee1))
* **deps:** bump serde_with from 1.14.0 to 2.1.0 ([965149a](https://github.com/andrey-yantsen/plex-api.rs/commit/965149a485b8bb7c385d7d66424316465971b573))
* **deps:** bump serde-aux from 4.0.0 to 4.1.0 ([c77a9da](https://github.com/andrey-yantsen/plex-api.rs/commit/c77a9da275a3010ad2507ffc8ac8908778abc00f))
* **deps:** bump serde-aux from 4.1.0 to 4.1.2 ([a6fe462](https://github.com/andrey-yantsen/plex-api.rs/commit/a6fe462cba17043b39c1111e1d8c00f86260af78))
* **deps:** bump uuid from 0.8.2 to 1.0.0 ([2bece57](https://github.com/andrey-yantsen/plex-api.rs/commit/2bece57759df502563bd57391869b79739d64a5c))
* **deps:** bump uuid from 1.0.0 to 1.1.1 ([612bff4](https://github.com/andrey-yantsen/plex-api.rs/commit/612bff4ccf81c4f95076515f4b609b133b675082))
* **deps:** bump uuid from 1.1.2 to 1.2.1 ([c79804f](https://github.com/andrey-yantsen/plex-api.rs/commit/c79804f22e2ec3f292ad5f56ad40905fa22a9345))
* **examples:** fix rpassword usage after upgrade ([b5724a0](https://github.com/andrey-yantsen/plex-api.rs/commit/b5724a02ff01573d9f3663b5e010132a0f984c38))
* **features:** use better check for is_unknown_value ([4177ae1](https://github.com/andrey-yantsen/plex-api.rs/commit/4177ae1b79d0d73f9f46475eea80401e9adeb34e))
* get rid of unused lifetime in build.rs ([e925883](https://github.com/andrey-yantsen/plex-api.rs/commit/e925883d6f97bc11f99a45fdf423df44184227da))
* **main:** release plex-api 0.0.4 ([59ece83](https://github.com/andrey-yantsen/plex-api.rs/commit/59ece83ef6322bad57318afeadd3cda16374e8c0))
* make clippy happier with expect_err() ([1df2b52](https://github.com/andrey-yantsen/plex-api.rs/commit/1df2b52ec63be5ae3867782a20c02be601ee707a))
* make it compile with quick_xml 0.27 ([0f621a6](https://github.com/andrey-yantsen/plex-api.rs/commit/0f621a6baee29b157500ec64b2043f06fb02a36e))
* remove needless borrow (thanks, clippy!) ([abcfdf0](https://github.com/andrey-yantsen/plex-api.rs/commit/abcfdf06cff2eef4669fda9fa1e69697bb69af03))
* update Feature enum ([b28ac67](https://github.com/andrey-yantsen/plex-api.rs/commit/b28ac675fae3baca3161f58a46173a50ad2118d5))
* update Feature enum ([7d318fd](https://github.com/andrey-yantsen/plex-api.rs/commit/7d318fdded24b92249d06070730b8b043f0e31df))
* update Feature enum ([0e62035](https://github.com/andrey-yantsen/plex-api.rs/commit/0e62035d116f268b5ed514a2df62ba754f9ce2d8))
* update Feature enum ([894608d](https://github.com/andrey-yantsen/plex-api.rs/commit/894608dec7edd77c78d116f34f4fdf01da957f70))
* update Feature enum ([fcfa47d](https://github.com/andrey-yantsen/plex-api.rs/commit/fcfa47d203b4b402cd7ebbc797456d36b7227dc3))
* update Feature enum ([175b3b0](https://github.com/andrey-yantsen/plex-api.rs/commit/175b3b0c3171b54d22b79fc160f03977056e348b))
* update Feature enum ([4b6daa9](https://github.com/andrey-yantsen/plex-api.rs/commit/4b6daa9f8288a784f051b060eb5bbfbf77ea4f38))
* update Feature enum ([a855adb](https://github.com/andrey-yantsen/plex-api.rs/commit/a855adbb09f1073276030d157946251d48215158))
* update Feature enum ([43b6f2b](https://github.com/andrey-yantsen/plex-api.rs/commit/43b6f2b0f67fd53712240ddead1a1e64a6c99805))
* update Feature enum ([9595cca](https://github.com/andrey-yantsen/plex-api.rs/commit/9595cca051b3b708592c57f75d4c46f480851894))
* update Feature enum ([e984f67](https://github.com/andrey-yantsen/plex-api.rs/commit/e984f67b1fc6f71bda4330d1d42618813021d826))
* update Feature enum ([20f2d0e](https://github.com/andrey-yantsen/plex-api.rs/commit/20f2d0e8e7ba9b474a7663ad5ca56b8f0ebbcbdc))

### [0.0.4](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.3...plex-api-v0.0.4) (2022-02-11)


### Features

* **srv:** add preferences management ([7454bc5](https://github.com/andrey-yantsen/plex-api.rs/commit/7454bc57d84a577ec324c48f2117296e86d88cbc))


### Bug Fixes

* **srv:** older PMS can send numbers as strings ([96bb0bf](https://github.com/andrey-yantsen/plex-api.rs/commit/96bb0bfc6e8d7259bf542f4224e1f79f0b1c5ba3))


### Miscellaneous

* **deps:** bump serde_with from 1.11.0 to 1.12.0 ([f25394a](https://github.com/andrey-yantsen/plex-api.rs/commit/f25394a0f995cada7d1599ccde033d5c344dcfc6))
