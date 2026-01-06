# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0](https://github.com/stac-utils/rustac-py/compare/v0.10.0...v1.0.0) (2026-01-06)


### ⚠ BREAKING CHANGES

* remove the typed dicts ([#177](https://github.com/stac-utils/rustac-py/issues/177))
* async search ([#24](https://github.com/stac-utils/rustac-py/issues/24))

### Features

* add ci ([74f73bd](https://github.com/stac-utils/rustac-py/commit/74f73bdc43c587f1b074dfc56e950dba06b2fd1e))
* add cli ([#46](https://github.com/stac-utils/rustac-py/issues/46)) ([1790e93](https://github.com/stac-utils/rustac-py/commit/1790e93d1d1dc7c35c63bea5f55e0d2e540ba5e7))
* add collection_from_id_and_items ([#109](https://github.com/stac-utils/rustac-py/issues/109)) ([89f92e4](https://github.com/stac-utils/rustac-py/commit/89f92e4829a5b78b8bb7418c10e0e204c713accf))
* add collections to parquet metadata ([#208](https://github.com/stac-utils/rustac-py/issues/208)) ([617ecca](https://github.com/stac-utils/rustac-py/commit/617ecca449c187b1d2bda4e49294f82f48d439e1))
* add conda badge ([e2b32b2](https://github.com/stac-utils/rustac-py/commit/e2b32b2e20ed3bec6583c7475b934e8bd4d127fc))
* add conda-forge note to readme ([b89b082](https://github.com/stac-utils/rustac-py/commit/b89b082cb597f96f82ae7331e305a00a6e4405c9))
* add config args to duckdb client ([#42](https://github.com/stac-utils/rustac-py/issues/42)) ([13a9190](https://github.com/stac-utils/rustac-py/commit/13a9190e0eeb292e6d8804ad21e1404151cee35e))
* add docs CI ([#2](https://github.com/stac-utils/rustac-py/issues/2)) ([df25b0e](https://github.com/stac-utils/rustac-py/commit/df25b0e6e112d11e7139fc8a4454e33560123673))
* add duckdb client ([#15](https://github.com/stac-utils/rustac-py/issues/15)) ([693c4d0](https://github.com/stac-utils/rustac-py/commit/693c4d04582908aa7d2d78ebf63c183d9615c53a))
* add geopandas notebook ([#57](https://github.com/stac-utils/rustac-py/issues/57)) ([65f8e24](https://github.com/stac-utils/rustac-py/commit/65f8e24a7433dfed090a978b209810ce1152010e))
* add icu extension ([#107](https://github.com/stac-utils/rustac-py/issues/107)) ([c6b0f30](https://github.com/stac-utils/rustac-py/commit/c6b0f30b17aaa498eb968f2ae1c58a5be47e8dea))
* add iter_search ([#151](https://github.com/stac-utils/rustac-py/issues/151)) ([d8ac4f9](https://github.com/stac-utils/rustac-py/commit/d8ac4f9d3afbac9253464cfb30f33051064a3597)), closes [#148](https://github.com/stac-utils/rustac-py/issues/148)
* add more docs ([#84](https://github.com/stac-utils/rustac-py/issues/84)) ([4f7c0d4](https://github.com/stac-utils/rustac-py/commit/4f7c0d46cd48cd9ef7a79c1e06a7782a8b5ae592))
* add parquet to extensions ([#108](https://github.com/stac-utils/rustac-py/issues/108)) ([18fc7ba](https://github.com/stac-utils/rustac-py/commit/18fc7bacdb9f33bc657c03f2f6f1287651a09c05))
* add public `build_search` ([#5](https://github.com/stac-utils/rustac-py/issues/5)) ([f468eec](https://github.com/stac-utils/rustac-py/commit/f468eeca61c4e7861b1ec887efbb041abee60cd3))
* add sha ([#99](https://github.com/stac-utils/rustac-py/issues/99)) ([91865b0](https://github.com/stac-utils/rustac-py/commit/91865b0ca3526005bc631bf8ec47f9187c3d3b31)), closes [#97](https://github.com/stac-utils/rustac-py/issues/97)
* add some readme notes ([4807f5e](https://github.com/stac-utils/rustac-py/commit/4807f5e3c64b03a4f778f209ef88643575e6fd0f))
* add store to search_to ([#130](https://github.com/stac-utils/rustac-py/issues/130)) ([07f89b7](https://github.com/stac-utils/rustac-py/commit/07f89b71982d1351402cf1fb519628b645c442ff))
* add stores ([#127](https://github.com/stac-utils/rustac-py/issues/127)) ([a54ae89](https://github.com/stac-utils/rustac-py/commit/a54ae8951693543a625a5668cc119bc8bdddb441))
* add typed dicts ([#101](https://github.com/stac-utils/rustac-py/issues/101)) ([ff603f4](https://github.com/stac-utils/rustac-py/commit/ff603f4b21fe9645c53f4e3a8643f002173e97aa)), closes [#95](https://github.com/stac-utils/rustac-py/issues/95)
* allow kwargs for search ([#14](https://github.com/stac-utils/rustac-py/issues/14)) ([8dfc8c6](https://github.com/stac-utils/rustac-py/commit/8dfc8c6725a4dae841c8b3b1887bcdc4ad1dd19f))
* async search ([#24](https://github.com/stac-utils/rustac-py/issues/24)) ([a0a547c](https://github.com/stac-utils/rustac-py/commit/a0a547c56f80d7404011635cbd3fe65f1d4a8f34))
* bring everything over ([79eee8b](https://github.com/stac-utils/rustac-py/commit/79eee8b5d5d24f252579e1d6d02a61a771be64db))
* build collections from stac-geoparquet ([#17](https://github.com/stac-utils/rustac-py/issues/17)) ([2de3756](https://github.com/stac-utils/rustac-py/commit/2de3756256b00394ae28d48186fb10bcd3e0522a))
* bump edition ([#104](https://github.com/stac-utils/rustac-py/issues/104)) ([b907143](https://github.com/stac-utils/rustac-py/commit/b907143644cd142c2b1d226605565a9d19120211))
* default to snappy compression for geoparquet ([#77](https://github.com/stac-utils/rustac-py/issues/77)) ([d75a684](https://github.com/stac-utils/rustac-py/commit/d75a684ae688be34152389da89665524c8f64db7))
* don't bundle duckdb ([#52](https://github.com/stac-utils/rustac-py/issues/52)) ([b7a8893](https://github.com/stac-utils/rustac-py/commit/b7a889348aacccae974bf5b493458f2fc590d42a))
* filter w/ invalid column ok ([#113](https://github.com/stac-utils/rustac-py/issues/113)) ([fb67122](https://github.com/stac-utils/rustac-py/commit/fb671227eaa423703e918d0e3bbf63af31311976))
* initial commit ([793224b](https://github.com/stac-utils/rustac-py/commit/793224b74902b68cbcc485b7e77e2b434347ae17))
* its-live case study ([#144](https://github.com/stac-utils/rustac-py/issues/144)) ([769b84e](https://github.com/stac-utils/rustac-py/commit/769b84ecf6b5f61a4f4f68f02fa594932764f1a5))
* move bundled just to wheels ([#22](https://github.com/stac-utils/rustac-py/issues/22)) ([710780d](https://github.com/stac-utils/rustac-py/commit/710780d4f536affa767f5c0a72c7925524d56fde))
* move in search ([#81](https://github.com/stac-utils/rustac-py/issues/81)) ([87cbdec](https://github.com/stac-utils/rustac-py/commit/87cbdec203488be888b08dea942f00d91894d4ee))
* normalize datetimes when searching ([#202](https://github.com/stac-utils/rustac-py/issues/202)) ([a1b5660](https://github.com/stac-utils/rustac-py/commit/a1b5660432886f6448cb15981be11fe4a5f02c32))
* parquet stream to object store ([#182](https://github.com/stac-utils/rustac-py/issues/182)) ([92c841a](https://github.com/stac-utils/rustac-py/commit/92c841aee84ba1628474a86c01a8ad2aa717c2d3))
* reduce size of release build ([#60](https://github.com/stac-utils/rustac-py/issues/60)) ([5394439](https://github.com/stac-utils/rustac-py/commit/5394439b601ec5affa0bbce1998e8e9db4d1f563))
* refactor ([#18](https://github.com/stac-utils/rustac-py/issues/18)) ([6c82934](https://github.com/stac-utils/rustac-py/commit/6c8293409f1800482b2b43a508d58e6387207f20))
* search to an arrow table ([#54](https://github.com/stac-utils/rustac-py/issues/54)) ([c051c9f](https://github.com/stac-utils/rustac-py/commit/c051c9fa42d4f3aae9164f84a670a024238db10f))
* sepearate parquet compression ([#150](https://github.com/stac-utils/rustac-py/issues/150)) ([a521204](https://github.com/stac-utils/rustac-py/commit/a52120484e8da7d1a98932fe1e3dfa761c2b89c5)), closes [#149](https://github.com/stac-utils/rustac-py/issues/149)
* set python logging ([#70](https://github.com/stac-utils/rustac-py/issues/70)) ([b9260a1](https://github.com/stac-utils/rustac-py/commit/b9260a1c3392d3821e05a6024bfb038cc5bf1af8))
* simplify docs build ([4a6f511](https://github.com/stac-utils/rustac-py/commit/4a6f51187aa4519b02982aa3a2e5df40b2f4af62))
* streaming geoparquet writes ([#178](https://github.com/stac-utils/rustac-py/issues/178)) ([6da4943](https://github.com/stac-utils/rustac-py/commit/6da4943f57138267a22d9e679b85bb0910870fbf))
* union by name when searching ([#152](https://github.com/stac-utils/rustac-py/issues/152)) ([ad33f39](https://github.com/stac-utils/rustac-py/commit/ad33f395d9b4aa1b9b91f5a6ef368aa6686d7871))
* update dependencies ([#10](https://github.com/stac-utils/rustac-py/issues/10)) ([ccdc103](https://github.com/stac-utils/rustac-py/commit/ccdc1036670160aace1a8d9c1a5be735e09a3f6a))
* update gadomski/stacrs to stac-utils/stacrs ([14d8514](https://github.com/stac-utils/rustac-py/commit/14d8514106911d535d8a9e0f716a65a961fd6bab))
* update logo attribution ([9b9b47f](https://github.com/stac-utils/rustac-py/commit/9b9b47f0b6c8f63ce75740034682a6b414773708))
* update readme w/ pronunciation ([640d6d6](https://github.com/stac-utils/rustac-py/commit/640d6d67bed3e9b8e244ae5de5128c13f3dc56d1))
* update rust deps ([f4390a9](https://github.com/stac-utils/rustac-py/commit/f4390a9d14639d72d2ca14931a89d4dfb6fb4e33))
* update versions and docs ([e6d6318](https://github.com/stac-utils/rustac-py/commit/e6d631888659049f66bf42ccc952d910ce21aa65))
* use stac-api's new python feature ([#6](https://github.com/stac-utils/rustac-py/issues/6)) ([776e5e4](https://github.com/stac-utils/rustac-py/commit/776e5e4090b9ef78326147670e1eb88ffa0076f7))
* walk ([#69](https://github.com/stac-utils/rustac-py/issues/69)) ([acc8f8c](https://github.com/stac-utils/rustac-py/commit/acc8f8cc8c248b3b2e045f1e68c01c73c9831c49))


### Bug Fixes

* actually update version number ([#89](https://github.com/stac-utils/rustac-py/issues/89)) ([c7d2e78](https://github.com/stac-utils/rustac-py/commit/c7d2e78682882c38e54efff996d8e4152e1fe04d))
* add environment to pypi publish ([270b094](https://github.com/stac-utils/rustac-py/commit/270b09442ea0f786502ee67678a6331bad8ee290))
* add self argument to duckdb search ([1530e4a](https://github.com/stac-utils/rustac-py/commit/1530e4a37d3725ab97c9ad603fc7a67d5adc4b9d))
* another ci fix attempt ([4727127](https://github.com/stac-utils/rustac-py/commit/4727127a1e020425a0c723c3a875137e8239a07c))
* badges ([3492065](https://github.com/stac-utils/rustac-py/commit/349206520e036865515b95db270cb8ee6e3198c4))
* build docs on pushes to gh-pages ([2e91c97](https://github.com/stac-utils/rustac-py/commit/2e91c97a94c185d1fa3f242ab19ec21c2f732f4b))
* circular import ([#186](https://github.com/stac-utils/rustac-py/issues/186)) ([d3b46b3](https://github.com/stac-utils/rustac-py/commit/d3b46b3fee8665f9e122c83431509f0ed0860399)), closes [#184](https://github.com/stac-utils/rustac-py/issues/184)
* **ci:** time-piece ([#180](https://github.com/stac-utils/rustac-py/issues/180)) ([3a4b945](https://github.com/stac-utils/rustac-py/commit/3a4b945c9968380210f8fc114dd0717eab992bcb))
* **ci:** update aliases on docs ([869cd20](https://github.com/stac-utils/rustac-py/commit/869cd20d01a8eaf860d4ae422c8a16a78c336004))
* clean up docs, remove migrate_href ([#78](https://github.com/stac-utils/rustac-py/issues/78)) ([d968382](https://github.com/stac-utils/rustac-py/commit/d9683825fa85b293ff29f61faf09bf3c4fce0b5c))
* CoC symlink ([954542a](https://github.com/stac-utils/rustac-py/commit/954542ab260d0f0093d417d9a1b776f0e210cadb))
* deconstruct item collection when writing ndjson ([#167](https://github.com/stac-utils/rustac-py/issues/167)) ([74d9544](https://github.com/stac-utils/rustac-py/commit/74d9544cc432f6a52e6b6db533bdd945c2742120))
* doc links ([7588531](https://github.com/stac-utils/rustac-py/commit/7588531840ca1435ceae96cc20b2ebc0f99183e3))
* docs build ([#56](https://github.com/stac-utils/rustac-py/issues/56)) ([30533c3](https://github.com/stac-utils/rustac-py/commit/30533c33ca5eac15a17095d893fedc414ad84531))
* docstring ([3720c4a](https://github.com/stac-utils/rustac-py/commit/3720c4affa4981e3fe71794001af0bd7c42d7c9d))
* docstring for write_geoparquet ([#185](https://github.com/stac-utils/rustac-py/issues/185)) ([fc82961](https://github.com/stac-utils/rustac-py/commit/fc82961bdaf5eb893bc04a24f27e4202a4cf4e53))
* documentation link ([28ec72b](https://github.com/stac-utils/rustac-py/commit/28ec72b9f96894ecce519a696e5b3ff8ddf3f327))
* don't cancel in progress ([dcdd207](https://github.com/stac-utils/rustac-py/commit/dcdd2070f366e1675c378f80a5560f759838dd17))
* don't publish stacrs ([5f9c370](https://github.com/stac-utils/rustac-py/commit/5f9c370a47ef94af4e555e658f28075f92ef6786))
* empty datetime intervals ([#111](https://github.com/stac-utils/rustac-py/issues/111)) ([c209522](https://github.com/stac-utils/rustac-py/commit/c209522788f32278ce72b8f9816ffc10d1b05331))
* example notebook link ([91a8ddf](https://github.com/stac-utils/rustac-py/commit/91a8ddfc587e2aad898e63263fe1667d563e195c))
* formatting ([38f2b8b](https://github.com/stac-utils/rustac-py/commit/38f2b8bc9846c45890fe5801cffdf1cb8e8db4f9))
* heading ([fe2e333](https://github.com/stac-utils/rustac-py/commit/fe2e3334e4f8ec64b38f2c923b4753fe002cf686))
* include type field in geoparquet write ([#136](https://github.com/stac-utils/rustac-py/issues/136)) ([be985a6](https://github.com/stac-utils/rustac-py/commit/be985a60e22a24e98e6709f2aaff1d98192b9e3a))
* install duckdb for docs ([#27](https://github.com/stac-utils/rustac-py/issues/27)) ([d2046b0](https://github.com/stac-utils/rustac-py/commit/d2046b0a30870346b22c37a81d1f0c7c9b4770f8))
* install openssl when building wheels ([#169](https://github.com/stac-utils/rustac-py/issues/169)) ([ed0523b](https://github.com/stac-utils/rustac-py/commit/ed0523b4e9f726257eb271adb485c4ceaeb37399))
* issues link ([f689415](https://github.com/stac-utils/rustac-py/commit/f689415875a477572a6c2df0e93b1a6c5607e040))
* its live notebook ([9970721](https://github.com/stac-utils/rustac-py/commit/997072100c09301f42ef1949aabd59e2cb5e8eb6))
* its-live notebook update ([#145](https://github.com/stac-utils/rustac-py/issues/145)) ([f7a45fa](https://github.com/stac-utils/rustac-py/commit/f7a45fa6dc7830b25ec7f77456a8b37c40ddd3ee))
* normalize search output ([#102](https://github.com/stac-utils/rustac-py/issues/102)) ([9525f9d](https://github.com/stac-utils/rustac-py/commit/9525f9dde70ff8252c6ef331483c23e8597613fd)), closes [#66](https://github.com/stac-utils/rustac-py/issues/66)
* object store typing ([#143](https://github.com/stac-utils/rustac-py/issues/143)) ([906a0a0](https://github.com/stac-utils/rustac-py/commit/906a0a0d156f1b08a8dfc67ad9e2c728da3a44c8))
* proj:geometry ([#120](https://github.com/stac-utils/rustac-py/issues/120)) ([0516f9d](https://github.com/stac-utils/rustac-py/commit/0516f9d34b9ffda2d45a3ccda01c7f78958c3ba2))
* re-add environment to CI ([853c898](https://github.com/stac-utils/rustac-py/commit/853c898c2fe29a29d9638575e5d0d6de1bd72f0d))
* re-add sdist ([#92](https://github.com/stac-utils/rustac-py/issues/92)) ([f1e8b08](https://github.com/stac-utils/rustac-py/commit/f1e8b086d0239d9046caaccc6d6b93dfdbe53308))
* read proj geometry ([#125](https://github.com/stac-utils/rustac-py/issues/125)) ([8a6430c](https://github.com/stac-utils/rustac-py/commit/8a6430ccf1251cb8dc518870bb6c4bcfe07a0faa)), closes [#124](https://github.com/stac-utils/rustac-py/issues/124)
* remove conda badge for now ([c07ab57](https://github.com/stac-utils/rustac-py/commit/c07ab5786a1cdb3fd5fd970e4ce58e7291fcc3ac))
* remove docs optional dependency ([c0b76ae](https://github.com/stac-utils/rustac-py/commit/c0b76aef871c28a445b51f426fff61c80b002182))
* remove s390x from pypi build ([#165](https://github.com/stac-utils/rustac-py/issues/165)) ([fd75e69](https://github.com/stac-utils/rustac-py/commit/fd75e690537b3933bfe32f69b1bc6f84461be730))
* remove sdist ([d6e4356](https://github.com/stac-utils/rustac-py/commit/d6e4356ed85a6caa0fda631cfbcca34f57fb06f6))
* remove sdist from pypi ([e970882](https://github.com/stac-utils/rustac-py/commit/e97088200ce52bb9f8e4c6d802f0dd04ae2067ae))
* remove tracing subscriber ([#49](https://github.com/stac-utils/rustac-py/issues/49)) ([67058d6](https://github.com/stac-utils/rustac-py/commit/67058d6c788450e50e0911364100e62cf5fa2450))
* search ([#122](https://github.com/stac-utils/rustac-py/issues/122)) ([8801a52](https://github.com/stac-utils/rustac-py/commit/8801a521fd5dd870fe8f74046df627696afbf354)), closes [#118](https://github.com/stac-utils/rustac-py/issues/118)
* set preserve order for serde ([#93](https://github.com/stac-utils/rustac-py/issues/93)) ([f40236c](https://github.com/stac-utils/rustac-py/commit/f40236c2e83b5358871020625c20042552dcb62f))
* spelling ([bf45f34](https://github.com/stac-utils/rustac-py/commit/bf45f34cced49bf0afaf5f39a54d26839d884bfe))
* split cargo dependencies ([#195](https://github.com/stac-utils/rustac-py/issues/195)) ([14919bb](https://github.com/stac-utils/rustac-py/commit/14919bb981edeaffdd28b9580dc8e09bb9ff5196))
* swallow broken pipe errors ([#73](https://github.com/stac-utils/rustac-py/issues/73)) ([e71a9e2](https://github.com/stac-utils/rustac-py/commit/e71a9e25b0b2d18bebaa1f25f5d618799b1f8375))
* typing ([#110](https://github.com/stac-utils/rustac-py/issues/110)) ([953515c](https://github.com/stac-utils/rustac-py/commit/953515c3646a4df4f07f86ac08e784b0be20c947))
* typing for DuckdbClient.search ([7bdbf0e](https://github.com/stac-utils/rustac-py/commit/7bdbf0e5de89c096d990861d41f29a4e25f42fe0))
* update changelog w/ remove ([f7e49fc](https://github.com/stac-utils/rustac-py/commit/f7e49fc308bdcb0c193a38f9e52184a5e6345781))
* update lockfile ([6cbb699](https://github.com/stac-utils/rustac-py/commit/6cbb699d2f6f53ef3228086eca2ad7c2e3baff4c))
* use uv to help build wheels ([aec6f48](https://github.com/stac-utils/rustac-py/commit/aec6f48965819f3fe170a037aad7bb71ad3bb468))
* uv run mike ([324b14e](https://github.com/stac-utils/rustac-py/commit/324b14ede5ad585e51ca3144ea740ed07e7e7883))
* vendor openssl ([#4](https://github.com/stac-utils/rustac-py/issues/4)) ([0db165d](https://github.com/stac-utils/rustac-py/commit/0db165d0002295e273f5ed823329d23df958393e))
* wheels ([#154](https://github.com/stac-utils/rustac-py/issues/154)) ([1d1991e](https://github.com/stac-utils/rustac-py/commit/1d1991e2a20b40daab56ebf857df6e1df6c9c8c9))
* yolo release workflow change ([26785f2](https://github.com/stac-utils/rustac-py/commit/26785f220c48f2e6f98c27e3e133e938ca154c9e))
* yolo, simplify release? ([9bad78a](https://github.com/stac-utils/rustac-py/commit/9bad78a1e5a54a97c281d2d1a8c1e197af183271))


### Dependencies

* bump a few ([#190](https://github.com/stac-utils/rustac-py/issues/190)) ([f2c6816](https://github.com/stac-utils/rustac-py/commit/f2c6816649465c568b5313729b61bb1bfd42ee61))
* cargo update ([4cd41eb](https://github.com/stac-utils/rustac-py/commit/4cd41ebfec1d03197dee481e7c7c8454d8904333))
* patch duckdb ([d688f0b](https://github.com/stac-utils/rustac-py/commit/d688f0bfbdc172c5bbce2bf59f984420eb8dc240))
* update ([22c8831](https://github.com/stac-utils/rustac-py/commit/22c883178cb799032c0928eeaf82463b38f6bd7a))
* update ([231bd72](https://github.com/stac-utils/rustac-py/commit/231bd72e54b0e81a2230429a22151da50fb49048))
* update ([d506991](https://github.com/stac-utils/rustac-py/commit/d50699116f20038cd1d86b65b18f4c3dd5641910))
* update ([4be8b70](https://github.com/stac-utils/rustac-py/commit/4be8b700470cd0e24f6decf174d3d5e4e6128664))
* update cargo lock ([f0e83e7](https://github.com/stac-utils/rustac-py/commit/f0e83e75d0d6f10f7d7e6f51d8b6f8f20074ceea))
* update rustac ([#141](https://github.com/stac-utils/rustac-py/issues/141)) ([573126b](https://github.com/stac-utils/rustac-py/commit/573126b737020909203062da3dbc43554c59860a)), closes [#140](https://github.com/stac-utils/rustac-py/issues/140)
* update stac-cli ([e471117](https://github.com/stac-utils/rustac-py/commit/e4711179018da5f3efa1a9cfbcd565552364b1d9))


### Documentation

* a little doc and devex fixups ([#98](https://github.com/stac-utils/rustac-py/issues/98)) ([89fbd85](https://github.com/stac-utils/rustac-py/commit/89fbd857d36066dc815c69f8fc29dc35da615c0c))
* add awaits to readme ([c22cb55](https://github.com/stac-utils/rustac-py/commit/c22cb553ab4781bf6a9d0117c12bb4cb52d4c1fa))
* add badge ([1528d7e](https://github.com/stac-utils/rustac-py/commit/1528d7e1b7185a86efe9fc7c42b0620093c5e9c6))
* add conda forge badge ([edd7501](https://github.com/stac-utils/rustac-py/commit/edd75012fd19b53ed2cf4aa1a379f5d42dd0c8f5))
* add more output for cli ([202a0de](https://github.com/stac-utils/rustac-py/commit/202a0deee42f22438c8b5c71cd81683c58012d27))
* add note about CLI ([f6a0ea0](https://github.com/stac-utils/rustac-py/commit/f6a0ea098bb9b1d641756c3f455e861db02ab4b2))
* add note about off-cycle doc pushes ([e07ce54](https://github.com/stac-utils/rustac-py/commit/e07ce549ae29507245569bb64e13e1de041e7cce))
* add performance check for search ([#30](https://github.com/stac-utils/rustac-py/issues/30)) ([7387bc7](https://github.com/stac-utils/rustac-py/commit/7387bc77ad7ae4fdff08cbcb742a8e80bb2bcf1f))
* add small note ([9607ea4](https://github.com/stac-utils/rustac-py/commit/9607ea402c488cd4bb75d985b01d7ebe4e7b54ec))
* add store example ([#128](https://github.com/stac-utils/rustac-py/issues/128)) ([082a194](https://github.com/stac-utils/rustac-py/commit/082a194d116484877302829e32458e9de06e3140))
* async iteration notebook ([#155](https://github.com/stac-utils/rustac-py/issues/155)) ([d866853](https://github.com/stac-utils/rustac-py/commit/d866853f809d768a5d1ea517dd10d8d6fb463013))
* describe formats ([#75](https://github.com/stac-utils/rustac-py/issues/75)) ([edd7a49](https://github.com/stac-utils/rustac-py/commit/edd7a49da3280438c303477281755fdcbfa5e141))
* fix CI badge ([eb1656b](https://github.com/stac-utils/rustac-py/commit/eb1656b5e9e6079f5e0271c9843b5abca44be0be))
* give write permissions ([096f2a7](https://github.com/stac-utils/rustac-py/commit/096f2a78615e58c58aebeb6843db330994887f52))
* logo ([d058ad1](https://github.com/stac-utils/rustac-py/commit/d058ad1f61095758f0f239af26ae4ba36d92980f))
* short note about searching geoparquet ([2ff8601](https://github.com/stac-utils/rustac-py/commit/2ff8601c0396a3bc0e1024b7ab6c925e358c2e34))
* some light readme work ([#94](https://github.com/stac-utils/rustac-py/issues/94)) ([03381d8](https://github.com/stac-utils/rustac-py/commit/03381d889819f984cf9190221b344f1ddb454e52))
* switch back to mkdocs-jupyter, add stac-geoparquet ([#142](https://github.com/stac-utils/rustac-py/issues/142)) ([0656a5f](https://github.com/stac-utils/rustac-py/commit/0656a5f0c802270d36fdeb2f68c79c7e4a7175a1)), closes [#139](https://github.com/stac-utils/rustac-py/issues/139)
* update README ([bab05a2](https://github.com/stac-utils/rustac-py/commit/bab05a24ce27c8e2721110ac628f7bfeb2505b85))
* update readme and docs landing page ([#114](https://github.com/stac-utils/rustac-py/issues/114)) ([0964fb5](https://github.com/stac-utils/rustac-py/commit/0964fb552d7a251c34eeaf1a93e8e6745a565215))


### Code Refactoring

* remove the typed dicts ([#177](https://github.com/stac-utils/rustac-py/issues/177)) ([1800edd](https://github.com/stac-utils/rustac-py/commit/1800eddc13ca6872b54cd4b08db423cda8c3e9f6))

## [0.10.0](https://github.com/stac-utils/rustac-py/compare/v0.9.2...v0.10.0) (2026-01-06)


### Features

* add collections to parquet metadata ([#208](https://github.com/stac-utils/rustac-py/issues/208)) ([617ecca](https://github.com/stac-utils/rustac-py/commit/617ecca449c187b1d2bda4e49294f82f48d439e1))
* normalize datetimes when searching ([#202](https://github.com/stac-utils/rustac-py/issues/202)) ([a1b5660](https://github.com/stac-utils/rustac-py/commit/a1b5660432886f6448cb15981be11fe4a5f02c32))


### Bug Fixes

* split cargo dependencies ([#195](https://github.com/stac-utils/rustac-py/issues/195)) ([14919bb](https://github.com/stac-utils/rustac-py/commit/14919bb981edeaffdd28b9580dc8e09bb9ff5196))


### Dependencies

* bump a few ([#190](https://github.com/stac-utils/rustac-py/issues/190)) ([f2c6816](https://github.com/stac-utils/rustac-py/commit/f2c6816649465c568b5313729b61bb1bfd42ee61))

## [Unreleased]

## [0.9.2]

### Fixed

- Circular import issue ([#186](https://github.com/stac-utils/rustac-py/pull/186))

## [0.9.1]

### Added

- Write **stac-geoparquet** to an object store ([#182](https://github.com/stac-utils/rustac-py/pull/182))

## [0.9.0]

### Added

- Chunked stac-geoparquet writing ([#178](https://github.com/stac-utils/rustac-py/pull/178))

### Removed

- STAC typed dicts ([#177](https://github.com/stac-utils/rustac-py/pull/177))

## [0.8.4] - 2025-10-22

### Fixed

- Deconstruct item collections when writing ndjson ([#167](https://github.com/stac-utils/rustac-py/pull/167))

## [0.8.3] - 2025-09-24

### Changed

- Don't set geo metadata for `proj:geometry` ([rustac #808](https://github.com/stac-utils/rustac/pull/808), [#161](https://github.com/stac-utils/rustac-py/pull/161))

## [0.8.2] - 2025-09-15

Bump **pyo3** version.

## [0.8.1] - 2025-06-16

### Added

- `type` field to geoparquet writes ([#136](https://github.com/stac-utils/rustac-py/pull/136), <https://github.com/stac-utils/rustac/pull/736>)
- `parquet_compression` argument to `write` and `search_to` ([#150](https://github.com/stac-utils/rustac-py/pull/150))
- `iter_search` ([#151](https://github.com/stac-utils/rustac-py/pull/151))
- `union_by_name` when searching **stac-geoparquet** ([#152](https://github.com/stac-utils/rustac-py/pull/152))

### Fixed

- Error instead of panic for cql ([#138](https://github.com/stac-utils/rustac-py/pull/138), <https://github.com/developmentseed/cql2-rs/pull/83>)

## [0.8.0] - 2025-05-13

### Added

- `rustac.store` ([#127](https://github.com/stac-utils/rustac-py/pull/127))
- More linux wheels ([#132](https://github.com/stac-utils/rustac-py/pull/132))

### Removed

- `options` from `read`, `write`, and `search_to` ([#127](https://github.com/stac-utils/rustac-py/pull/127), [#130](https://github.com/stac-utils/rustac-py/pull/130))

## [0.7.2] - 2025-05-05

### Fixed

- Search ([#122](https://github.com/stac-utils/rustac-py/pull/122))
- Reading `proj:geometry` (and other geometries) ([#125](https://github.com/stac-utils/rustac-py/pull/125))

## [0.7.1] - 2025-05-02

### Fixed

- `proj:geometry` ([#120](https://github.com/stac-utils/rustac-py/pull/120))

## [0.7.0] - 2025-04-29

### Added

- Source distribution to PyPI publish ([#92](https://github.com/stac-utils/rustac-py/pull/92))
- `rustac.sha` ([#99](https://github.com/stac-utils/rustac-py/pull/99))
- Typed dictionaries for STAC entities ([#101](https://github.com/stac-utils/rustac-py/pull/101))
- `rustac.collection_from_id_and_items` ([#109](https://github.com/stac-utils/rustac-py/pull/109))

### Fixed

- Deterministic asset ordering ([rustac #709](https://github.com/stac-utils/rustac/pull/709), [#93](https://github.com/stac-utils/rustac-py/pull/93))
- Normalize search output ([#102](https://github.com/stac-utils/rustac-py/pull/102))

### Removed

- Python 3.10 support ([#110](https://github.com/stac-utils/rustac-py/pull/110))

## [0.6.0] - 2025-04-18

> [!NOTE]
> This package was renamed from **stacrs** to **rustac**.

### Added

- Construct `stac_api::Search` (moved from `stac_api` crate) ([#81](https://github.com/stac-utils/rustac-py/pull/81))

### Fixed

- Swallow broken pipe errors ([#73](https://github.com/stac-utils/rustac-py/pull/73))
- Clean up docs ([#78](https://github.com/stac-utils/rustac-py/pull/78))

### Removed

- `migrate_href` ([#78](https://github.com/stac-utils/rustac-py/pull/78))

## [0.5.9] - 2025-03-03

### Added

- `walk` and `set_self_link` for `read` ([#69](https://github.com/stac-utils/rustac-py/pull/69))

## [0.5.8] - 2025-02-27

### Fixed

- Patch DuckDB ([#64](https://github.com/stac-utils/rustac-py/pull/64))

## [0.5.7] - 2025-02-26

### Changed

- Don't include libduckdb, but rather build bundled to save size ([#61](https://github.com/stac-utils/rustac-py/pull/61))

## [0.5.6] - 2025-02-26

### Added

- Search to an arrow table ([#54](https://github.com/stac-utils/rustac-py/pull/54))
- Create a item collection from an arrow table ([#57](https://github.com/stac-utils/rustac-py/pull/57))

### Changed

- Include **libduckdb** in wheels ([#52](https://github.com/stac-utils/rustac-py/pull/52))

## [0.5.5] - 2025-02-20

### Fixed

- Removed tracing subscriber to fix CLI ([#49](https://github.com/stac-utils/rustac-py/pull/49))

## [0.5.4] - 2025-02-19

### Added

- CLI ([#46](https://github.com/stac-utils/rustac-py/pull/46))
- Config args to DuckDB client ([#42](https://github.com/stac-utils/rustac-py/pull/42))

## [0.5.3] - 2025-02-07

### Changed

- Use only abi3 wheels ([#36](https://github.com/gadomski/rustac-py/pull/36))

> [!WARNING]
> All versions older than v0.5.3 were deleted from PyPI, but some tags still exist on this repo.
> See <https://github.com/gadomski/rustac-py/discussions/37> for more.

## [0.5.2] - 2025-02-07

### Changed

- Bundle by default ([#32](https://github.com/gadomski/rustac-py/pull/32))

## [0.5.1] - 2025-02-07

### Added

- More wheels ([#28](https://github.com/gadomski/rustac-py/pull/28))

## [0.5.0] - 2025-02-06

### Changed

- `search` and `search_to` are now async ([#24](https://github.com/gadomski/rustac-py/pull/24))

## [0.4.0] - 2025-01-13

### Added

- DuckDB client ([#15](https://github.com/gadomski/rustac-py/pull/15))

### Changed

- `read` and `write` are now async ([#18](https://github.com/gadomski/rustac-py/pull/18))

## [0.3.0] - 2024-11-21

### Removed

- Validation, pending <https://github.com/stac-utils/stac-rs/issues/517>

### Changed

- Moved out of the <https://github.com/stac-utils/stac-rs> into <https://github.com/gadomski/rustac-py>

## [0.2.2] - 2024-10-22

### Added

- Send user agent when searching ([#487](https://github.com/stac-utils/stac-rs/pull/487))

## [0.2.1] - 2024-10-21

### Added

- More wheels ([#481](https://github.com/stac-utils/stac-rs/pull/481))

## [0.2.0] - 2024-10-19

### Added

- `version` ([#476](https://github.com/stac-utils/stac-rs/pull/476))

### Changed

- Moved docstrings to stub file ([#468](https://github.com/stac-utils/stac-rs/pull/468))

### Removed

- `pystac` ([#468](https://github.com/stac-utils/stac-rs/pull/468))

## [0.1.3] - 2024-10-17

### Added

- Experimental DuckDB 🦆 search on **stac-geoparquet** files ([#458](https://github.com/stac-utils/stac-rs/pull/458))

## [0.1.2] - 2024-09-22

### Changed

- Return the item count from `search_to` ([#426](https://github.com/stac-utils/stac-rs/pull/426))

## [0.1.1] - 2024-09-21

### Added

- Extension module feature

### Changed

- Use Github Pages for docs ([#420](https://github.com/stac-utils/stac-rs/pull/420))

## [0.1.0] - 2024-09-20

### Added

- `migrate_href` ([#334](https://github.com/stac-utils/stac-rs/pull/334))
- `search` and `search_to` ([#387](https://github.com/stac-utils/stac-rs/pull/387))
- `read`, `write`, and `pystac` ([#418](https://github.com/stac-utils/stac-rs/pull/418))

## [0.0.3] - 2024-08-29

### Added

- `migrate` ([#309](https://github.com/stac-utils/stac-rs/pull/309))
- `validate` and docs ([#307](https://github.com/stac-utils/stac-rs/pull/307))

## [0.0.2] - 2024-08-28

Non-functional release to fix releasing from Github actions.

## [0.0.1] - 2024-08-28

Initial release.

[Unreleased]: https://github.com/stac-utils/rustac-py/compare/v0.9.2...main
[0.9.2]: https://github.com/stac-utils/rustac-py/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/stac-utils/rustac-py/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/stac-utils/rustac-py/compare/v0.8.4...v0.9.0
[0.8.4]: https://github.com/stac-utils/rustac-py/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/stac-utils/rustac-py/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/stac-utils/rustac-py/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/stac-utils/rustac-py/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/stac-utils/rustac-py/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/stac-utils/rustac-py/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/stac-utils/rustac-py/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/stac-utils/rustac-py/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/stac-utils/rustac-py/compare/v0.5.9...v0.6.0
[0.5.9]: https://github.com/stac-utils/rustac-py/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/stac-utils/rustac-py/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/stac-utils/rustac-py/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/stac-utils/rustac-py/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/stac-utils/rustac-py/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/stac-utils/rustac-py/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/stac-utils/rustac-py/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/stac-utils/rustac-py/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/stac-utils/rustac-py/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/stac-utils/rustac-py/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/stac-utils/rustac-py/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/stac-utils/rustac-py/releases/tag/v0.3.0
[0.2.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.2.1...python-v0.2.2
[0.2.1]: https://github.com/stac-utils/stac-rs/compare/python-v0.2.0...python-v0.2.1
[0.2.0]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.3...python-v0.2.0
[0.1.3]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.2...python-v0.1.3
[0.1.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.1...python-v0.1.2
[0.1.1]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.0...python-v0.1.1
[0.1.0]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.3...python-v0.1.0
[0.0.3]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.2...python-v0.0.3
[0.0.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.1...python-v0.0.2
[0.0.1]: https://github.com/stac-utils/stac-rs/releases/tag/python-v0.0.1

<!-- markdownlint-disable-file MD024 -->
