## [Release 0.6.0](https://github.com/aetheric-oss/svc-pricing/releases/tag/v0.6.0)

### ✨ Features

-  **client-grpc:** add prelude module and fix examples ([`6e55d2f`](https://github.com/aetheric-oss/svc-pricing/commit/6e55d2fdf2f31d3e1f91ed83e696dbbcb7a13b5d))
- improve unit tests and arrow references ([`2ce2b1f`](https://github.com/aetheric-oss/svc-pricing/commit/2ce2b1fe1317e5f7d4cfe61c00e361ceabe76b50))

### 🐛 Fixes

- update arrow-air refs to aetheric-oss ([`8428417`](https://github.com/aetheric-oss/svc-pricing/commit/842841720230c4391f11085bf11ec80450988d55))
- add error log to failure on Config unwrap ([`943ccfb`](https://github.com/aetheric-oss/svc-pricing/commit/943ccfb5dfeb6b82cf7c495dab07ce90f92b9fc6))
- update references to aetheric-oss ([`10849ea`](https://github.com/aetheric-oss/svc-pricing/commit/10849eaf3dc1ea0768840ffd82d940bcdb079ea8))

### 🛠 Maintenance

- terraform provisioned file changes ([`281a991`](https://github.com/aetheric-oss/svc-pricing/commit/281a99161da63f7fb302a305e54e68614403dda2))
- update cargo dependencies ([`40ba028`](https://github.com/aetheric-oss/svc-pricing/commit/40ba02878ac279c0b901412293e749c187b3529a))
- update dependencies ([`7a6df8f`](https://github.com/aetheric-oss/svc-pricing/commit/7a6df8fbcce1e9564323d701cea3fdb36b3c6707))
- terraform provisioned file changes ([`fc11ea5`](https://github.com/aetheric-oss/svc-pricing/commit/fc11ea538f01bec379b353ae1153a4611c0e4b60))
- tofu provisioned file changes ([`4376153`](https://github.com/aetheric-oss/svc-pricing/commit/437615340c0d336c95e922fb1d2b479d23daea7a))
- final fixes for R4 ([`a054986`](https://github.com/aetheric-oss/svc-pricing/commit/a054986d54fc47b39875bb516b5add71809b60dd))
- update references ([`030a5b9`](https://github.com/aetheric-oss/svc-pricing/commit/030a5b961079a85ae4e0e49cac94fa4ea1012806))

## [Release 0.5.1](https://github.com/Arrow-air/svc-pricing/releases/tag/v0.5.1)

### 🔥 Refactorings

- use lib-common and mock/stub additions ([`00f2b77`](https://github.com/Arrow-air/svc-pricing/commit/00f2b7751704e43d435f85b0f86fc444eab76154))

### 🛠 Maintenance

- terraform provisioned file changes ([`35f31ff`](https://github.com/Arrow-air/svc-pricing/commit/35f31ff69979c8d121991f37f3f203c7db8d4834))
- r3 final cleanup ([`c5cc2e1`](https://github.com/Arrow-air/svc-pricing/commit/c5cc2e10a5dead6c2af1e5c4e370877bbce91726))

### 📚 Documentation

- fix icons/banners and add client README ([`09242c2`](https://github.com/Arrow-air/svc-pricing/commit/09242c22eb64a2650fff901ff80e5e13f255c9a9))

## [Release 0.5.0](https://github.com/Arrow-air/svc-pricing/releases/tag/v0.5.0)

### ✨ Features

- enhance test coverage to 100% ([`5cecee6`](https://github.com/Arrow-air/svc-pricing/commit/5cecee6d41187c937a4629438811b42b0a859bec))

### 🐛 Fixes

- address review comments ([`17d9bf3`](https://github.com/Arrow-air/svc-pricing/commit/17d9bf3b229051e65e10107c1e7d5756c5de34e0))

### 🔥 Refactorings

-  **pricing:** support pricing for multi-leg trips ([`980b996`](https://github.com/Arrow-air/svc-pricing/commit/980b9969b8ca39e03d9cdf74c49549738995863b))
-  **pricing:** support returning an array of prices for multi-leg trips ([`22086a4`](https://github.com/Arrow-air/svc-pricing/commit/22086a4e0957fc1999c4ba4f4dda76cfb190a7e6))

### 🛠 Maintenance

- terraform provisioned file changes ([`81c8d0e`](https://github.com/Arrow-air/svc-pricing/commit/81c8d0e9ebae53efcfe2fbeaf467fd8c91146955))
- clean up for r2 review ([`d4a66ed`](https://github.com/Arrow-air/svc-pricing/commit/d4a66ed012b1a015b69ccf8776f875c19f274306))

### ✅ Tests

- ensures the order of returned prices ([`b18b6f6`](https://github.com/Arrow-air/svc-pricing/commit/b18b6f64d9ba9c499872caeecb8689be2143034b))

### 📚 Documentation

-  **readme:** add license notice and additional info ([`f653ff7`](https://github.com/Arrow-air/svc-pricing/commit/f653ff7e547201ea38dc0ad7858366ea2377b28b))

## [Release 0.4.0](https://github.com/Arrow-air/svc-pricing/releases/tag/v0.4.0)

### ✨ Features

-  **grpc:** add health check and related example (#4) ([`376add7`](https://github.com/Arrow-air/svc-pricing/commit/376add7471c64a5ed5d69e9236e7af4b7f5124c6))
-  **logger:** implement loggers for logging and debugging purposes ([`be8ed4c`](https://github.com/Arrow-air/svc-pricing/commit/be8ed4c97a0afffd0947f85d9ae3eaa0d73f909b))
-  **r1 release:** launch the official release ([`1674849`](https://github.com/Arrow-air/svc-pricing/commit/16748494026f24133fc364fde07deb9068f0713e))

### 🐛 Fixes

-  **grpc:** correct package name (#5) ([`f7dc73b`](https://github.com/Arrow-air/svc-pricing/commit/f7dc73b901ca0287825e4773083bbda44852996c))
-  **grpc:** server startup on arch linux (#7) ([`faf9c21`](https://github.com/Arrow-air/svc-pricing/commit/faf9c21a94b8d0e54577a3f7e6e363acf5431bb3))
-  **cargo:** add vendored-openssl feature ([`1045181`](https://github.com/Arrow-air/svc-pricing/commit/104518192b94ff659b39dfe1526de4983dd368eb))
-  **reviews:** address changes requested ([`f974800`](https://github.com/Arrow-air/svc-pricing/commit/f974800cc14ed879884f9ebe392b826fe3c9b806))

### 🛠 Maintenance

-  **init:** initial repository setup ([`fa31537`](https://github.com/Arrow-air/svc-pricing/commit/fa31537e72f06f49c10b9f08efd50aa27cecae20))
-  **grpc:** add km to `distance` field to improve readability (#6) ([`4bd15ba`](https://github.com/Arrow-air/svc-pricing/commit/4bd15baa1586a0d509aa48913690ad837b933ec2))
-  **logging:** upgrade to log4rs (#11) ([`87a0f8f`](https://github.com/Arrow-air/svc-pricing/commit/87a0f8f178ffb5e1ab5a2bc00291bf89058251bb))
-  **toml:** remove patch versions from dependencies (#12) ([`d96b4ee`](https://github.com/Arrow-air/svc-pricing/commit/d96b4eebc9d594f7da8fed21dce495d61741af1d))
-  **CHANGELOG:** reset for merging to main ([`5eff38f`](https://github.com/Arrow-air/svc-pricing/commit/5eff38f298cd0e508bee35d74b30690460629211))
- service Cargo versions for release ([`e2f9848`](https://github.com/Arrow-air/svc-pricing/commit/e2f9848b8b1814a15d74f7b950eef02132aae83b))

### 📚 Documentation

-  **changelog:** add CHANGELOG.md ([`06bb3d4`](https://github.com/Arrow-air/svc-pricing/commit/06bb3d43cae39cf3903e7cf9950df5e2eaba93f7))
-  **icd:** add Interface Control Document (ICD) Draft ([`3003fd4`](https://github.com/Arrow-air/svc-pricing/commit/3003fd4d1535fbb0069781fd524f6a91646dd085))
-  **conops:** add Concept of Operations (CONOPS) ([`0a38178`](https://github.com/Arrow-air/svc-pricing/commit/0a3817861ff2874192aab1b30181561cda8e6747))
-  **sdd:** publish Software Design Document (#9) ([`48a87ad`](https://github.com/Arrow-air/svc-pricing/commit/48a87adaba8592c098c55ca94c213cab6af8acfc))
