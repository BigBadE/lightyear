# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.21.0 (2025-07-03)

<csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/>
<csr-id-1abda441054255978b6d5bef9da8e538b91aa1ed/>
<csr-id-81341e91707b31a5cba6967d23e230945180a4e8/>
<csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/>
<csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/>
<csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/>
<csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/>
<csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/>
<csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/>
<csr-id-249b40f358977f6f85e269967d3912bfb4080f73/>
<csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/>
<csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/>
<csr-id-411733089f59eb90d405f7ad327b5440b55ef060/>
<csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/>
<csr-id-307f947d03668b272beab1a4285dc9cc3463b867/>
<csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/>
<csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/>
<csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/>
<csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/>

### Chore

 - <csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/> release rc3
 - <csr-id-1abda441054255978b6d5bef9da8e538b91aa1ed/> separate avian into 2 crates, fix delta-compression example, fix book links
 - <csr-id-81341e91707b31a5cba6967d23e230945180a4e8/> release 0.21 rc 2
 - <csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/> add tests for delta-compression
 - <csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/> fix compiletime benchmark
 - <csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/> fix ci
 - <csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/> fix
 - <csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/> cargo fmt
 - <csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/> fix tests, cargo doc, cargo clippy
 - <csr-id-249b40f358977f6f85e269967d3912bfb4080f73/> fix clippy
 - <csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/> cargo fmt
 - <csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/> enable host-server for all examples
 - <csr-id-411733089f59eb90d405f7ad327b5440b55ef060/> enable host-client mode on simple box
 - <csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/> fix warns
 - <csr-id-307f947d03668b272beab1a4285dc9cc3463b867/> fix typo in docs

### Chore

 - <csr-id-7fe5e08d715fa55ad003270be95139b003aca396/> release 0.21.0

### Documentation

 - <csr-id-f4985d9f1c6c3fec718f11925060448f22c8be93/> fix typos, update ring

### New Features

 - <csr-id-0bd3fbe9db6d8dfd350a0e014e7beec9392df1de/> enable steam on simple_box example and fix wasm
 - <csr-id-117b0841a25dba5c6ffaadad88a8c4dba09d3cbb/> support BEI inputs
 - <csr-id-d11f597140a81b73656dabb585e2ecca62fd208b/> implement wasm & fix small mistakes
 - <csr-id-7781029b8c17f1558400b7095cedeb85c6f269c9/> add non-working native ws impl

### Bug Fixes

 - <csr-id-7d9dbbf435e94e1bc85e631a1df76951150f5aad/> register prespawned entity in predicted_entity_map during server/client match
 - <csr-id-ae2f4b2a5caf60eabbbd83877a5c5c8a3486588e/> Remove `ring` to fix wasm32 web builds
 - <csr-id-b77f2eeb5e7751016e9a981407710c60a9c75c88/> expose ReplicateToServer
 - <csr-id-86f20c79f6930d19ecc3cbf5b97a7e36b6b5b7a7/> Add try_from_bytes method to ConnectToken
 - <csr-id-f96c5fa8fcaabb61d884baf1ee9ce6d6f37d5322/> sending now after opening socket and uncomment packetsender impl code
 - <csr-id-c3f1faf98d770390323f1821861bf80e3e9d0415/> use io task pool instead of tokio::spawn and change receiver / sender
 - <csr-id-2293aed17794c1a2017a78145258bdf9a84bca6a/> recv impl

### Other

 - <csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/> add option to trigger change detection
   * add option to trigger change detection
   
   * fix bug
 - <csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/> add close method to packet sender / packet receiver
 - <csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/> small ownership issues
 - <csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/> add unit tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1273 commits contributed to the release.
 - 32 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 209 unique issues were worked on: [#1001](https://github.com/cBournhonesque/lightyear/issues/1001), [#1003](https://github.com/cBournhonesque/lightyear/issues/1003), [#1004](https://github.com/cBournhonesque/lightyear/issues/1004), [#1006](https://github.com/cBournhonesque/lightyear/issues/1006), [#1007](https://github.com/cBournhonesque/lightyear/issues/1007), [#1008](https://github.com/cBournhonesque/lightyear/issues/1008), [#1017](https://github.com/cBournhonesque/lightyear/issues/1017), [#1018](https://github.com/cBournhonesque/lightyear/issues/1018), [#1019](https://github.com/cBournhonesque/lightyear/issues/1019), [#1021](https://github.com/cBournhonesque/lightyear/issues/1021), [#1023](https://github.com/cBournhonesque/lightyear/issues/1023), [#1024](https://github.com/cBournhonesque/lightyear/issues/1024), [#1029](https://github.com/cBournhonesque/lightyear/issues/1029), [#1039](https://github.com/cBournhonesque/lightyear/issues/1039), [#1043](https://github.com/cBournhonesque/lightyear/issues/1043), [#1049](https://github.com/cBournhonesque/lightyear/issues/1049), [#1051](https://github.com/cBournhonesque/lightyear/issues/1051), [#1054](https://github.com/cBournhonesque/lightyear/issues/1054), [#1055](https://github.com/cBournhonesque/lightyear/issues/1055), [#1058](https://github.com/cBournhonesque/lightyear/issues/1058), [#1061](https://github.com/cBournhonesque/lightyear/issues/1061), [#1062](https://github.com/cBournhonesque/lightyear/issues/1062), [#464](https://github.com/cBournhonesque/lightyear/issues/464), [#512](https://github.com/cBournhonesque/lightyear/issues/512), [#513](https://github.com/cBournhonesque/lightyear/issues/513), [#515](https://github.com/cBournhonesque/lightyear/issues/515), [#523](https://github.com/cBournhonesque/lightyear/issues/523), [#525](https://github.com/cBournhonesque/lightyear/issues/525), [#528](https://github.com/cBournhonesque/lightyear/issues/528), [#530](https://github.com/cBournhonesque/lightyear/issues/530), [#531](https://github.com/cBournhonesque/lightyear/issues/531), [#534](https://github.com/cBournhonesque/lightyear/issues/534), [#537](https://github.com/cBournhonesque/lightyear/issues/537), [#538](https://github.com/cBournhonesque/lightyear/issues/538), [#542](https://github.com/cBournhonesque/lightyear/issues/542), [#543](https://github.com/cBournhonesque/lightyear/issues/543), [#544](https://github.com/cBournhonesque/lightyear/issues/544), [#548](https://github.com/cBournhonesque/lightyear/issues/548), [#549](https://github.com/cBournhonesque/lightyear/issues/549), [#554](https://github.com/cBournhonesque/lightyear/issues/554), [#555](https://github.com/cBournhonesque/lightyear/issues/555), [#556](https://github.com/cBournhonesque/lightyear/issues/556), [#557](https://github.com/cBournhonesque/lightyear/issues/557), [#558](https://github.com/cBournhonesque/lightyear/issues/558), [#559](https://github.com/cBournhonesque/lightyear/issues/559), [#561](https://github.com/cBournhonesque/lightyear/issues/561), [#566](https://github.com/cBournhonesque/lightyear/issues/566), [#567](https://github.com/cBournhonesque/lightyear/issues/567), [#570](https://github.com/cBournhonesque/lightyear/issues/570), [#572](https://github.com/cBournhonesque/lightyear/issues/572), [#575](https://github.com/cBournhonesque/lightyear/issues/575), [#578](https://github.com/cBournhonesque/lightyear/issues/578), [#581](https://github.com/cBournhonesque/lightyear/issues/581), [#583](https://github.com/cBournhonesque/lightyear/issues/583), [#588](https://github.com/cBournhonesque/lightyear/issues/588), [#589](https://github.com/cBournhonesque/lightyear/issues/589), [#591](https://github.com/cBournhonesque/lightyear/issues/591), [#592](https://github.com/cBournhonesque/lightyear/issues/592), [#594](https://github.com/cBournhonesque/lightyear/issues/594), [#597](https://github.com/cBournhonesque/lightyear/issues/597), [#598](https://github.com/cBournhonesque/lightyear/issues/598), [#600](https://github.com/cBournhonesque/lightyear/issues/600), [#601](https://github.com/cBournhonesque/lightyear/issues/601), [#602](https://github.com/cBournhonesque/lightyear/issues/602), [#606](https://github.com/cBournhonesque/lightyear/issues/606), [#607](https://github.com/cBournhonesque/lightyear/issues/607), [#612](https://github.com/cBournhonesque/lightyear/issues/612), [#615](https://github.com/cBournhonesque/lightyear/issues/615), [#622](https://github.com/cBournhonesque/lightyear/issues/622), [#623](https://github.com/cBournhonesque/lightyear/issues/623), [#632](https://github.com/cBournhonesque/lightyear/issues/632), [#636](https://github.com/cBournhonesque/lightyear/issues/636), [#638](https://github.com/cBournhonesque/lightyear/issues/638), [#642](https://github.com/cBournhonesque/lightyear/issues/642), [#646](https://github.com/cBournhonesque/lightyear/issues/646), [#647](https://github.com/cBournhonesque/lightyear/issues/647), [#653](https://github.com/cBournhonesque/lightyear/issues/653), [#654](https://github.com/cBournhonesque/lightyear/issues/654), [#656](https://github.com/cBournhonesque/lightyear/issues/656), [#657](https://github.com/cBournhonesque/lightyear/issues/657), [#661](https://github.com/cBournhonesque/lightyear/issues/661), [#665](https://github.com/cBournhonesque/lightyear/issues/665), [#666](https://github.com/cBournhonesque/lightyear/issues/666), [#670](https://github.com/cBournhonesque/lightyear/issues/670), [#671](https://github.com/cBournhonesque/lightyear/issues/671), [#672](https://github.com/cBournhonesque/lightyear/issues/672), [#674](https://github.com/cBournhonesque/lightyear/issues/674), [#675](https://github.com/cBournhonesque/lightyear/issues/675), [#682](https://github.com/cBournhonesque/lightyear/issues/682), [#687](https://github.com/cBournhonesque/lightyear/issues/687), [#693](https://github.com/cBournhonesque/lightyear/issues/693), [#694](https://github.com/cBournhonesque/lightyear/issues/694), [#695](https://github.com/cBournhonesque/lightyear/issues/695), [#698](https://github.com/cBournhonesque/lightyear/issues/698), [#700](https://github.com/cBournhonesque/lightyear/issues/700), [#703](https://github.com/cBournhonesque/lightyear/issues/703), [#705](https://github.com/cBournhonesque/lightyear/issues/705), [#706](https://github.com/cBournhonesque/lightyear/issues/706), [#708](https://github.com/cBournhonesque/lightyear/issues/708), [#709](https://github.com/cBournhonesque/lightyear/issues/709), [#710](https://github.com/cBournhonesque/lightyear/issues/710), [#711](https://github.com/cBournhonesque/lightyear/issues/711), [#713](https://github.com/cBournhonesque/lightyear/issues/713), [#714](https://github.com/cBournhonesque/lightyear/issues/714), [#722](https://github.com/cBournhonesque/lightyear/issues/722), [#723](https://github.com/cBournhonesque/lightyear/issues/723), [#724](https://github.com/cBournhonesque/lightyear/issues/724), [#726](https://github.com/cBournhonesque/lightyear/issues/726), [#732](https://github.com/cBournhonesque/lightyear/issues/732), [#736](https://github.com/cBournhonesque/lightyear/issues/736), [#741](https://github.com/cBournhonesque/lightyear/issues/741), [#746](https://github.com/cBournhonesque/lightyear/issues/746), [#748](https://github.com/cBournhonesque/lightyear/issues/748), [#749](https://github.com/cBournhonesque/lightyear/issues/749), [#754](https://github.com/cBournhonesque/lightyear/issues/754), [#757](https://github.com/cBournhonesque/lightyear/issues/757), [#759](https://github.com/cBournhonesque/lightyear/issues/759), [#761](https://github.com/cBournhonesque/lightyear/issues/761), [#764](https://github.com/cBournhonesque/lightyear/issues/764), [#766](https://github.com/cBournhonesque/lightyear/issues/766), [#767](https://github.com/cBournhonesque/lightyear/issues/767), [#768](https://github.com/cBournhonesque/lightyear/issues/768), [#769](https://github.com/cBournhonesque/lightyear/issues/769), [#770](https://github.com/cBournhonesque/lightyear/issues/770), [#771](https://github.com/cBournhonesque/lightyear/issues/771), [#772](https://github.com/cBournhonesque/lightyear/issues/772), [#773](https://github.com/cBournhonesque/lightyear/issues/773), [#774](https://github.com/cBournhonesque/lightyear/issues/774), [#775](https://github.com/cBournhonesque/lightyear/issues/775), [#776](https://github.com/cBournhonesque/lightyear/issues/776), [#778](https://github.com/cBournhonesque/lightyear/issues/778), [#780](https://github.com/cBournhonesque/lightyear/issues/780), [#781](https://github.com/cBournhonesque/lightyear/issues/781), [#783](https://github.com/cBournhonesque/lightyear/issues/783), [#787](https://github.com/cBournhonesque/lightyear/issues/787), [#788](https://github.com/cBournhonesque/lightyear/issues/788), [#794](https://github.com/cBournhonesque/lightyear/issues/794), [#796](https://github.com/cBournhonesque/lightyear/issues/796), [#798](https://github.com/cBournhonesque/lightyear/issues/798), [#799](https://github.com/cBournhonesque/lightyear/issues/799), [#802](https://github.com/cBournhonesque/lightyear/issues/802), [#803](https://github.com/cBournhonesque/lightyear/issues/803), [#804](https://github.com/cBournhonesque/lightyear/issues/804), [#805](https://github.com/cBournhonesque/lightyear/issues/805), [#806](https://github.com/cBournhonesque/lightyear/issues/806), [#808](https://github.com/cBournhonesque/lightyear/issues/808), [#810](https://github.com/cBournhonesque/lightyear/issues/810), [#811](https://github.com/cBournhonesque/lightyear/issues/811), [#812](https://github.com/cBournhonesque/lightyear/issues/812), [#813](https://github.com/cBournhonesque/lightyear/issues/813), [#815](https://github.com/cBournhonesque/lightyear/issues/815), [#819](https://github.com/cBournhonesque/lightyear/issues/819), [#820](https://github.com/cBournhonesque/lightyear/issues/820), [#821](https://github.com/cBournhonesque/lightyear/issues/821), [#822](https://github.com/cBournhonesque/lightyear/issues/822), [#823](https://github.com/cBournhonesque/lightyear/issues/823), [#824](https://github.com/cBournhonesque/lightyear/issues/824), [#825](https://github.com/cBournhonesque/lightyear/issues/825), [#826](https://github.com/cBournhonesque/lightyear/issues/826), [#831](https://github.com/cBournhonesque/lightyear/issues/831), [#832](https://github.com/cBournhonesque/lightyear/issues/832), [#838](https://github.com/cBournhonesque/lightyear/issues/838), [#841](https://github.com/cBournhonesque/lightyear/issues/841), [#844](https://github.com/cBournhonesque/lightyear/issues/844), [#845](https://github.com/cBournhonesque/lightyear/issues/845), [#846](https://github.com/cBournhonesque/lightyear/issues/846), [#849](https://github.com/cBournhonesque/lightyear/issues/849), [#853](https://github.com/cBournhonesque/lightyear/issues/853), [#854](https://github.com/cBournhonesque/lightyear/issues/854), [#856](https://github.com/cBournhonesque/lightyear/issues/856), [#857](https://github.com/cBournhonesque/lightyear/issues/857), [#860](https://github.com/cBournhonesque/lightyear/issues/860), [#861](https://github.com/cBournhonesque/lightyear/issues/861), [#862](https://github.com/cBournhonesque/lightyear/issues/862), [#864](https://github.com/cBournhonesque/lightyear/issues/864), [#871](https://github.com/cBournhonesque/lightyear/issues/871), [#872](https://github.com/cBournhonesque/lightyear/issues/872), [#873](https://github.com/cBournhonesque/lightyear/issues/873), [#874](https://github.com/cBournhonesque/lightyear/issues/874), [#876](https://github.com/cBournhonesque/lightyear/issues/876), [#877](https://github.com/cBournhonesque/lightyear/issues/877), [#881](https://github.com/cBournhonesque/lightyear/issues/881), [#884](https://github.com/cBournhonesque/lightyear/issues/884), [#896](https://github.com/cBournhonesque/lightyear/issues/896), [#902](https://github.com/cBournhonesque/lightyear/issues/902), [#903](https://github.com/cBournhonesque/lightyear/issues/903), [#908](https://github.com/cBournhonesque/lightyear/issues/908), [#913](https://github.com/cBournhonesque/lightyear/issues/913), [#916](https://github.com/cBournhonesque/lightyear/issues/916), [#917](https://github.com/cBournhonesque/lightyear/issues/917), [#919](https://github.com/cBournhonesque/lightyear/issues/919), [#923](https://github.com/cBournhonesque/lightyear/issues/923), [#924](https://github.com/cBournhonesque/lightyear/issues/924), [#925](https://github.com/cBournhonesque/lightyear/issues/925), [#928](https://github.com/cBournhonesque/lightyear/issues/928), [#935](https://github.com/cBournhonesque/lightyear/issues/935), [#943](https://github.com/cBournhonesque/lightyear/issues/943), [#950](https://github.com/cBournhonesque/lightyear/issues/950), [#951](https://github.com/cBournhonesque/lightyear/issues/951), [#954](https://github.com/cBournhonesque/lightyear/issues/954), [#955](https://github.com/cBournhonesque/lightyear/issues/955), [#958](https://github.com/cBournhonesque/lightyear/issues/958), [#959](https://github.com/cBournhonesque/lightyear/issues/959), [#962](https://github.com/cBournhonesque/lightyear/issues/962), [#965](https://github.com/cBournhonesque/lightyear/issues/965), [#976](https://github.com/cBournhonesque/lightyear/issues/976), [#982](https://github.com/cBournhonesque/lightyear/issues/982), [#989](https://github.com/cBournhonesque/lightyear/issues/989), [#999](https://github.com/cBournhonesque/lightyear/issues/999)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1001](https://github.com/cBournhonesque/lightyear/issues/1001)**
    - Fix replication when entities are spawned between replication_send_interval ([`c3b8398`](https://github.com/cBournhonesque/lightyear/commit/c3b83988283363cab7297d74f83b3577293a179b))
 * **[#1003](https://github.com/cBournhonesque/lightyear/issues/1003)**
    - Fix wasm for 0.20.0 ([`ea05ae7`](https://github.com/cBournhonesque/lightyear/commit/ea05ae794cf575313d2c8313e5d3c086f1269e2a))
 * **[#1004](https://github.com/cBournhonesque/lightyear/issues/1004)**
    - Fix hierarchy ([`08d6c24`](https://github.com/cBournhonesque/lightyear/commit/08d6c2456717330ac4f15bd6869bd1cc2dd67d81))
 * **[#1006](https://github.com/cBournhonesque/lightyear/issues/1006)**
    - Make relationship pub ([`bf4e756`](https://github.com/cBournhonesque/lightyear/commit/bf4e756791ea75ab4313d48a41d588c904d78741))
 * **[#1007](https://github.com/cBournhonesque/lightyear/issues/1007)**
    - Make AppTriggerExt pub ([`779c0d1`](https://github.com/cBournhonesque/lightyear/commit/779c0d13d9d59f854020cc201d517c54a255427c))
 * **[#1008](https://github.com/cBournhonesque/lightyear/issues/1008)**
    - Fix examples and upgrade avian ([`a70fc44`](https://github.com/cBournhonesque/lightyear/commit/a70fc44d1e7c78b6ae533c8ce051af0c5329920a))
 * **[#1017](https://github.com/cBournhonesque/lightyear/issues/1017)**
    - Release 0.21 rc1 ([`dc0e61e`](https://github.com/cBournhonesque/lightyear/commit/dc0e61e06fe68309ed8cbfdcdfead633ad567537))
 * **[#1018](https://github.com/cBournhonesque/lightyear/issues/1018)**
    - Separate Connected from LocalId/RemoteId ([`89ce3e7`](https://github.com/cBournhonesque/lightyear/commit/89ce3e705fb262fe819ac1d254468caf3fc5fce5))
 * **[#1019](https://github.com/cBournhonesque/lightyear/issues/1019)**
    - Add steam ([`c1de749`](https://github.com/cBournhonesque/lightyear/commit/c1de749a142b763874596d131b1a9f673e4ea5a1))
 * **[#1021](https://github.com/cBournhonesque/lightyear/issues/1021)**
    - Fix lobby example (without HostServer) and add protocolhash ([`0beb664`](https://github.com/cBournhonesque/lightyear/commit/0beb664f0161f73e4a53c06530ae139078ed8763))
 * **[#1023](https://github.com/cBournhonesque/lightyear/issues/1023)**
    - Add HostServer ([`5b6af7e`](https://github.com/cBournhonesque/lightyear/commit/5b6af7edd3b41c05333d14dde258ea5e89c07c2d))
 * **[#1024](https://github.com/cBournhonesque/lightyear/issues/1024)**
    - Enable host-client mode on simple box ([`4117330`](https://github.com/cBournhonesque/lightyear/commit/411733089f59eb90d405f7ad327b5440b55ef060))
 * **[#1029](https://github.com/cBournhonesque/lightyear/issues/1029)**
    - Enable host-server for all examples ([`bc7cf37`](https://github.com/cBournhonesque/lightyear/commit/bc7cf371f822ff7a2667c329b6f77e5a694a93d4))
 * **[#1039](https://github.com/cBournhonesque/lightyear/issues/1039)**
    - Support BEI inputs ([`117b084`](https://github.com/cBournhonesque/lightyear/commit/117b0841a25dba5c6ffaadad88a8c4dba09d3cbb))
 * **[#1043](https://github.com/cBournhonesque/lightyear/issues/1043)**
    - Make workspace crates depend on individual bevy crates ([`5dc3dc3`](https://github.com/cBournhonesque/lightyear/commit/5dc3dc3e17a8b821c35162b904b73eea0e1c69be))
 * **[#1049](https://github.com/cBournhonesque/lightyear/issues/1049)**
    - Alternative replication system + fix delta-compression ([`4d5e690`](https://github.com/cBournhonesque/lightyear/commit/4d5e69072485faa3975543792a8e11be7608a0ea))
 * **[#1051](https://github.com/cBournhonesque/lightyear/issues/1051)**
    - Add tests for delta-compression ([`72ecbb9`](https://github.com/cBournhonesque/lightyear/commit/72ecbb9604bbb7add8e911cf9d72f21fd00eed6c))
 * **[#1054](https://github.com/cBournhonesque/lightyear/issues/1054)**
    - Chore(docs) ([`59b9f7e`](https://github.com/cBournhonesque/lightyear/commit/59b9f7eb37b036488d3ceab780074274074a9bd6))
 * **[#1055](https://github.com/cBournhonesque/lightyear/issues/1055)**
    - Release 0.21 rc 2 ([`81341e9`](https://github.com/cBournhonesque/lightyear/commit/81341e91707b31a5cba6967d23e230945180a4e8))
 * **[#1058](https://github.com/cBournhonesque/lightyear/issues/1058)**
    - Separate avian into 2 crates, fix delta-compression example, fix book links ([`1abda44`](https://github.com/cBournhonesque/lightyear/commit/1abda441054255978b6d5bef9da8e538b91aa1ed))
 * **[#1061](https://github.com/cBournhonesque/lightyear/issues/1061)**
    - Enable steam on simple_box example and fix wasm ([`0bd3fbe`](https://github.com/cBournhonesque/lightyear/commit/0bd3fbe9db6d8dfd350a0e014e7beec9392df1de))
 * **[#1062](https://github.com/cBournhonesque/lightyear/issues/1062)**
    - Release lightyear_avian2d v0.21.0-rc.3, lightyear_avian3d v0.21.0-rc.3, lightyear_crossbeam v0.21.0-rc.3, lightyear_inputs v0.21.0-rc.3, lightyear_inputs_bei v0.21.0-rc.3, lightyear_inputs_leafwing v0.21.0-rc.3, lightyear_inputs_native v0.21.0-rc.3, lightyear_netcode v0.21.0-rc.3, lightyear_steam v0.21.0-rc.3, lightyear_webtransport v0.21.0-rc.3, lightyear_udp v0.21.0-rc.3, lightyear v0.21.0-rc.3 ([`0004a2d`](https://github.com/cBournhonesque/lightyear/commit/0004a2db67662a5ee9284bec7c146e58dc4d1494))
 * **[#464](https://github.com/cBournhonesque/lightyear/issues/464)**
    - Upgrade to bevy 0.14 ([`61809eb`](https://github.com/cBournhonesque/lightyear/commit/61809eb3d38d61a874c6a6d948c597873808f1ba))
 * **[#512](https://github.com/cBournhonesque/lightyear/issues/512)**
    - Fix some replication issues ([`d860c5c`](https://github.com/cBournhonesque/lightyear/commit/d860c5ccd47f94f8214ec61847959c5f8e535e46))
 * **[#513](https://github.com/cBournhonesque/lightyear/issues/513)**
    - Remove extra visibility ([`2717753`](https://github.com/cBournhonesque/lightyear/commit/27177533f08ed9180404fda1a4234fcf07617d97))
 * **[#515](https://github.com/cBournhonesque/lightyear/issues/515)**
    - Fix ([`deec7e6`](https://github.com/cBournhonesque/lightyear/commit/deec7e62fae81cbf328728a6a08d2cc17032cbd1))
 * **[#523](https://github.com/cBournhonesque/lightyear/issues/523)**
    - Fix ([`226ceb9`](https://github.com/cBournhonesque/lightyear/commit/226ceb90a4c4abdf99f0cea776df8373b92b9383))
 * **[#525](https://github.com/cBournhonesque/lightyear/issues/525)**
    - Fix space ships host-server ([`7dfa84a`](https://github.com/cBournhonesque/lightyear/commit/7dfa84a9bc5aa45dac03488e5b1128e9c4a736d3))
 * **[#528](https://github.com/cBournhonesque/lightyear/issues/528)**
    - Fix host-server control ([`e53e5a9`](https://github.com/cBournhonesque/lightyear/commit/e53e5a9f9b5a999d1744863d7599b4885e8f1f98))
 * **[#530](https://github.com/cBournhonesque/lightyear/issues/530)**
    - Update the visibility of some packages ([`d55d86e`](https://github.com/cBournhonesque/lightyear/commit/d55d86e7c4d7dd4554a07dff05ae5d51734ef709))
 * **[#531](https://github.com/cBournhonesque/lightyear/issues/531)**
    - Panic if no interpolation function is provided ([`58db131`](https://github.com/cBournhonesque/lightyear/commit/58db13136404903e4f9a60281aa7981cc862c6cb))
 * **[#534](https://github.com/cBournhonesque/lightyear/issues/534)**
    - Upgrade to avian 0.1.1 ([`0316042`](https://github.com/cBournhonesque/lightyear/commit/0316042ce4ed79bdef10394d4655d66475482d68))
 * **[#537](https://github.com/cBournhonesque/lightyear/issues/537)**
    - Add HostServer test harness ([`bf8df70`](https://github.com/cBournhonesque/lightyear/commit/bf8df70ce413851507e1d4ebcadaa94dda141a5c))
 * **[#538](https://github.com/cBournhonesque/lightyear/issues/538)**
    - Let the local-client in host-server mode send a message to the Server ([`6466922`](https://github.com/cBournhonesque/lightyear/commit/6466922b6672f46f6915b85807404f478d1a2e59))
 * **[#542](https://github.com/cBournhonesque/lightyear/issues/542)**
    - In host-server mode, send messages from server to the local client ([`446ab5c`](https://github.com/cBournhonesque/lightyear/commit/446ab5c66d2b859e7236ed5e84e091666a426f5f))
 * **[#543](https://github.com/cBournhonesque/lightyear/issues/543)**
    - Fix events clearing ([`6466ade`](https://github.com/cBournhonesque/lightyear/commit/6466ade5cbf2236ea3b61db36c30a189b22c963e))
 * **[#544](https://github.com/cBournhonesque/lightyear/issues/544)**
    - Update prelude to have non-conflicting names ([`528ecf9`](https://github.com/cBournhonesque/lightyear/commit/528ecf98caa4fa8da00145cf2744927adc1e5377))
 * **[#548](https://github.com/cBournhonesque/lightyear/issues/548)**
    - Propagate replicate for children after the entity is spawned ([`2f3f393`](https://github.com/cBournhonesque/lightyear/commit/2f3f39335b641b56a34475195a7dda1468a689e8))
 * **[#549](https://github.com/cBournhonesque/lightyear/issues/549)**
    - Fix observer panicking with resource missing ([`d4e1e2e`](https://github.com/cBournhonesque/lightyear/commit/d4e1e2ef1ce013b7c256f7de4bb75ca5a8f01faa))
 * **[#554](https://github.com/cBournhonesque/lightyear/issues/554)**
    - Add helper functions to map from local world to remote world ([`7b312e0`](https://github.com/cBournhonesque/lightyear/commit/7b312e0e61072b42e3540e4e0edc94465f4791d1))
 * **[#555](https://github.com/cBournhonesque/lightyear/issues/555)**
    - Add linear interpolation quat ([`4fd6ddb`](https://github.com/cBournhonesque/lightyear/commit/4fd6ddbdd97ed3c04a3d8ba777261a6577da987f))
 * **[#556](https://github.com/cBournhonesque/lightyear/issues/556)**
    - Cb/add quat lerp ([`06d70a9`](https://github.com/cBournhonesque/lightyear/commit/06d70a9919b5afda6719e5ce098b6fa87549349d))
 * **[#557](https://github.com/cBournhonesque/lightyear/issues/557)**
    - Make lifetime pub ([`faee583`](https://github.com/cBournhonesque/lightyear/commit/faee5836fbbb472cfa8e54115553e303b3c9a371))
 * **[#558](https://github.com/cBournhonesque/lightyear/issues/558)**
    - Add option to trigger change detection ([`89b97d2`](https://github.com/cBournhonesque/lightyear/commit/89b97d2126258e428f1dc6a4773da7e8d9973cda))
 * **[#559](https://github.com/cBournhonesque/lightyear/issues/559)**
    - Add option to trigger change detection ([`87806d0`](https://github.com/cBournhonesque/lightyear/commit/87806d0c9cb6fef22978cf7b170089e37711d329))
 * **[#561](https://github.com/cBournhonesque/lightyear/issues/561)**
    - Fixes for bevy 0.14.1 ([`5bd83d8`](https://github.com/cBournhonesque/lightyear/commit/5bd83d8273c3dfa23d64c9d909488ff1b8ed2b41))
 * **[#566](https://github.com/cBournhonesque/lightyear/issues/566)**
    - Set the channel priority to INF for ping/pong/inputs ([`6293fd4`](https://github.com/cBournhonesque/lightyear/commit/6293fd4060a5822eeae16076577457465fb6471b))
 * **[#567](https://github.com/cBournhonesque/lightyear/issues/567)**
    - Add MTU config for QUIC ([`11e4e95`](https://github.com/cBournhonesque/lightyear/commit/11e4e9552c2fc634fa943101592a794f6e381ad8))
 * **[#570](https://github.com/cBournhonesque/lightyear/issues/570)**
    - Make InputBuffer::pop public ([`6544bf0`](https://github.com/cBournhonesque/lightyear/commit/6544bf00d0fc827374fc014f161e8f5f4db3e895))
 * **[#572](https://github.com/cBournhonesque/lightyear/issues/572)**
    - Add avian3d utilities ([`42f2a15`](https://github.com/cBournhonesque/lightyear/commit/42f2a15be99b0dd40618c45879114c321c6475c6))
 * **[#575](https://github.com/cBournhonesque/lightyear/issues/575)**
    - Upgrade to leafwing 0.15 ([`61fb8f0`](https://github.com/cBournhonesque/lightyear/commit/61fb8f0c4604a8fa2504ba6118467a8291986526))
 * **[#578](https://github.com/cBournhonesque/lightyear/issues/578)**
    - Fixed Local client not sending Connect event in HostServer Mode + Added Test ([`c449505`](https://github.com/cBournhonesque/lightyear/commit/c449505aea9afe09c19ec032409fb476d4317378))
 * **[#581](https://github.com/cBournhonesque/lightyear/issues/581)**
    - Use `Commands` instead of `World` to apply replication updates ([`3045d31`](https://github.com/cBournhonesque/lightyear/commit/3045d314e617bb90166f7eb2a30ddc341f3e0b7e))
 * **[#583](https://github.com/cBournhonesque/lightyear/issues/583)**
    - Add rollback for non-networked components ([`fa2d971`](https://github.com/cBournhonesque/lightyear/commit/fa2d971310726ed8fd48b79d1eb65833a4a79d9b))
 * **[#588](https://github.com/cBournhonesque/lightyear/issues/588)**
    - Add unit test server send message in host-server mode ([`fe8c363`](https://github.com/cBournhonesque/lightyear/commit/fe8c363f7af9fa2506aec510ef16d7c30e76a008))
 * **[#589](https://github.com/cBournhonesque/lightyear/issues/589)**
    - Dep upgrade ([`57f69dd`](https://github.com/cBournhonesque/lightyear/commit/57f69ddd3b891c9d34a6867fd27002b3039c9c84))
 * **[#591](https://github.com/cBournhonesque/lightyear/issues/591)**
    - Allow seamless entity mapping ([`aa302f5`](https://github.com/cBournhonesque/lightyear/commit/aa302f5fa561a44f3f3414fc0893229d4d560ddf))
 * **[#592](https://github.com/cBournhonesque/lightyear/issues/592)**
    - Fix ([`298fd84`](https://github.com/cBournhonesque/lightyear/commit/298fd8485b2534c05cc39f3f3915535c3b7105e3))
 * **[#594](https://github.com/cBournhonesque/lightyear/issues/594)**
    - Add authority handling natively ([`4d6f645`](https://github.com/cBournhonesque/lightyear/commit/4d6f6458f869957711bf2a98077787b83631a93e))
 * **[#597](https://github.com/cBournhonesque/lightyear/issues/597)**
    - Add an example for authority transfer ([`841a7c1`](https://github.com/cBournhonesque/lightyear/commit/841a7c17f39ede17355d367b70ade45508082be9))
 * **[#598](https://github.com/cBournhonesque/lightyear/issues/598)**
    - Fix Pre-Prediction ([`880d9df`](https://github.com/cBournhonesque/lightyear/commit/880d9dfba031216c6da9dc5c06bc32114102eea4))
 * **[#600](https://github.com/cBournhonesque/lightyear/issues/600)**
    - Release 0.17 ([`f4cec2e`](https://github.com/cBournhonesque/lightyear/commit/f4cec2e18dac7c7b3d254de97160ea83987fdaf8))
 * **[#601](https://github.com/cBournhonesque/lightyear/issues/601)**
    - Make the RemoteEntityMap public ([`747b03e`](https://github.com/cBournhonesque/lightyear/commit/747b03e46ea001e01949a7f268c79b4bea5c39fd))
 * **[#602](https://github.com/cBournhonesque/lightyear/issues/602)**
    - Order lightyear plugins w.r.t avian plugins ([`6ea2c2e`](https://github.com/cBournhonesque/lightyear/commit/6ea2c2edd4f6a3b5f7c13e3557d9e8c1a8674adc))
 * **[#606](https://github.com/cBournhonesque/lightyear/issues/606)**
    - Send events as server::events::InputEvent instead of client::events:InputEvent  in send_input_directly_to_client_events when using HostServer ([`93b72b9`](https://github.com/cBournhonesque/lightyear/commit/93b72b9add5d45742c0f054ea9baf5d541b6ae8d))
 * **[#607](https://github.com/cBournhonesque/lightyear/issues/607)**
    - Remove unneeded import ([`cb83132`](https://github.com/cBournhonesque/lightyear/commit/cb83132eaf10d0143f5ac820ffa9396c93e04603))
 * **[#612](https://github.com/cBournhonesque/lightyear/issues/612)**
    - Use the parent's ReplicationGroup when propagating it to the children ([`a3e54a6`](https://github.com/cBournhonesque/lightyear/commit/a3e54a66bf253c3ecfff7086d9a828c8f43da76e))
 * **[#615](https://github.com/cBournhonesque/lightyear/issues/615)**
    - Cargo fmt ([`df475e9`](https://github.com/cBournhonesque/lightyear/commit/df475e9d0d622cf7920dd80b31e705c9bd2c76c5))
 * **[#622](https://github.com/cBournhonesque/lightyear/issues/622)**
    - Support rolling back resources ([`2cd8cf8`](https://github.com/cBournhonesque/lightyear/commit/2cd8cf8c32fa3106512ede5d52e221472fc5a3be))
 * **[#623](https://github.com/cBournhonesque/lightyear/issues/623)**
    - Rollback time resource during rollback ([`a7dd781`](https://github.com/cBournhonesque/lightyear/commit/a7dd781d81a09694c4055dcd2bb8bbe9813ebc61))
 * **[#632](https://github.com/cBournhonesque/lightyear/issues/632)**
    - Add unit tests for map-entities related to PreSpawned and Prediction ([`3a610d9`](https://github.com/cBournhonesque/lightyear/commit/3a610d9d808d3166d51c47ecef2bd381760aa973))
 * **[#636](https://github.com/cBournhonesque/lightyear/issues/636)**
    - Add missing `reflect(Component)`s ([`1724b6d`](https://github.com/cBournhonesque/lightyear/commit/1724b6d0513dd74af15a60a21abc6585c6398cbb))
 * **[#638](https://github.com/cBournhonesque/lightyear/issues/638)**
    - Separate `Replicated` from `InitialReplicated` ([`d916fb8`](https://github.com/cBournhonesque/lightyear/commit/d916fb87ee8935ee6f20e9bb17c410f03dedb52a))
 * **[#642](https://github.com/cBournhonesque/lightyear/issues/642)**
    - Fix 639 ([`193f59c`](https://github.com/cBournhonesque/lightyear/commit/193f59c7dc94cfecc08b56b90bb6e6c9f37d4e4e))
 * **[#646](https://github.com/cBournhonesque/lightyear/issues/646)**
    - Update xwt-web-sys requirement from 0.12 to 0.13 ([`1328354`](https://github.com/cBournhonesque/lightyear/commit/132835485b69e07942adc77df22403e6c776841d))
 * **[#647](https://github.com/cBournhonesque/lightyear/issues/647)**
    - Map entities when rollbacking predicted component ([`28e8c42`](https://github.com/cBournhonesque/lightyear/commit/28e8c420d67561a67cd46a3436fbb8d782bfb460))
 * **[#653](https://github.com/cBournhonesque/lightyear/issues/653)**
    - Simplify wrapping_diff code ([`09db2d4`](https://github.com/cBournhonesque/lightyear/commit/09db2d4762cc3d8d6cbb127ab6bdfa6ea6866352))
 * **[#654](https://github.com/cBournhonesque/lightyear/issues/654)**
    - Expose and document `SerializeFns` ([`9ddab9a`](https://github.com/cBournhonesque/lightyear/commit/9ddab9a6f1826109e9fd613b23e15efe82138cd7))
 * **[#656](https://github.com/cBournhonesque/lightyear/issues/656)**
    - Return correctly the entity that is to be deleted from the RemoteEntityMap if it had been pre-mapped ([`770eef0`](https://github.com/cBournhonesque/lightyear/commit/770eef09eaeaa02842bbea81b28a86a8b8247a66))
 * **[#657](https://github.com/cBournhonesque/lightyear/issues/657)**
    - Expose ReplicateToServer ([`b77f2ee`](https://github.com/cBournhonesque/lightyear/commit/b77f2eeb5e7751016e9a981407710c60a9c75c88))
 * **[#661](https://github.com/cBournhonesque/lightyear/issues/661)**
    - Support LWIM 15.1 triple axis input ([`7bca1d8`](https://github.com/cBournhonesque/lightyear/commit/7bca1d8fe07e6af5899ec544d7c8b1c3ac88154b))
 * **[#665](https://github.com/cBournhonesque/lightyear/issues/665)**
    - Require UserAction bound for InputPlugin ([`56970e1`](https://github.com/cBournhonesque/lightyear/commit/56970e186229055b5cc59f0ed6f2d071b42ee4ba))
 * **[#666](https://github.com/cBournhonesque/lightyear/issues/666)**
    - Avoid panic when getting EntityCommands in hierarchy ([`a7aed47`](https://github.com/cBournhonesque/lightyear/commit/a7aed471ee6e35e937cfa6ff3b6baec1d245fc05))
 * **[#670](https://github.com/cBournhonesque/lightyear/issues/670)**
    - Disable default leafwing features ([`d3146a7`](https://github.com/cBournhonesque/lightyear/commit/d3146a7ab5ee90d40611d6cdaa24a5f20a8a2ce4))
 * **[#671](https://github.com/cBournhonesque/lightyear/issues/671)**
    - Fix compiling in wasm ([`0b7a485`](https://github.com/cBournhonesque/lightyear/commit/0b7a4852fbb2eb9848d2fbc04dd4eea70d51ea98))
 * **[#672](https://github.com/cBournhonesque/lightyear/issues/672)**
    - Update metrics-util requirement from 0.15 to 0.18 ([`7aefaee`](https://github.com/cBournhonesque/lightyear/commit/7aefaee14d6b838f803aea9f1eb732ca159cfbaa))
 * **[#674](https://github.com/cBournhonesque/lightyear/issues/674)**
    - Update metrics-tracing-context requirement from 0.15 to 0.17 ([`4425f91`](https://github.com/cBournhonesque/lightyear/commit/4425f91c69377ef0ef8de955e9fae4d075123d46))
 * **[#675](https://github.com/cBournhonesque/lightyear/issues/675)**
    - Update metrics requirement from 0.23 to 0.24 ([`aa12ccc`](https://github.com/cBournhonesque/lightyear/commit/aa12ccc07b9c4b62c04144236139f75e292f490e))
 * **[#682](https://github.com/cBournhonesque/lightyear/issues/682)**
    - Update governor requirement from 0.6.0 to 0.7.0 ([`12b95e7`](https://github.com/cBournhonesque/lightyear/commit/12b95e7b13382aafb299096188c7c9535a1f2591))
 * **[#687](https://github.com/cBournhonesque/lightyear/issues/687)**
    - Remove `ring` to fix wasm32 web builds ([`ae2f4b2`](https://github.com/cBournhonesque/lightyear/commit/ae2f4b2a5caf60eabbbd83877a5c5c8a3486588e))
 * **[#693](https://github.com/cBournhonesque/lightyear/issues/693)**
    - Fix clippy lint error ([`aa4c668`](https://github.com/cBournhonesque/lightyear/commit/aa4c6687932a51cfff69bc469b3c8762798e8f2d))
 * **[#694](https://github.com/cBournhonesque/lightyear/issues/694)**
    - Fix tarpaulin compile error ([`986e48e`](https://github.com/cBournhonesque/lightyear/commit/986e48e81949042aab1630657dbbd9923c1ce2c5))
 * **[#695](https://github.com/cBournhonesque/lightyear/issues/695)**
    - Ignore leafwing test ([`d74f55f`](https://github.com/cBournhonesque/lightyear/commit/d74f55f7a6e234549b9bed9fea171a4b78c9e00d))
 * **[#698](https://github.com/cBournhonesque/lightyear/issues/698)**
    - Fix logspam when client disconnects ([`59b089c`](https://github.com/cBournhonesque/lightyear/commit/59b089c41e57911653c8d0fe891db86bca50142d))
 * **[#700](https://github.com/cBournhonesque/lightyear/issues/700)**
    - Upgrade to bevy 0.15 ([`56afd9e`](https://github.com/cBournhonesque/lightyear/commit/56afd9e6aee285d17bf99661fee16716fbf52296))
 * **[#703](https://github.com/cBournhonesque/lightyear/issues/703)**
    - Fix issue ([`3c6a93e`](https://github.com/cBournhonesque/lightyear/commit/3c6a93efb7deecb33ab6fa344a1aded5422b9bdf))
 * **[#705](https://github.com/cBournhonesque/lightyear/issues/705)**
    - Fix client replication example ([`eca62ef`](https://github.com/cBournhonesque/lightyear/commit/eca62ef92dd41893e1c957d9c9ccec187a1b6849))
 * **[#706](https://github.com/cBournhonesque/lightyear/issues/706)**
    - Update wtransport requirement from =0.1.14 to =0.5.0 ([`1063b35`](https://github.com/cBournhonesque/lightyear/commit/1063b3593e91f0ead06638fbe3cd1f155daa91e3))
 * **[#708](https://github.com/cBournhonesque/lightyear/issues/708)**
    - Update hashbrown requirement from 0.14 to 0.15 ([`38a3c07`](https://github.com/cBournhonesque/lightyear/commit/38a3c07907ca4ad87b3e61b87931e28145e7bce2))
 * **[#709](https://github.com/cBournhonesque/lightyear/issues/709)**
    - Update thiserror requirement from 1.0.50 to 2.0.3 ([`7f5f110`](https://github.com/cBournhonesque/lightyear/commit/7f5f1107619d2853867437861dbd67aeedf4df21))
 * **[#710](https://github.com/cBournhonesque/lightyear/issues/710)**
    - Use a hook on PreSpawnedPlayerObject component to compute missing hashes ([`faebbd2`](https://github.com/cBournhonesque/lightyear/commit/faebbd2d894ebbe9966a05621d1c64fccc0deed2))
 * **[#711](https://github.com/cBournhonesque/lightyear/issues/711)**
    - Compilation fixes ([`989bc90`](https://github.com/cBournhonesque/lightyear/commit/989bc9006eb19b702ff106591d52fb6882701fe5))
 * **[#713](https://github.com/cBournhonesque/lightyear/issues/713)**
    - Fix pre predicted replication ([`dec5861`](https://github.com/cBournhonesque/lightyear/commit/dec5861d62c18f43ac8a6ad1c319588cea773424))
 * **[#714](https://github.com/cBournhonesque/lightyear/issues/714)**
    - Update ([`5845699`](https://github.com/cBournhonesque/lightyear/commit/5845699d02ee953b73fa6f341224fe0df9dfa12b))
 * **[#722](https://github.com/cBournhonesque/lightyear/issues/722)**
    - Improve flow for host-server disconnection ([`8f76532`](https://github.com/cBournhonesque/lightyear/commit/8f765324183d3dac989ac349be8d6919a5fc3268))
 * **[#723](https://github.com/cBournhonesque/lightyear/issues/723)**
    - Make states enum reflective for pretty UIs :) ([`3190e74`](https://github.com/cBournhonesque/lightyear/commit/3190e744c3f6c4858dbf6a3a11abf721c99cf7cb))
 * **[#724](https://github.com/cBournhonesque/lightyear/issues/724)**
    - Fix duplicate despawn ([`1c5e1f9`](https://github.com/cBournhonesque/lightyear/commit/1c5e1f961092cb45ffcf27e06ae15ab4a36ed35e))
 * **[#726](https://github.com/cBournhonesque/lightyear/issues/726)**
    - Upgrade dependencies ([`02521f0`](https://github.com/cBournhonesque/lightyear/commit/02521f07278d1f36ffa20b9dc0aff564000e2652))
 * **[#732](https://github.com/cBournhonesque/lightyear/issues/732)**
    - Refactor Cargo workspace and examples ([`717c4b3`](https://github.com/cBournhonesque/lightyear/commit/717c4b33b4166f7282d292d36e7caeeca6aecc68))
 * **[#736](https://github.com/cBournhonesque/lightyear/issues/736)**
    - Fix input delay config ([`ced8f17`](https://github.com/cBournhonesque/lightyear/commit/ced8f17007f75faafe5b09ba1833874564931180))
 * **[#741](https://github.com/cBournhonesque/lightyear/issues/741)**
    - Fix delta-compression by storing ack_tick for each (entity, component) ([`cf178fb`](https://github.com/cBournhonesque/lightyear/commit/cf178fb05b20b41a10b199a7bcee0e754d8c4dd4))
 * **[#746](https://github.com/cBournhonesque/lightyear/issues/746)**
    - Fix track_change_detection ([`2f825f5`](https://github.com/cBournhonesque/lightyear/commit/2f825f572984da69a52156572f8eefc08bc036a8))
 * **[#748](https://github.com/cBournhonesque/lightyear/issues/748)**
    - Remove DisabledComponent<C> in favor of DisabledComponents ([`b3ba4cb`](https://github.com/cBournhonesque/lightyear/commit/b3ba4cbb0996c52744551b8a63ad06909b1d9c98))
 * **[#749](https://github.com/cBournhonesque/lightyear/issues/749)**
    - Simplify wasm certificate ([`f7c9cfe`](https://github.com/cBournhonesque/lightyear/commit/f7c9cfe7308bbf68e81db97849388798df2de219))
 * **[#754](https://github.com/cBournhonesque/lightyear/issues/754)**
    - Add event replication ([`c2c061d`](https://github.com/cBournhonesque/lightyear/commit/c2c061d0a1dfdf0687e810d54065c1972f7c2111))
 * **[#757](https://github.com/cBournhonesque/lightyear/issues/757)**
    - Add unit test ([`dae335e`](https://github.com/cBournhonesque/lightyear/commit/dae335eb52b63296055033189b1c456ee60d77a2))
 * **[#759](https://github.com/cBournhonesque/lightyear/issues/759)**
    - Allow enabling prediction/interpolation even with authority transfer ([`2526956`](https://github.com/cBournhonesque/lightyear/commit/25269560312c7075e50d3e430e757b5a3b215423))
 * **[#761](https://github.com/cBournhonesque/lightyear/issues/761)**
    - Fix pre-prediction ([`1a6d9d9`](https://github.com/cBournhonesque/lightyear/commit/1a6d9d9dacbdbd6eaa3c1060bd0e5683d33aa9f0))
 * **[#764](https://github.com/cBournhonesque/lightyear/issues/764)**
    - Make sure ChannelDirection is respected ([`a77026d`](https://github.com/cBournhonesque/lightyear/commit/a77026d6ede36e400ad453435304128da4a1a7db))
 * **[#766](https://github.com/cBournhonesque/lightyear/issues/766)**
    - Improvements to room logic ([`8637abe`](https://github.com/cBournhonesque/lightyear/commit/8637abe36885cdd334f9b0fe9ad51b80870f4727))
 * **[#767](https://github.com/cBournhonesque/lightyear/issues/767)**
    - Add unit test ([`e750ca5`](https://github.com/cBournhonesque/lightyear/commit/e750ca51bc018209949975c31ef87a666640e819))
 * **[#768](https://github.com/cBournhonesque/lightyear/issues/768)**
    - Add unit test ([`18a6228`](https://github.com/cBournhonesque/lightyear/commit/18a6228ff13bbbf795046da4a25376a6d6a490bc))
 * **[#769](https://github.com/cBournhonesque/lightyear/issues/769)**
    - Improve the server stop logic ([`03503db`](https://github.com/cBournhonesque/lightyear/commit/03503db7277d925655e89ab69751897912804a95))
 * **[#770](https://github.com/cBournhonesque/lightyear/issues/770)**
    - Update Docs, closes #386 ([`10da718`](https://github.com/cBournhonesque/lightyear/commit/10da7186f8f595ef4f62f9e369b9cae1cc9422ad))
 * **[#771](https://github.com/cBournhonesque/lightyear/issues/771)**
    - Fix replicate propagation through hierarchy ([`fc48cc2`](https://github.com/cBournhonesque/lightyear/commit/fc48cc2b25b71d23285f54b24c8c94447500dd01))
 * **[#772](https://github.com/cBournhonesque/lightyear/issues/772)**
    - Make sure that hierarchy is propagated correctly to Predicted/Interpolated entities ([`329a47f`](https://github.com/cBournhonesque/lightyear/commit/329a47f1a5668d460bb08709809f3337685dedbc))
 * **[#773](https://github.com/cBournhonesque/lightyear/issues/773)**
    - Add unit test ([`774e948`](https://github.com/cBournhonesque/lightyear/commit/774e948dcadc2564ea07540b5ea73ed8cbd81368))
 * **[#774](https://github.com/cBournhonesque/lightyear/issues/774)**
    - Add benchmark to measure compilation times ([`c1ae879`](https://github.com/cBournhonesque/lightyear/commit/c1ae87996a648a770e1754ddc7bc47131b4fd957))
 * **[#775](https://github.com/cBournhonesque/lightyear/issues/775)**
    - Type erase the receive-message and receive-event systems ([`6026831`](https://github.com/cBournhonesque/lightyear/commit/60268319e5fd621ffb9e1c5023962abe51b28507))
 * **[#776](https://github.com/cBournhonesque/lightyear/issues/776)**
    - Register prespawned entity in predicted_entity_map during server/client match ([`7d9dbbf`](https://github.com/cBournhonesque/lightyear/commit/7d9dbbf435e94e1bc85e631a1df76951150f5aad))
 * **[#778](https://github.com/cBournhonesque/lightyear/issues/778)**
    - Improve flow to disconnect client ([`deedd71`](https://github.com/cBournhonesque/lightyear/commit/deedd71ff4bdb7764311dad0f8470d236f481c8e))
 * **[#780](https://github.com/cBournhonesque/lightyear/issues/780)**
    - Use ComponentHooks to maintain the prediction/confirmed map in sync ([`44c251e`](https://github.com/cBournhonesque/lightyear/commit/44c251ee84401d89f062016e529658e759e4442b))
 * **[#781](https://github.com/cBournhonesque/lightyear/issues/781)**
    - Update entity map to return Entity::PLACEHOLDER if the mapping fials ([`3bbc380`](https://github.com/cBournhonesque/lightyear/commit/3bbc3800f73953d578f993722b59b24f3924e496))
 * **[#783](https://github.com/cBournhonesque/lightyear/issues/783)**
    - Fix ([`d9073b3`](https://github.com/cBournhonesque/lightyear/commit/d9073b354be755b90f4b05d8e6e5660a0c0b1641))
 * **[#787](https://github.com/cBournhonesque/lightyear/issues/787)**
    - Avoid log ([`37f0b54`](https://github.com/cBournhonesque/lightyear/commit/37f0b54263aff7c5cc11e85d541356a142f5ba4b))
 * **[#788](https://github.com/cBournhonesque/lightyear/issues/788)**
    - Fix rollback for pre-predicted entities ([`eb07354`](https://github.com/cBournhonesque/lightyear/commit/eb0735458dcc9806f8cf6cf08948bbeaa9edceab))
 * **[#794](https://github.com/cBournhonesque/lightyear/issues/794)**
    - Add ([`df992d6`](https://github.com/cBournhonesque/lightyear/commit/df992d6c5f9abae261701f25efc556cf1ea038bc))
 * **[#796](https://github.com/cBournhonesque/lightyear/issues/796)**
    - Add order ([`e6147de`](https://github.com/cBournhonesque/lightyear/commit/e6147de0c23a08830ee1c4237ed35c3c7a0221f3))
 * **[#798](https://github.com/cBournhonesque/lightyear/issues/798)**
    - Update HistoryBuffers during client TickEvents ([`ca52843`](https://github.com/cBournhonesque/lightyear/commit/ca52843226f76872b3eefa9148901ad53dd4fd3f))
 * **[#799](https://github.com/cBournhonesque/lightyear/issues/799)**
    - Improvements ([`e82804c`](https://github.com/cBournhonesque/lightyear/commit/e82804cf36ff5851d54847c281ce505413fe0e07))
 * **[#802](https://github.com/cBournhonesque/lightyear/issues/802)**
    - Add metrics visualizer ([`8b127f9`](https://github.com/cBournhonesque/lightyear/commit/8b127f98d58e2b2952e25f0774c40597e5525a14))
 * **[#803](https://github.com/cBournhonesque/lightyear/issues/803)**
    - Fix lints ([`0b1865e`](https://github.com/cBournhonesque/lightyear/commit/0b1865ec6b2207e52317c2798b5d021b796bc2f9))
 * **[#804](https://github.com/cBournhonesque/lightyear/issues/804)**
    - Fix ServerConnection::client_addr ([`dbd4ac5`](https://github.com/cBournhonesque/lightyear/commit/dbd4ac5c50da10b1a58c7ef2af9b28f2abe6d708))
 * **[#805](https://github.com/cBournhonesque/lightyear/issues/805)**
    - Refactor examples to build a single binary ([`6f067b5`](https://github.com/cBournhonesque/lightyear/commit/6f067b5d7fa29e2c75f94df189ad8c2f188f9c15))
 * **[#806](https://github.com/cBournhonesque/lightyear/issues/806)**
    - Fix ([`6699c3d`](https://github.com/cBournhonesque/lightyear/commit/6699c3d039d0ff50bad823fd3c348f817609fad1))
 * **[#808](https://github.com/cBournhonesque/lightyear/issues/808)**
    - Spaceships predicted despawn behaviour ([`6920b23`](https://github.com/cBournhonesque/lightyear/commit/6920b2375a7f14ae9b4c8e3d7edb4bc2590fe35b))
 * **[#810](https://github.com/cBournhonesque/lightyear/issues/810)**
    - Add metrics for rollback ([`5761fce`](https://github.com/cBournhonesque/lightyear/commit/5761fcee2aaad30f21fa4dbb82baafa58ad7ac76))
 * **[#811](https://github.com/cBournhonesque/lightyear/issues/811)**
    - Add unit test for prespawn despawn ([`b698a44`](https://github.com/cBournhonesque/lightyear/commit/b698a44f74aac00e32833923e129f1720b519c85))
 * **[#812](https://github.com/cBournhonesque/lightyear/issues/812)**
    - Add unit tests for predicted despawn ([`a48fc2a`](https://github.com/cBournhonesque/lightyear/commit/a48fc2a835ed25187fa85c3f56437a7266537fe0))
 * **[#813](https://github.com/cBournhonesque/lightyear/issues/813)**
    - Add non-networked component to despawned predicted entity test ([`cccf8ed`](https://github.com/cBournhonesque/lightyear/commit/cccf8ed5a3bd8961cfbb4884d410514101cf56de))
 * **[#815](https://github.com/cBournhonesque/lightyear/issues/815)**
    - Fix extra rollbacks on pre-spawned entities ([`06423de`](https://github.com/cBournhonesque/lightyear/commit/06423de57c3c53a1d3a32e86e7e446f814ffe3e0))
 * **[#819](https://github.com/cBournhonesque/lightyear/issues/819)**
    - Fix despawn for pre-spawned entities ([`048c7be`](https://github.com/cBournhonesque/lightyear/commit/048c7bed5a1cd1a9356aec3810e88580b639bf2b))
 * **[#820](https://github.com/cBournhonesque/lightyear/issues/820)**
    - Fix ([`4593438`](https://github.com/cBournhonesque/lightyear/commit/45934386a14f08447bb5f3470146cf5b5513f54f))
 * **[#821](https://github.com/cBournhonesque/lightyear/issues/821)**
    - Fix hashing in bullet-prespawn example ([`6d6fd30`](https://github.com/cBournhonesque/lightyear/commit/6d6fd30437f95329c62b95458ed16c34a4c61120))
 * **[#822](https://github.com/cBournhonesque/lightyear/issues/822)**
    - Add alias for Replicate ([`eeb885d`](https://github.com/cBournhonesque/lightyear/commit/eeb885dc6c4704087bf2614d22320c45841e84ce))
 * **[#823](https://github.com/cBournhonesque/lightyear/issues/823)**
    - Add metrics for inputs ([`6f2e5b4`](https://github.com/cBournhonesque/lightyear/commit/6f2e5b4bb7cb5c304b7cc7417ddfe8d4d15f78c0))
 * **[#824](https://github.com/cBournhonesque/lightyear/issues/824)**
    - Allow not replicating hierarchy ([`b357491`](https://github.com/cBournhonesque/lightyear/commit/b357491c9577faf2596b9515d20e5f6b243ac92f))
 * **[#825](https://github.com/cBournhonesque/lightyear/issues/825)**
    - Improve visual interp ([`6574b19`](https://github.com/cBournhonesque/lightyear/commit/6574b19e98e0471af7edeebd1c87fe4b747307ea))
 * **[#826](https://github.com/cBournhonesque/lightyear/issues/826)**
    - Move UpdateVisualInterpolation to FixedLast ([`035d081`](https://github.com/cBournhonesque/lightyear/commit/035d0813bda02d1d26aed7f995472336266ac9e2))
 * **[#831](https://github.com/cBournhonesque/lightyear/issues/831)**
    - Make history buffer pub ([`076bad5`](https://github.com/cBournhonesque/lightyear/commit/076bad5634d4bafb9e885faec21c9810d4539a5e))
 * **[#832](https://github.com/cBournhonesque/lightyear/issues/832)**
    - Add intoiterator for historybuffer ([`7154478`](https://github.com/cBournhonesque/lightyear/commit/7154478783c90bc46c9320131d16816e6c1d7e4e))
 * **[#838](https://github.com/cBournhonesque/lightyear/issues/838)**
    - Fix ([`db2edcf`](https://github.com/cBournhonesque/lightyear/commit/db2edcfb4e873c447f26e81ec00da49365ccbb22))
 * **[#841](https://github.com/cBournhonesque/lightyear/issues/841)**
    - Separate new() and new_with_app_id() ([`1d1a3e9`](https://github.com/cBournhonesque/lightyear/commit/1d1a3e99ea58d4cc824ebd84701bc731be207366))
 * **[#844](https://github.com/cBournhonesque/lightyear/issues/844)**
    - Emit disconnect event on connection failure ([`e5018a5`](https://github.com/cBournhonesque/lightyear/commit/e5018a5a1d361f5e4b050885a2dc2a98037269c4))
 * **[#845](https://github.com/cBournhonesque/lightyear/issues/845)**
    - Commands.trigger -> world.trigger; properly handle steam disconnection reasons in the absence of the DisconnectReason type ([`d3b4aa6`](https://github.com/cBournhonesque/lightyear/commit/d3b4aa64e5cf5a7da14b5365e0c0d2884c29b992))
 * **[#846](https://github.com/cBournhonesque/lightyear/issues/846)**
    - Perform state transition on disconnects that happen at connect attempt time ([`bc749d6`](https://github.com/cBournhonesque/lightyear/commit/bc749d6185ce8751cbf97ec52d8c47de547fcd80))
 * **[#849](https://github.com/cBournhonesque/lightyear/issues/849)**
    - Add lag compensation ([`db78764`](https://github.com/cBournhonesque/lightyear/commit/db78764289dd410eea511c516be55772c1970e23))
 * **[#853](https://github.com/cBournhonesque/lightyear/issues/853)**
    - 0.19 ([`e61e01a`](https://github.com/cBournhonesque/lightyear/commit/e61e01a96417164de1eddeee7d52ca6cb6c497af))
 * **[#854](https://github.com/cBournhonesque/lightyear/issues/854)**
    - Improve error handling and reporting for Netcode ([`84394a7`](https://github.com/cBournhonesque/lightyear/commit/84394a7c2a124c5ddb3268529c41bfb770291840))
 * **[#856](https://github.com/cBournhonesque/lightyear/issues/856)**
    - Clean registry serialization ([`795dc07`](https://github.com/cBournhonesque/lightyear/commit/795dc07b0e11a090571d6a313f70fc1fb4141114))
 * **[#857](https://github.com/cBournhonesque/lightyear/issues/857)**
    - Add batch write for replication receive ([`23c78ae`](https://github.com/cBournhonesque/lightyear/commit/23c78ae54a7818b7ae8fb1e38c0350e427bf01f8))
 * **[#860](https://github.com/cBournhonesque/lightyear/issues/860)**
    - Allow confirmed->predicted entity mapping to fail ([`2011ff8`](https://github.com/cBournhonesque/lightyear/commit/2011ff894ec08368041437731e29035fb6f003d0))
 * **[#861](https://github.com/cBournhonesque/lightyear/issues/861)**
    - Batch sync from Confirmed to Predicted ([`b5284b0`](https://github.com/cBournhonesque/lightyear/commit/b5284b0c8301910e5d8678aa7d9559e04e17c54e))
 * **[#862](https://github.com/cBournhonesque/lightyear/issues/862)**
    - Fix ReplicateOnce components not being replicated to new clients ([`7427083`](https://github.com/cBournhonesque/lightyear/commit/7427083ffa5b35f71e9d047490a8080bbd59d5fa))
 * **[#864](https://github.com/cBournhonesque/lightyear/issues/864)**
    - Use dynamic system param for message events ([`b1bceab`](https://github.com/cBournhonesque/lightyear/commit/b1bceab49d625a906631c2de1c7a56b867b87504))
 * **[#871](https://github.com/cBournhonesque/lightyear/issues/871)**
    - Introduce parallel send_message API ([`f955d31`](https://github.com/cBournhonesque/lightyear/commit/f955d311b1a96c6985b44010d0996fe82a94f9c0))
 * **[#872](https://github.com/cBournhonesque/lightyear/issues/872)**
    - Avoid panic when using PrePredicted in HostServer mode ([`2fb6cd4`](https://github.com/cBournhonesque/lightyear/commit/2fb6cd4f9dea1d5136f14a95dc196ff5f7d2892a))
 * **[#873](https://github.com/cBournhonesque/lightyear/issues/873)**
    - Simplify HostServer configuration ([`28cfd40`](https://github.com/cBournhonesque/lightyear/commit/28cfd4042c6b87d089c970afacda6013ba7a3716))
 * **[#874](https://github.com/cBournhonesque/lightyear/issues/874)**
    - Simplify InputMessage logic ([`8c6eeab`](https://github.com/cBournhonesque/lightyear/commit/8c6eeab91bfd536d2d414f8886425d207472f764))
 * **[#876](https://github.com/cBournhonesque/lightyear/issues/876)**
    - Fix ([`81092a0`](https://github.com/cBournhonesque/lightyear/commit/81092a029ede950a7ec03568ab987b4cb7a68699))
 * **[#877](https://github.com/cBournhonesque/lightyear/issues/877)**
    - Update ClientCommands and ServerCommands to work on World ([`8e277cb`](https://github.com/cBournhonesque/lightyear/commit/8e277cba623b6f391097ded8c884a74c2cca8d90))
 * **[#881](https://github.com/cBournhonesque/lightyear/issues/881)**
    - Simplify prespawning using observers ([`f3aea02`](https://github.com/cBournhonesque/lightyear/commit/f3aea02321b21ad3a22e8bd2881a055f112b7dec))
 * **[#884](https://github.com/cBournhonesque/lightyear/issues/884)**
    - Pub input messages for hostserver input replication ([`879bda7`](https://github.com/cBournhonesque/lightyear/commit/879bda7fbeacaf88b79d641d986b407a125aeaf2))
 * **[#896](https://github.com/cBournhonesque/lightyear/issues/896)**
    - Bevy Input replication to clients bug fix ([`2c67d65`](https://github.com/cBournhonesque/lightyear/commit/2c67d65786c49856ce9ed59f35dc41195e4b7b00))
 * **[#902](https://github.com/cBournhonesque/lightyear/issues/902)**
    - Remove confusing duplicate replication interval settings ([`70b6017`](https://github.com/cBournhonesque/lightyear/commit/70b6017db28a52800058fd95ff2e2eeb7a323bec))
 * **[#903](https://github.com/cBournhonesque/lightyear/issues/903)**
    - Fix fragment too big and examples ([`d7ff0b0`](https://github.com/cBournhonesque/lightyear/commit/d7ff0b0ead980b0ef4b6cb69ec80f1b3fdc3c5c2))
 * **[#908](https://github.com/cBournhonesque/lightyear/issues/908)**
    - Do not apply correction if the component was not mispredicted ([`9a9cee4`](https://github.com/cBournhonesque/lightyear/commit/9a9cee45211fd2c3f4f4314c4ca9e146bfda14c2))
 * **[#913](https://github.com/cBournhonesque/lightyear/issues/913)**
    - Improve correction and add test ([`d439dd9`](https://github.com/cBournhonesque/lightyear/commit/d439dd971ffbe32742f547ac4ae08e79d0b21630))
 * **[#916](https://github.com/cBournhonesque/lightyear/issues/916)**
    - Add option to exclude an entity from rollback ([`3748543`](https://github.com/cBournhonesque/lightyear/commit/3748543999b82f9bd74e8ae9b9f27e4404da9ed5))
 * **[#917](https://github.com/cBournhonesque/lightyear/issues/917)**
    - Add test for multiple prespawn entities with the same hash ([`ac412ab`](https://github.com/cBournhonesque/lightyear/commit/ac412ab69d181ab5a6edb15e5595ab4854f6decd))
 * **[#919](https://github.com/cBournhonesque/lightyear/issues/919)**
    - Fix system ordering with avian + correction ([`0f51455`](https://github.com/cBournhonesque/lightyear/commit/0f51455b0cc38169253dba81038b7f699e5d6340))
 * **[#923](https://github.com/cBournhonesque/lightyear/issues/923)**
    - Update latest_tick correctly in receive_update ([`dd7f392`](https://github.com/cBournhonesque/lightyear/commit/dd7f3920da9e2c6bb4dcffb65d88da4ae2440943))
 * **[#924](https://github.com/cBournhonesque/lightyear/issues/924)**
    - Fix correction ([`e7a6e95`](https://github.com/cBournhonesque/lightyear/commit/e7a6e95c3d5e58cc0f37c0933fc34448bc42435e))
 * **[#925](https://github.com/cBournhonesque/lightyear/issues/925)**
    - Do not rollback on initial Confirmed spawn ([`c4b44d9`](https://github.com/cBournhonesque/lightyear/commit/c4b44d9e916fbccc9d405fb7bc486cddd349af50))
 * **[#928](https://github.com/cBournhonesque/lightyear/issues/928)**
    - Apply TickEvent to Correction ([`8c672f2`](https://github.com/cBournhonesque/lightyear/commit/8c672f2a1d39eec6e4698ecf586f5a0bad8b0e9c))
 * **[#935](https://github.com/cBournhonesque/lightyear/issues/935)**
    - Unify leafwing and native input plugins ([`4164799`](https://github.com/cBournhonesque/lightyear/commit/41647998c086778e7cafcbf27be008603a995b4c))
 * **[#943](https://github.com/cBournhonesque/lightyear/issues/943)**
    - Fix correction issue ([`bc9e898`](https://github.com/cBournhonesque/lightyear/commit/bc9e8988076f95ed510cf44076eb65e3bf2db542))
 * **[#950](https://github.com/cBournhonesque/lightyear/issues/950)**
    - Fix host-server mode for avian_physics and spaceships example ([`e37c732`](https://github.com/cBournhonesque/lightyear/commit/e37c7325a1e977225c77d2e3bebd13c8f350b92c))
 * **[#951](https://github.com/cBournhonesque/lightyear/issues/951)**
    - Make SteamworksClient construction fallible ([`7e398fc`](https://github.com/cBournhonesque/lightyear/commit/7e398fc66cb9a5742cdb9ac36f99d528c133c23e))
 * **[#954](https://github.com/cBournhonesque/lightyear/issues/954)**
    - Fix Inputs ([`17f63d1`](https://github.com/cBournhonesque/lightyear/commit/17f63d1943765775d2750680dcc1efc375e7f507))
 * **[#955](https://github.com/cBournhonesque/lightyear/issues/955)**
    - Make prediction_despawn be able to run on the server or the client ([`a8bbbf4`](https://github.com/cBournhonesque/lightyear/commit/a8bbbf4a8217f6a41bd35d9c7bf58aea36ae8fad))
 * **[#958](https://github.com/cBournhonesque/lightyear/issues/958)**
    - Fix prespawn tick mismatch ([`e10b7dc`](https://github.com/cBournhonesque/lightyear/commit/e10b7dcc87dd3ac118c5c76e7bf8cbbc69a99923))
 * **[#959](https://github.com/cBournhonesque/lightyear/issues/959)**
    - Fix issue where we could apply an Update message that is older than the latest Action message ([`d6d03cc`](https://github.com/cBournhonesque/lightyear/commit/d6d03cc1458a00933636b96a9f27b3bb6d945c47))
 * **[#962](https://github.com/cBournhonesque/lightyear/issues/962)**
    - Add required component bounds ([`be14f3d`](https://github.com/cBournhonesque/lightyear/commit/be14f3d8c9ed17c9d3f8459aeabe03e7504677b1))
 * **[#965](https://github.com/cBournhonesque/lightyear/issues/965)**
    - Change logs to info ([`76c52c0`](https://github.com/cBournhonesque/lightyear/commit/76c52c0d61ee02baba89f4f2129a7d0f4200cb1d))
 * **[#976](https://github.com/cBournhonesque/lightyear/issues/976)**
    - Upgrade wtransport ([`eeb514b`](https://github.com/cBournhonesque/lightyear/commit/eeb514b0842a9d23144a4e1f5b7fb77f458238d4))
 * **[#982](https://github.com/cBournhonesque/lightyear/issues/982)**
    - Fix pre-prediction obs ([`0ab23b4`](https://github.com/cBournhonesque/lightyear/commit/0ab23b46faf579a55661c389df108427cbd2e68d))
 * **[#989](https://github.com/cBournhonesque/lightyear/issues/989)**
    - Bevy main refactor ([`b236123`](https://github.com/cBournhonesque/lightyear/commit/b236123c8331f9feea8c34cb9e0d6a179bb34918))
 * **[#999](https://github.com/cBournhonesque/lightyear/issues/999)**
    - Cb/0.20 ([`e7a259a`](https://github.com/cBournhonesque/lightyear/commit/e7a259ab58b6c0caa10d7f7924d443268b2a456d))
 * **Uncategorized**
    - Release 0.21.0 ([`7fe5e08`](https://github.com/cBournhonesque/lightyear/commit/7fe5e08d715fa55ad003270be95139b003aca396))
    - Adjusting changelogs prior to release of lightyear_serde v0.21.0, lightyear_utils v0.21.0, lightyear_core v0.21.0, lightyear_link v0.21.0, lightyear_aeronet v0.21.0, lightyear_connection v0.21.0, lightyear_macros v0.21.0, lightyear_transport v0.21.0, lightyear_messages v0.21.0, lightyear_replication v0.21.0, lightyear_sync v0.21.0, lightyear_interpolation v0.21.0, lightyear_prediction v0.21.0, lightyear_frame_interpolation v0.21.0, lightyear_avian2d v0.21.0, lightyear_avian3d v0.21.0, lightyear_crossbeam v0.21.0, lightyear_inputs v0.21.0, lightyear_inputs_bei v0.21.0, lightyear_inputs_leafwing v0.21.0, lightyear_inputs_native v0.21.0, lightyear_netcode v0.21.0, lightyear_steam v0.21.0, lightyear_webtransport v0.21.0, lightyear_udp v0.21.0, lightyear v0.21.0 ([`6ed9ae9`](https://github.com/cBournhonesque/lightyear/commit/6ed9ae95f9a75a9803c75c56c4e81f40f72fc3c8))
    - Release lightyear_serde v0.21.0-rc.3, lightyear_utils v0.21.0-rc.3, lightyear_core v0.21.0-rc.3, lightyear_link v0.21.0-rc.3, lightyear_aeronet v0.21.0-rc.3, lightyear_connection v0.21.0-rc.3, lightyear_macros v0.21.0-rc.3, lightyear_transport v0.21.0-rc.3, lightyear_messages v0.21.0-rc.3, lightyear_replication v0.21.0-rc.3, lightyear_sync v0.21.0-rc.3, lightyear_interpolation v0.21.0-rc.3, lightyear_prediction v0.21.0-rc.3, lightyear_frame_interpolation v0.21.0-rc.3, lightyear_avian2d v0.21.0-rc.3, lightyear_avian3d v0.21.0-rc.3, lightyear_crossbeam v0.21.0-rc.3, lightyear_inputs v0.21.0-rc.3, lightyear_inputs_bei v0.21.0-rc.3, lightyear_inputs_leafwing v0.21.0-rc.3, lightyear_inputs_native v0.21.0-rc.3, lightyear_netcode v0.21.0-rc.3, lightyear_steam v0.21.0-rc.3, lightyear_webtransport v0.21.0-rc.3, lightyear_udp v0.21.0-rc.3, lightyear v0.21.0-rc.3 ([`134306e`](https://github.com/cBournhonesque/lightyear/commit/134306eaf4e23d2f609c8a7c93adc3c55618ff11))
    - Release rc3 ([`5dc2e81`](https://github.com/cBournhonesque/lightyear/commit/5dc2e81f8c2b1171df33703d73e38a49e7b4695d))
    - Fix compiletime benchmark ([`cc8433c`](https://github.com/cBournhonesque/lightyear/commit/cc8433c61122e6f8c712c3463d0e91d5230290e7))
    - Fix ci ([`f9bc3e3`](https://github.com/cBournhonesque/lightyear/commit/f9bc3e3d8322d252d80363f716d5e78782520cff))
    - Fix ([`9436dd6`](https://github.com/cBournhonesque/lightyear/commit/9436dd60efc0604f874dc09abe43c4dff12579fb))
    - Cargo fmt ([`ade88ca`](https://github.com/cBournhonesque/lightyear/commit/ade88cad9e463e79f3251e55e8eeb18182deb5e3))
    - Fix tests, cargo doc, cargo clippy ([`fe0bb4a`](https://github.com/cBournhonesque/lightyear/commit/fe0bb4a24112a308eaf9c829fe5cfae0180ef946))
    - Fix clippy ([`249b40f`](https://github.com/cBournhonesque/lightyear/commit/249b40f358977f6f85e269967d3912bfb4080f73))
    - Cargo fmt ([`f55c117`](https://github.com/cBournhonesque/lightyear/commit/f55c117c1627368978d26c788efbcb2ddda1da01))
    - Fix lints ([`9040874`](https://github.com/cBournhonesque/lightyear/commit/904087429078e4bbda90a01edd0a0bad68801767))
    - Fix simple_setup example ([`b0b392b`](https://github.com/cBournhonesque/lightyear/commit/b0b392bb319c0fc79ec71111b5366b6726edad41))
    - Fix for wtransport ([`67a9063`](https://github.com/cBournhonesque/lightyear/commit/67a90632748a4fe3b92f06775140babbf8ebca42))
    - Make ReadyBuffer pub ([`e77e3e5`](https://github.com/cBournhonesque/lightyear/commit/e77e3e53d17b87dfd37e768dfa20c90bb273a93b))
    - Fix noisy log ([`9f3f3c0`](https://github.com/cBournhonesque/lightyear/commit/9f3f3c02d2377662e645d6719217b9763d539af0))
    - Add public  method ([`c1e02e7`](https://github.com/cBournhonesque/lightyear/commit/c1e02e776375b20134fb89b3bc720389c9b4ce6e))
    - Add public  method ([`e894b6c`](https://github.com/cBournhonesque/lightyear/commit/e894b6c22aae03d7b810f74cd9550b87acfda595))
    - Add comment on DisabledComponents ([`34765ad`](https://github.com/cBournhonesque/lightyear/commit/34765ad8342db5195c1bebd9ab7ae0d359f3aabf))
    - Fix ([`1332df3`](https://github.com/cBournhonesque/lightyear/commit/1332df36dd5c89928412cfa3bec380029d08a518))
    - Add Debug to confirmed ([`f38c88f`](https://github.com/cBournhonesque/lightyear/commit/f38c88f0459645f916f296688ab97a532e82bf77))
    - Relax Event bound on trigger messages ([`9a34877`](https://github.com/cBournhonesque/lightyear/commit/9a348775d0f47f6b59384ecf5a7214b145be8754))
    - Remove dbg ([`cee4cd6`](https://github.com/cBournhonesque/lightyear/commit/cee4cd6d5c7daebec8ea6f724fcb4c30fd6da1e9))
    - Fix correction for multiple frames without FixedUpdate ([`6bd5b26`](https://github.com/cBournhonesque/lightyear/commit/6bd5b267117c529390141a06dd24531967fdb0f0))
    - Cargo fix ([`95abca4`](https://github.com/cBournhonesque/lightyear/commit/95abca4e25020cc4da17dc7edcbc33bd4f3a5c95))
    - Fix tests ([`96cabc9`](https://github.com/cBournhonesque/lightyear/commit/96cabc97496b537321364802fc87a2c3d00d9412))
    - Revert initial rollback on confirmed spawn ([`b8e3cb9`](https://github.com/cBournhonesque/lightyear/commit/b8e3cb9cbff7d420a3495e4f614fa15b51e17554))
    - Fix ([`ccdde09`](https://github.com/cBournhonesque/lightyear/commit/ccdde09a99e7784625cdbc72acfd97c79f8a3926))
    - Provide extra alias for message events ([`d213113`](https://github.com/cBournhonesque/lightyear/commit/d213113fc78596633aaddc7ca624cb66d0761242))
    - Fix Preprediction ([`ab1af1e`](https://github.com/cBournhonesque/lightyear/commit/ab1af1ea9af1b157dca8a87d59c90d0d30e123ff))
    - Fix ([`f677ed9`](https://github.com/cBournhonesque/lightyear/commit/f677ed9d6bfca9bd2c6485e757a00831319d464d))
    - Also trigger the error ([`4b3c31b`](https://github.com/cBournhonesque/lightyear/commit/4b3c31b27098868a21911a372ce34d8f6603f447))
    - Fix example and fix NetworkTarget::Single serialization ([`7bcee01`](https://github.com/cBournhonesque/lightyear/commit/7bcee01084130217bb49ab2d5a50015b85e73e89))
    - Fix check ([`bc0301d`](https://github.com/cBournhonesque/lightyear/commit/bc0301d5b59271e709da8e8e5006a4b3bd0100b3))
    - Avoid panics when history tick is older than last history tick ([`f194510`](https://github.com/cBournhonesque/lightyear/commit/f1945105d1a138b831bd9f14f14c14cb86b0212c))
    - Fix timeout ([`2994029`](https://github.com/cBournhonesque/lightyear/commit/2994029e6e40d4bf83a2b0165d5c98b6fef15d46))
    - Fix' ([`20eefe3`](https://github.com/cBournhonesque/lightyear/commit/20eefe33362232cff5c938aa74e19860bb711f30))
    - Fix ([`542705d`](https://github.com/cBournhonesque/lightyear/commit/542705da7eb5e51e96dfef3cd2a7bcad7fae993d))
    - Fix ([`ab22c33`](https://github.com/cBournhonesque/lightyear/commit/ab22c3358892838919df75749377a12e0ed0cd38))
    - Remove error logs when server is stopping ([`11cd319`](https://github.com/cBournhonesque/lightyear/commit/11cd31961281a8621c520c861c624a388d7e912b))
    - Remove logs ([`6c55b07`](https://github.com/cBournhonesque/lightyear/commit/6c55b07ca3ba775731f1d93ec70822739da1e81d))
    - Transfer authority immediately to client if AuthorityPeer::Client was added on the server ([`cd2164c`](https://github.com/cBournhonesque/lightyear/commit/cd2164cde5f978a91114041030f5db74063d564c))
    - Fix debug log ([`7e876f8`](https://github.com/cBournhonesque/lightyear/commit/7e876f88c31a31ff64bbae009b744007144aea39))
    - Add unit test for native inputs in host server mode ([`beef9a7`](https://github.com/cBournhonesque/lightyear/commit/beef9a724c862c0517c09da292b6a48158a60f1c))
    - Fix ([`a922051`](https://github.com/cBournhonesque/lightyear/commit/a92205176d83e03e1b50122ee61e77d12718a892))
    - Fix ([`2f55d0c`](https://github.com/cBournhonesque/lightyear/commit/2f55d0ca3ba7b89267fd93205a025bec382005ce))
    - Add feature ([`a4d414a`](https://github.com/cBournhonesque/lightyear/commit/a4d414aa8a7f4b56f675dc81048252b8d376fd40))
    - Lightyear 0.16.4 ([`7f934a1`](https://github.com/cBournhonesque/lightyear/commit/7f934a1fe3b60902634069ede9929b04985c2984))
    - Make field public ([`356f61d`](https://github.com/cBournhonesque/lightyear/commit/356f61d26ad78881e7d4c6399d52174148c0f31c))
    - Make DeltaManager public ([`58e0fa3`](https://github.com/cBournhonesque/lightyear/commit/58e0fa3dad0ef71aa1b2dd02961f04017de5b809))
    - Typo ([`cc016db`](https://github.com/cBournhonesque/lightyear/commit/cc016db80e0de866979c8420fb0b2ad7792b4c8b))
    - Add order constraint on receive replication ([`c3af31c`](https://github.com/cBournhonesque/lightyear/commit/c3af31cdaf39f00de684710edefe86bb56268a60))
    - Make leafwing public ([`04af84e`](https://github.com/cBournhonesque/lightyear/commit/04af84e2da6d66d9fb89aa561f8cf13f7488df2f))
    - Trigger Connect/Disconnect events on client ([`0471d84`](https://github.com/cBournhonesque/lightyear/commit/0471d84da671693c87ef0b764c0994d325503e9a))
    - Fix visual interpolation tests ([`2be4eeb`](https://github.com/cBournhonesque/lightyear/commit/2be4eeb2d32a0a2d9d8eca6081dbb917b5169b8a))
    - Upgrade to fix docs again.. ([`a574790`](https://github.com/cBournhonesque/lightyear/commit/a57479042782b29c31510f43da7e093510fce510))
    - Upgrade to 0.16.1 to fix docs ([`551b34f`](https://github.com/cBournhonesque/lightyear/commit/551b34faddb718b1a8deb0272b07b35b4fa8bc2a))
    - Add an send_interval in InputConfig ([`d698128`](https://github.com/cBournhonesque/lightyear/commit/d698128df21528b4cdf861167c75ed892257cc52))
    - Merge pull request #508 from blinkdog/websocket-on-wasm ([`11b8470`](https://github.com/cBournhonesque/lightyear/commit/11b8470cb737d2152872db8a0aea43deedcd8187))
    - Add wasm-bindgen-futures dependency to websocket feature ([`e4b1e4f`](https://github.com/cBournhonesque/lightyear/commit/e4b1e4f32ee442022f3eb01f6a9c8acac0d7a46c))
    - Merge pull request #506 from blinkdog/websocket-without-webtransport ([`10d7a9f`](https://github.com/cBournhonesque/lightyear/commit/10d7a9fd26125ce8e1e24d517d3e31a4a081c6e8))
    - Remove WebTransport use directives from WebSocket module ([`850c55d`](https://github.com/cBournhonesque/lightyear/commit/850c55d06d02c1bcdaa7d383004dde5e4e005128))
    - Merge pull request #501 from RJ/spaceships ([`32323ba`](https://github.com/cBournhonesque/lightyear/commit/32323ba04bea1600fd3a1638ccec8e5f4ac46d1b))
    - Bullet collisions and predict missing inputs ([`c440192`](https://github.com/cBournhonesque/lightyear/commit/c44019268f65cebb6a32881fe9ef94a918cbceba))
    - Merge pull request #480 from RJ/spaceships ([`a5cea32`](https://github.com/cBournhonesque/lightyear/commit/a5cea3200b231dedb522467384254e59eca23203))
    - Explicit annotations for transmute, to please clippy ([`6cbedba`](https://github.com/cBournhonesque/lightyear/commit/6cbedba4c5fbcea9d5e98e70fdf33984e5c5efa8))
    - Use ignore for rustdoc ([`804f599`](https://github.com/cBournhonesque/lightyear/commit/804f599b1646d7975c62ff932a9a412439361afb))
    - Add prespawn hash salt feature, fix cooldown tick wrapping issue ([`d23a390`](https://github.com/cBournhonesque/lightyear/commit/d23a390603fa4684c08ab4ff4fb2309277b485ed))
    - Revert prespawn changes ([`64ef2ad`](https://github.com/cBournhonesque/lightyear/commit/64ef2ade72da4284a44fe381ce08c4f39e1d76bc))
    - Revert changes to prespawning ([`498c6ef`](https://github.com/cBournhonesque/lightyear/commit/498c6efaac9858c9ead9a9e9cdb7340370ace90e))
    - Restore previous behaviour of prespawns ([`06b0a3d`](https://github.com/cBournhonesque/lightyear/commit/06b0a3d383e1925b066bb8dd5e26effa27b232cd))
    - Prep ([`f8ce46f`](https://github.com/cBournhonesque/lightyear/commit/f8ce46f51940ed159aaf1f9eaff35e2c290a72d8))
    - Readme, tidyup ([`cde7bca`](https://github.com/cBournhonesque/lightyear/commit/cde7bca367c0b40eecb779bf8b292d425a9ecd97))
    - More tidying ([`30f2bc0`](https://github.com/cBournhonesque/lightyear/commit/30f2bc07a3228f0cf3b0e5cb430c90d4bbdc9cb6))
    - Expose client::web::KeepaliveSettings so games can configure webworker wake interval ([`4011b11`](https://github.com/cBournhonesque/lightyear/commit/4011b11c00e18acc12757b9b164d83e49d65b694))
    - Show thrust based on last seen input ([`5e674d8`](https://github.com/cBournhonesque/lightyear/commit/5e674d8ca303361594d6e8fa95a3c7dff4b0c914))
    - Draw engines on server too ([`c89c3c2`](https://github.com/cBournhonesque/lightyear/commit/c89c3c2acb0f86075975e4ce2b48281951d50630))
    - Wip shooting work ([`2e403e4`](https://github.com/cBournhonesque/lightyear/commit/2e403e45f8ba54e3f5de5592b1cc7e98245af806))
    - Widen visibility for InputBuffer ([`2318953`](https://github.com/cBournhonesque/lightyear/commit/2318953c8da39aa4dc174d3abb5ba11284ccab81))
    - Fix merge bits ([`b3a5b04`](https://github.com/cBournhonesque/lightyear/commit/b3a5b044e5ac6039d27417e23cc8dc88bd374262))
    - Tidy up after rebasing main ([`47344a8`](https://github.com/cBournhonesque/lightyear/commit/47344a84abc3df972816d382fc8239fa4a1dc4f2))
    - Show labels next to players with rtt/jitter ([`c712a22`](https://github.com/cBournhonesque/lightyear/commit/c712a22d66630147e7e6c395067f067b8106bae6))
    - Merge pull request #476 from cBournhonesque/cb/rename-visibility ([`c69feb4`](https://github.com/cBournhonesque/lightyear/commit/c69feb49fe03368ea32b105c77371fe5fe8ed732))
    - Rename visibility to network relevance ([`897f449`](https://github.com/cBournhonesque/lightyear/commit/897f449ebc41b3dec93f5c3a9af3e0dcbe2557a2))
    - Merge pull request #474 from cBournhonesque/cb/fix-priority ([`19e536d`](https://github.com/cBournhonesque/lightyear/commit/19e536d06ca7b153295f78652c7640953efec3c4))
    - Remove logs ([`4911983`](https://github.com/cBournhonesque/lightyear/commit/4911983f298557cd3b0ac538de44d2ae676d47bc))
    - Wip ([`4259ee3`](https://github.com/cBournhonesque/lightyear/commit/4259ee39138433a110f3b576f9e50bc6e59769e0))
    - Merge pull request #472 from cBournhonesque/cb/send-inputs ([`8a9ab67`](https://github.com/cBournhonesque/lightyear/commit/8a9ab6781b98c0b6e896640d8cbf6b93e12beb4f))
    - Fix action diff panic ([`33cf858`](https://github.com/cBournhonesque/lightyear/commit/33cf858f99afa3b0ba5377c1f24a8f6eb2a5ad41))
    - Remove logs, and stop replicating ActionState directly ([`021c5b8`](https://github.com/cBournhonesque/lightyear/commit/021c5b8e0c451cb970eb9dcd5017e90d401fa88a))
    - Added some unit tests for client ([`eb23d45`](https://github.com/cBournhonesque/lightyear/commit/eb23d4564d3492b422b01a5b4e586e406ef406c7))
    - Added tests for input delay, input delay works but rollback does not ([`9fbcbd9`](https://github.com/cBournhonesque/lightyear/commit/9fbcbd988e93e8f4285e33efa12395acaa22d3dd))
    - Enabled logs ([`0128907`](https://github.com/cBournhonesque/lightyear/commit/0128907a27e670da0ad408ef6c9958207e079af6))
    - Remove ActionDiffBuffer ([`16e846f`](https://github.com/cBournhonesque/lightyear/commit/16e846f0ae64a90a27de32151342ec73a796a810))
    - Fix example ([`7fd93eb`](https://github.com/cBournhonesque/lightyear/commit/7fd93eb34ab2994e0ce55a301e06c192b01fe666))
    - Wip ([`115de0a`](https://github.com/cBournhonesque/lightyear/commit/115de0ab55b4dc6af82a011ba68912c1e7c8beef))
    - Separate input types and include start ActionState in InputMessage ([`8bcb343`](https://github.com/cBournhonesque/lightyear/commit/8bcb3438eefa2be2e738ed9febddf4c15ed32a56))
    - Merge pull request #466 from cBournhonesque/cb/send-interval-per-channel ([`d0f465d`](https://github.com/cBournhonesque/lightyear/commit/d0f465d74773a3c6bb5f57718e19da802236cac5))
    - Fix bug related to newly connected clients being cleared every frame ([`a008a13`](https://github.com/cBournhonesque/lightyear/commit/a008a138b6bc41a51ea8e35fa346d087109f4e7d))
    - Fix priority accumulation ([`42ea377`](https://github.com/cBournhonesque/lightyear/commit/42ea3779b07f79cc490ea404fe459acda2654128))
    - Examples work ([`38ade78`](https://github.com/cBournhonesque/lightyear/commit/38ade7829384f41feaef41e14a34092c1b038a36))
    - Tests pass (the issue was schedule cycles) ([`d2e7b8d`](https://github.com/cBournhonesque/lightyear/commit/d2e7b8db1166f873389d7e4715fca51f6679b498))
    - Improve ([`f05dde2`](https://github.com/cBournhonesque/lightyear/commit/f05dde2c4e462b8f485c1b1a125ce1e40c52a575))
    - Remove send_interval for networking, and add send_interval per channel and per replication_sender ([`6c86320`](https://github.com/cBournhonesque/lightyear/commit/6c86320c067d498f8cd5399c8786fe13655a4bd5))
    - Merge pull request #465 from cBournhonesque/cb/per-replication-group-frequency ([`2403575`](https://github.com/cBournhonesque/lightyear/commit/2403575c3a4aaaa16b327dcc524d4f5fb71ea600))
    - Enable specifying a send_frequency for a ReplicationGroup ([`b32006e`](https://github.com/cBournhonesque/lightyear/commit/b32006ee836d3228a2c4bb7ee1ad64629db20e50))
    - Merge pull request #461 from cBournhonesque/cb/type-erased-replication ([`87bad5b`](https://github.com/cBournhonesque/lightyear/commit/87bad5beec8f84ccc84815124589ad519364436d))
    - Fix docs ([`5853b88`](https://github.com/cBournhonesque/lightyear/commit/5853b88cd65c57fa68195e6501ab250cb93cf6fd))
    - Fix preprediction not removing the correct componenst ([`69102a3`](https://github.com/cBournhonesque/lightyear/commit/69102a35137ce19af83aef20e0d35e983c0bccbb))
    - Update client replication as well, client replication tests pass but replication is buggy still ([`93769db`](https://github.com/cBournhonesque/lightyear/commit/93769dbcdfade02d403fa3b9a38d79026e4502d6))
    - Wip ([`da3803e`](https://github.com/cBournhonesque/lightyear/commit/da3803e2252acc7393b7fab45a1adb84ebedadaa))
    - Type-erased the replication systems ([`16fa38e`](https://github.com/cBournhonesque/lightyear/commit/16fa38e015a7269bac4efff126adfc964457576b))
    - Merge pull request #460 from cBournhonesque/cb/big-message ([`90c4881`](https://github.com/cBournhonesque/lightyear/commit/90c48813eb5e1e3aca89c049046be19fd46a61e3))
    - Flag to enable sending messages bigger than 300MB ([`90b71b4`](https://github.com/cBournhonesque/lightyear/commit/90b71b48ae7cc9abdba94ced2c9354b61941d981))
    - Merge pull request #459 from cBournhonesque/cb/single-replication-group ([`24235f2`](https://github.com/cBournhonesque/lightyear/commit/24235f21228557d1497ab708e127c0dc587f160f))
    - More efficient writes ([`be8517e`](https://github.com/cBournhonesque/lightyear/commit/be8517eb674b38154fa4b15ec6af0c779380f5aa))
    - Use BytesMut to reduce the number of allocations at write time ([`710b6bd`](https://github.com/cBournhonesque/lightyear/commit/710b6bd50a59b16a117fbaccecc22e4eb2bf6b64))
    - Merge pull request #458 from cBournhonesque/cb/fix-build-pack ([`e83de1d`](https://github.com/cBournhonesque/lightyear/commit/e83de1da3d74ad54a291481e75b699a571e19fa6))
    - Remove unused variables ([`212b8d6`](https://github.com/cBournhonesque/lightyear/commit/212b8d64b3ac4736ae0b9b888112b86c1788f4ed))
    - A ([`fb2c6c8`](https://github.com/cBournhonesque/lightyear/commit/fb2c6c8595ab38bf0334195e1f16923b87e19cb4))
    - Merge pull request #454 from cBournhonesque/cb/fragment-too-big ([`452a02a`](https://github.com/cBournhonesque/lightyear/commit/452a02a72216d33114366825a9a2f11ffe5f618e))
    - Buffer_send returns a Result in case buffering the message fails ([`07f9e1f`](https://github.com/cBournhonesque/lightyear/commit/07f9e1fad6f90650e9cab54692d24484d37adf15))
    - Merge pull request #452 from cBournhonesque/cb/refactor-serialize ([`fa13fa1`](https://github.com/cBournhonesque/lightyear/commit/fa13fa1eaf3c9072393326bfc57d06e98a060c1d))
    - All tests pass ([`b0aa0cc`](https://github.com/cBournhonesque/lightyear/commit/b0aa0ccc1128308b0816fb2d17bc0c845333a992))
    - Working apart from Client to Server messages ([`1d5c520`](https://github.com/cBournhonesque/lightyear/commit/1d5c520e2a7a14847cce77d7c4d51cc596d93045))
    - Merge pull request #451 from cBournhonesque/cb/fix-visual-interpolation-ordering ([`7297f5a`](https://github.com/cBournhonesque/lightyear/commit/7297f5ad635bd9d9b884bcac570a7b71cbc0d95b))
    - Add ToBytes implementation for replication messages ([`800ffec`](https://github.com/cBournhonesque/lightyear/commit/800ffec44f292c440490285c35b3f5874dbe228c))
    - Tests compile ([`5ba4b56`](https://github.com/cBournhonesque/lightyear/commit/5ba4b56404809d7e59d2e3a18e505f66874fa7f8))
    - Fix warnings ([`d601a07`](https://github.com/cBournhonesque/lightyear/commit/d601a07c98c1eed7e81ba42e506e8cf64bba5135))
    - Use Bytes everywhere ([`8a16b18`](https://github.com/cBournhonesque/lightyear/commit/8a16b188a02856d50f6686398b1db20da56a8ae9))
    - Apply visual interpolation before transform propagate ([`20a2891`](https://github.com/cBournhonesque/lightyear/commit/20a289122b640a85dec04f4e9104d70cafc49982))
    - Wip ([`29811be`](https://github.com/cBournhonesque/lightyear/commit/29811bec5d175bb1c1b9cb1570410f7cc0f107dc))
    - Receive Bytes and make SingleData and FragmentData re-use subset of these bytes ([`265ee52`](https://github.com/cBournhonesque/lightyear/commit/265ee5260d18097e95399395104ddbc85e7a4d6c))
    - Builds ([`f7e234d`](https://github.com/cBournhonesque/lightyear/commit/f7e234d1d7f5483b5065dde5b6dc5971f701af1b))
    - Remove bitcode ([`58ed219`](https://github.com/cBournhonesque/lightyear/commit/58ed2197828fc97f20de1159a1863d2b4f908895))
    - Remove bitcode ([`88192ea`](https://github.com/cBournhonesque/lightyear/commit/88192ea4276f704d372832482d054bb3fd05eb8c))
    - Merge pull request #446 from cBournhonesque/cb/separate-replication-message ([`2b2f618`](https://github.com/cBournhonesque/lightyear/commit/2b2f618c1c02751141891116180953b599307814))
    - Lints ([`8eff608`](https://github.com/cBournhonesque/lightyear/commit/8eff6085b81ad49488f1bb9daa3efb920020e5a8))
    - Fix sending multiple messages in the same channel ([`81dc21a`](https://github.com/cBournhonesque/lightyear/commit/81dc21a88e74aba538f73769b8303eaf5f4d8eab))
    - Tests pass ([`4898249`](https://github.com/cBournhonesque/lightyear/commit/489824921ec921ae24a50dc8bdc3d44cd13f73a2))
    - Merge branch 'main' into cb/separate-replication-message ([`9201711`](https://github.com/cBournhonesque/lightyear/commit/92017118aee7b9805d7b8749f1eb70d24ae58fc0))
    - Merge pull request #447 from cBournhonesque/cb/remove-unused-imports-leafwing ([`9f67aaf`](https://github.com/cBournhonesque/lightyear/commit/9f67aaf5ce534cbfa4b4e115d485b443ec1ffc9a))
    - Merge branch 'main' into cb/remove-unused-imports-leafwing ([`3f4a7c3`](https://github.com/cBournhonesque/lightyear/commit/3f4a7c386756fe6036a5a6e24be1c8591f0a3f79))
    - Fixes ([`1c32467`](https://github.com/cBournhonesque/lightyear/commit/1c3246743243e5216d01f4c16037623afd5a41e9))
    - Fix unused imports ([`4d162b6`](https://github.com/cBournhonesque/lightyear/commit/4d162b612118279b81ece6219ffa9c82d0f09823))
    - Merge pull request #445 from cBournhonesque/dependabot/cargo/tokio-tungstenite-0.23.0 ([`dd48070`](https://github.com/cBournhonesque/lightyear/commit/dd48070c82bbb0aee3d862f5638149b518bfb63f))
    - Merge pull request #441 from cBournhonesque/dependabot/cargo/metrics-0.23 ([`b1768d4`](https://github.com/cBournhonesque/lightyear/commit/b1768d4852e24d19ed8aa1c2e5ba319a3e46360e))
    - Update tests ([`891fe13`](https://github.com/cBournhonesque/lightyear/commit/891fe13ab127f7a2da8c844dd98a897a8e02f616))
    - Fix replication updates ([`45d4106`](https://github.com/cBournhonesque/lightyear/commit/45d41067055e4735658dcd9afaf659aec59f703d))
    - Refactor to appease borrow checker :( ([`4f5a0df`](https://github.com/cBournhonesque/lightyear/commit/4f5a0df95342d7fe08ddf83aef18517b48424fe5))
    - Update tokio-tungstenite requirement from 0.21.0 to 0.23.0 ([`1c856a0`](https://github.com/cBournhonesque/lightyear/commit/1c856a0d9d9b72374a656da189e94f4bac097d88))
    - Update metrics requirement from 0.22 to 0.23 ([`536de2c`](https://github.com/cBournhonesque/lightyear/commit/536de2cbbe78559761e0a331b5d13f8298b714af))
    - Wip ([`1eeb0f9`](https://github.com/cBournhonesque/lightyear/commit/1eeb0f9307c3bb763b0a3ae7c10dcadb5493f493))
    - Only errors left are weird iterator lifetimes.. ([`6cb484d`](https://github.com/cBournhonesque/lightyear/commit/6cb484d4c0bd991b790916577179170b0f880082))
    - Merge pull request #440 from cBournhonesque/cb/tiny-vec ([`f947af7`](https://github.com/cBournhonesque/lightyear/commit/f947af7846a9892f527aeb46ba28aa5a5a33b81e))
    - Cleanup unused deps ([`d68a2f0`](https://github.com/cBournhonesque/lightyear/commit/d68a2f08d0775d85230644285507509aeeb205c8))
    - Merge pull request #439 from cBournhonesque/cb/fix-docs ([`2425120`](https://github.com/cBournhonesque/lightyear/commit/2425120ea9fe5e83d9fb18ee5def7cbfe7da2348))
    - Fix docs ([`2760608`](https://github.com/cBournhonesque/lightyear/commit/276060866c71aec3cf00797958496fcba84076a9))
    - Fix steam ([`403cec9`](https://github.com/cBournhonesque/lightyear/commit/403cec9f6d8267d0fe7d208c84c2f32d2f328541))
    - Fix doc ([`ee9abea`](https://github.com/cBournhonesque/lightyear/commit/ee9abea313562c640f48035e4335f98b05c32756))
    - Merge pull request #438 from cBournhonesque/cb/remove-unused-imports ([`dc632c4`](https://github.com/cBournhonesque/lightyear/commit/dc632c41c32d16da30b9de0577699a670d75ff0b))
    - Lints ([`9ec0660`](https://github.com/cBournhonesque/lightyear/commit/9ec06600039a4a155bb88db29942c849ed23a958))
    - Remove unused imports ([`9c068b2`](https://github.com/cBournhonesque/lightyear/commit/9c068b289670f94bb447de7253f6f81a9a0e0231))
    - Remove unused imports ([`4f037ac`](https://github.com/cBournhonesque/lightyear/commit/4f037ac69d1bb73088629632d27c5966ef2fef26))
    - Merge pull request #434 from cBournhonesque/cb/remove-anyhow-packet ([`5558683`](https://github.com/cBournhonesque/lightyear/commit/555868345e9966cb4dc17c5313e1f84db893da99))
    - Remove anyhow ([`5f6b75f`](https://github.com/cBournhonesque/lightyear/commit/5f6b75f6947cc94724d7a902e2205d70cdaeb075))
    - Remove anyhow everywhere but connection folder ([`ad13b10`](https://github.com/cBournhonesque/lightyear/commit/ad13b10aaed31355371b04fd58d4590542308242))
    - Remove anyhow ([`2bb8b73`](https://github.com/cBournhonesque/lightyear/commit/2bb8b73a747786c6c950919fc15d47417690a094))
    - Introduce ClientError and ServerError ([`d8d7fa6`](https://github.com/cBournhonesque/lightyear/commit/d8d7fa6234af5f5ef595cfe039b26720a96d1f8a))
    - Merge pull request #433 from cBournhonesque/cb/add-lz4 ([`8d4eacd`](https://github.com/cBournhonesque/lightyear/commit/8d4eacd9168a2b230e909209b3f4f7d8c488134b))
    - Add lz4 ([`b5a2295`](https://github.com/cBournhonesque/lightyear/commit/b5a2295ba3317fd8f7a78ce6bf88ff33695ca76f))
    - Merge pull request #432 from cBournhonesque/cb/debug-perf ([`a917672`](https://github.com/cBournhonesque/lightyear/commit/a9176728be999365ce20cfcea4d2b2a1c7cf2305))
    - A ([`817b0bd`](https://github.com/cBournhonesque/lightyear/commit/817b0bd2bcc9b5ccfb4cb564188f0d0ffaf48399))
    - Merge pull request #431 from cBournhonesque/cb/client-replication-to-new-server ([`cef1597`](https://github.com/cBournhonesque/lightyear/commit/cef15974772d56869e6c15a988384c0db6f4c8ea))
    - Remove logs ([`892c4fe`](https://github.com/cBournhonesque/lightyear/commit/892c4feb8dd8ae796e353706c754dc8cf63d0ad0))
    - Fix ([`889f99d`](https://github.com/cBournhonesque/lightyear/commit/889f99d7da611108791b3b79e8e651eb9c97d498))
    - A ([`ba560be`](https://github.com/cBournhonesque/lightyear/commit/ba560be967c05a737569aa10c93ac1ae5ed59f82))
    - Wip ([`772a426`](https://github.com/cBournhonesque/lightyear/commit/772a426fc5b651d5f34b318f7e1de8bb8a13ebb1))
    - Merge pull request #429 from cBournhonesque/cb/register-hierarchy ([`89839ba`](https://github.com/cBournhonesque/lightyear/commit/89839ba84d039cf09ac23591cb540208f40bf8cf))
    - Fix ([`0f9947b`](https://github.com/cBournhonesque/lightyear/commit/0f9947b24a81948bd6aac6cc1f6b1ac234b18dfb))
    - Merge pull request #428 from cBournhonesque/cb/use-bytes ([`81abbee`](https://github.com/cBournhonesque/lightyear/commit/81abbee278a3a1091863bd90b10bbd1b2a232209))
    - Fix docs ([`b1d9b82`](https://github.com/cBournhonesque/lightyear/commit/b1d9b82d463a240f30149ee24e57cf57b7e52574))
    - Clippy ([`49b28ec`](https://github.com/cBournhonesque/lightyear/commit/49b28ec792832a5a0aa51ed20e7ac4c6a339e414))
    - Fix ([`fef5c23`](https://github.com/cBournhonesque/lightyear/commit/fef5c23f0ab0d46924f688b25b85f9f4ad4d3119))
    - Fix all tests ([`453c42b`](https://github.com/cBournhonesque/lightyear/commit/453c42b1751be0c537294ebe986ccd97a4513f78))
    - Tests almost pass ([`e3b480a`](https://github.com/cBournhonesque/lightyear/commit/e3b480a0fd5cc0ebe4d9340ba92de5e0de0902e8))
    - Refactor packet builder ([`852f9ea`](https://github.com/cBournhonesque/lightyear/commit/852f9ea0731137011dcd065108c3e258afc02643))
    - Can pack small messages ([`f0c9c83`](https://github.com/cBournhonesque/lightyear/commit/f0c9c83ed60a9f67256ea8a00c2d69f32efb897e))
    - Fix tests ([`1977c95`](https://github.com/cBournhonesque/lightyear/commit/1977c9569ada705a50195ed4ef124b2079b5b05d))
    - Compiles ([`c605b23`](https://github.com/cBournhonesque/lightyear/commit/c605b23b998a06d5ce7d57f2cbcada301143c3b2))
    - Various fixes ([`66829fa`](https://github.com/cBournhonesque/lightyear/commit/66829fa07c62dffe011286bbe75ff17f235474fa))
    - Get rid of octets ([`443b247`](https://github.com/cBournhonesque/lightyear/commit/443b247be8648c542f9e9d17de434710d4ad56ae))
    - Get rid of octets ([`31a8073`](https://github.com/cBournhonesque/lightyear/commit/31a807364af3085f5528e43ce4973514a3d1818c))
    - Wip fix tests ([`098ff9c`](https://github.com/cBournhonesque/lightyear/commit/098ff9cf669131a894eda04cbc0df74b63134bde))
    - Refactor packet creation to use octets ([`3061341`](https://github.com/cBournhonesque/lightyear/commit/30613410d06d3bf900a769c27060c4d2aa2df173))
    - Merge pull request #424 from cBournhonesque/cb/add-bandwidth-stats ([`80f460c`](https://github.com/cBournhonesque/lightyear/commit/80f460cdc1ded7f9e85512825b702fb231871d5f))
    - Add channel bandwidth stats ([`1e45683`](https://github.com/cBournhonesque/lightyear/commit/1e45683f5ae877ef3a66560146a2f4c307d6f761))
    - Merge pull request #422 from cBournhonesque/cb/profile-replication ([`e9ec829`](https://github.com/cBournhonesque/lightyear/commit/e9ec82995ab7c225f4254d1d6b31d366c390d9d8))
    - Taplo ([`468b4e7`](https://github.com/cBournhonesque/lightyear/commit/468b4e75ca383cce01a1a92e78dac01e4fdb0b42))
    - Add trace feature with lots of trace instrumentation ([`54a41aa`](https://github.com/cBournhonesque/lightyear/commit/54a41aa7f027e5c83d0cc01baab4e7ff150e4cea))
    - Merge pull request #417 from cBournhonesque/cb/remove-none-state ([`886d0a7`](https://github.com/cBournhonesque/lightyear/commit/886d0a77fef9f4fa5e4845d28c90bae52e3fccf7))
    - Cleanpu ([`b2f5ae1`](https://github.com/cBournhonesque/lightyear/commit/b2f5ae1194fd0e9178fdd3fcd2febce2d153bc3f))
    - Remove None state and instead init NetworkingState without entering OnDisconnect ([`97fa229`](https://github.com/cBournhonesque/lightyear/commit/97fa2291b8aef5ecd38a67cc6f7ce99e9a280c52))
    - Merge pull request #416 from cBournhonesque/cb/add-disconnection-error ([`90a7a6d`](https://github.com/cBournhonesque/lightyear/commit/90a7a6d2fdf47903e85b13735b9bf82864b1145f))
    - Merge branch 'main' into cb/add-disconnection-error ([`2326bb6`](https://github.com/cBournhonesque/lightyear/commit/2326bb6fbe36d196821300e2b2334d722635a7a6))
    - Resturn DisconnectReason in the DisconnectEvent for clients ([`3cedb3a`](https://github.com/cBournhonesque/lightyear/commit/3cedb3a1165334f72f55d044fcdaa202db5c8cb2))
    - Merge pull request #414 from cBournhonesque/cb/fix-remote-inputs ([`6c56e4d`](https://github.com/cBournhonesque/lightyear/commit/6c56e4db4d41c9e00c87391166f0ad711caf6ee7))
    - Remove duplicate nack log ([`72b2d26`](https://github.com/cBournhonesque/lightyear/commit/72b2d26ceb1f2c3605bfe2b345a9e3388dead2ea))
    - Fix remote inputs entity mapping ([`f316d19`](https://github.com/cBournhonesque/lightyear/commit/f316d1919530f39739f336ac6fa71f8059964186))
    - Merge pull request #346 from msvbg/steam-p2p ([`03fcf7f`](https://github.com/cBournhonesque/lightyear/commit/03fcf7f06d297bc795e0e7309caf165db2e31bfc))
    - Merge pull request #413 from cBournhonesque/cb/add-benchmarks ([`8e37150`](https://github.com/cBournhonesque/lightyear/commit/8e37150ddc7e0daecbd749113577c31c80c5b67c))
    - Switch benchmark backend to criterion ([`8539d29`](https://github.com/cBournhonesque/lightyear/commit/8539d297ad95599fc7354c80c7af8bff26b9c38a))
    - Add doc strings ([`e35cdb5`](https://github.com/cBournhonesque/lightyear/commit/e35cdb5687c1e9f92ef1055c7ed93d54e12e8ee9))
    - That should do it ([`c2f6d83`](https://github.com/cBournhonesque/lightyear/commit/c2f6d83a2ddf580ae4c9dbfa8a5ee5d906917f49))
    - Update client to support p2p as well ([`1fb1823`](https://github.com/cBournhonesque/lightyear/commit/1fb1823841743344f7ab54c7f8c8dd3e0b65a35a))
    - Remove relay network setting ([`fa8af6b`](https://github.com/cBournhonesque/lightyear/commit/fa8af6b3ce51619924f6af3e033544a974dd24f7))
    - Refactor steamworks client logic ([`a573130`](https://github.com/cBournhonesque/lightyear/commit/a573130c57e943a1d6ab1989192764bd3c1dd39b))
    - Support steam p2p sockets ([`9b279dc`](https://github.com/cBournhonesque/lightyear/commit/9b279dcfb859d4805a44499b17e9059af5187381))
    - Merge pull request #412 from cBournhonesque/cb/nits ([`f1b2cb8`](https://github.com/cBournhonesque/lightyear/commit/f1b2cb86c50fdb0e8d2570612aa594fd661fcc17))
    - Fix clippy ([`d2d20a6`](https://github.com/cBournhonesque/lightyear/commit/d2d20a6a61a294928fa70e7980b7a8b2de34a1a2))
    - Make  pub ([`c1bd3d5`](https://github.com/cBournhonesque/lightyear/commit/c1bd3d572476e03061cefb9d46dba03ba2343600))
    - Merge pull request #411 from cBournhonesque/cb/fix-example ([`8c274b3`](https://github.com/cBournhonesque/lightyear/commit/8c274b318d9adc35a87ac541d8b1fea27ea78e5a))
    - Fix example ([`73db400`](https://github.com/cBournhonesque/lightyear/commit/73db400bd6e7fa74e22ac96601965e1d199c8905))
    - Merge pull request #409 from cBournhonesque/cb/small-fix ([`08fbbff`](https://github.com/cBournhonesque/lightyear/commit/08fbbff1f11a8232bddd8812da0046575dcbc367))
    - Fix ([`391eab5`](https://github.com/cBournhonesque/lightyear/commit/391eab581a1a524b5c590004ead577802de72f31))
    - Merge pull request #406 from cBournhonesque/cb/fix-delta-compression ([`bef7f3d`](https://github.com/cBournhonesque/lightyear/commit/bef7f3d1fab6e8643a20bcd411aa878864446547))
    - Update docs ([`5e739be`](https://github.com/cBournhonesque/lightyear/commit/5e739be55627c1018b38b0cbe14241e9107ab3a0))
    - Fix delta compression on the receive side ([`7d3a42c`](https://github.com/cBournhonesque/lightyear/commit/7d3a42c6702502735a862185e9da864e0d0c5536))
    - Merge pull request #404 from cBournhonesque/cb/add-docs-delta-compression ([`533d7a5`](https://github.com/cBournhonesque/lightyear/commit/533d7a5b5a401ab5cbf78338042e44b416e51715))
    - Taplo ([`1bc4fb1`](https://github.com/cBournhonesque/lightyear/commit/1bc4fb10ce8943a3c79379bfbbb90b4a65e8bea5))
    - Fix tests ([`ff92dd5`](https://github.com/cBournhonesque/lightyear/commit/ff92dd580b602332229a991c8103737384ffb077))
    - Merge pull request #403 from cBournhonesque/cb/use-keepalive2 ([`2ee9ccd`](https://github.com/cBournhonesque/lightyear/commit/2ee9ccda39f80e78ab346ebebc958fe988a5fff3))
    - Merge pull request #401 from cBournhonesque/cb/add-hook-to-accept-connection ([`9d0dfb7`](https://github.com/cBournhonesque/lightyear/commit/9d0dfb7308323b3e5a380787460ea22190c0c70a))
    - Fix ([`4faab86`](https://github.com/cBournhonesque/lightyear/commit/4faab8601d46310d727671f72d020653510809f0))
    - Allow passing a custom deny connection reason, which will be printed on the client side ([`e41a65b`](https://github.com/cBournhonesque/lightyear/commit/e41a65b0ab3c7ff4e77f22a31dd6bfa944b3b88b))
    - Switch to a trait ConnectionRequestHandler ([`71ffdf2`](https://github.com/cBournhonesque/lightyear/commit/71ffdf2a4618e82507951ed4ddaeedd4c87c324a))
    - Switch to bevy_keepalive ([`852d308`](https://github.com/cBournhonesque/lightyear/commit/852d30809b9c3338a2ede44921e5e9d7bd25ef7f))
    - Add unit test for accept connection request ([`51deae3`](https://github.com/cBournhonesque/lightyear/commit/51deae36d47421accffaae38767762218ace2431))
    - Add docs ([`702d40e`](https://github.com/cBournhonesque/lightyear/commit/702d40efed535a09bd79bb1d654e4f1cb1c4388e))
    - Add close in config to let the user accept/reject connections ([`ea1cab4`](https://github.com/cBournhonesque/lightyear/commit/ea1cab4e93338e78774904225f7f6340c6aa35b2))
    - Revert "add close in config to let the user accept/reject connections" ([`e610332`](https://github.com/cBournhonesque/lightyear/commit/e61033298dfaf287ba76c549d5b0fae25e903057))
    - Revert "add docs" ([`ed867ca`](https://github.com/cBournhonesque/lightyear/commit/ed867ca96fcf77377db2a0175e0e2ad23511c520))
    - Add docs ([`5dc855a`](https://github.com/cBournhonesque/lightyear/commit/5dc855a8f8f3aef3e884aab6e992186600c59d3d))
    - Add close in config to let the user accept/reject connections ([`f5bc235`](https://github.com/cBournhonesque/lightyear/commit/f5bc23574078e5c35d990a30f6617c17ec8226a8))
    - Merge pull request #400 from cBournhonesque/cb/improve-config-docs ([`66725ce`](https://github.com/cBournhonesque/lightyear/commit/66725ce4f449c7fb823eb11cfd496209c3f873cf))
    - Improve config docs ([`f9ed488`](https://github.com/cBournhonesque/lightyear/commit/f9ed48898129349715c81fd12107a09c9c6f739b))
    - Merge pull request #399 from cBournhonesque/cb/replication-bug ([`f614a9a`](https://github.com/cBournhonesque/lightyear/commit/f614a9ac50d72e13c66b722f2f9c28cec2f420fa))
    - Fix test ([`36620fa`](https://github.com/cBournhonesque/lightyear/commit/36620fa55e6212d058464fe43140275ba36fb540))
    - Merge pull request #392 from cBournhonesque/cb/sync-controlled ([`e792d7e`](https://github.com/cBournhonesque/lightyear/commit/e792d7ef37d9a6cabdd9a63a254fc447a870b2dc))
    - Add componentsync mode to Controlled ([`d7cfda6`](https://github.com/cBournhonesque/lightyear/commit/d7cfda6c682ad0799bf63f57cb6796c4066dd037))
    - Merge pull request #384 from cBournhonesque/cb/delta-compression ([`6cc3036`](https://github.com/cBournhonesque/lightyear/commit/6cc303633ec025cc076bf5728ceb17db049206a3))
    - Fix bandwidth diagnostic ([`60429d8`](https://github.com/cBournhonesque/lightyear/commit/60429d8d38058605268a21010882099fcbffb3b5))
    - Fix replication issue with send_tick ([`1c1696b`](https://github.com/cBournhonesque/lightyear/commit/1c1696b83161aa9f5df0b4ec6414ea474290e4b6))
    - Add tick clamping ([`58f97eb`](https://github.com/cBournhonesque/lightyear/commit/58f97ebadca791094bfefed49beb208324ab0882))
    - Fix tests ([`17b909c`](https://github.com/cBournhonesque/lightyear/commit/17b909cfbc6e27268116882ee5179caf10aeb87e))
    - Add DeltaManager to keep unified storage shared across all connections ([`dc68d02`](https://github.com/cBournhonesque/lightyear/commit/dc68d02e8e753ba113972d7b66b729942db926d4))
    - Add tests apply_diff ([`e6d8329`](https://github.com/cBournhonesque/lightyear/commit/e6d83297737fa1a5e3704c1a0d15f11ea004f126))
    - Fix type-erased functions, add tests, run with miri ([`b4d77d9`](https://github.com/cBournhonesque/lightyear/commit/b4d77d92eede3dc15feb50be6a1da8de7b479f00))
    - Add drop functions for delta message, add DeltaType to distinguish between a normal diff and a diff from base ([`d07cb06`](https://github.com/cBournhonesque/lightyear/commit/d07cb06546332586e79107f51af0b0fcca9c5971))
    - Tests pass ([`b8dff23`](https://github.com/cBournhonesque/lightyear/commit/b8dff230a83bf8617764967a5b54ce192328ea47))
    - Update type-erasure. Make prepare_component_update type-erased. Add Diffable trait. Update ReplicationSend to handle delta-compression ([`f0f82a9`](https://github.com/cBournhonesque/lightyear/commit/f0f82a9505acba23ac86b9c909814197ec29151d))
    - Merge pull request #380 from cBournhonesque/cb/add-more-diagnostics-plugins ([`303a2f0`](https://github.com/cBournhonesque/lightyear/commit/303a2f00201a0e55bdd84fb940e955ee47387187))
    - Add PingDiagnosticPlugin ([`1823243`](https://github.com/cBournhonesque/lightyear/commit/1823243435dda57070e4c5bcfb27ba48a080ee08))
    - Merge pull request #379 from cBournhonesque/cb/add-rollback-diagnostics ([`e758b14`](https://github.com/cBournhonesque/lightyear/commit/e758b147a5c1555d37345a8c7860eba8e948e5e8))
    - Put rollback diagnostics into PredictionDiagnosticsPlugin ([`f1222f9`](https://github.com/cBournhonesque/lightyear/commit/f1222f9a2fc7ac842e2c64aeb65e931fd1466495))
    - Merge pull request #378 from RJ/diagnostics ([`8dd55dd`](https://github.com/cBournhonesque/lightyear/commit/8dd55dd643e1fea1cb594d85f7311738ef253e57))
    - Collect rollback metrics, write to Diagnostics ([`459459e`](https://github.com/cBournhonesque/lightyear/commit/459459e0eb2234c6aad6477fef537656578de89e))
    - Merge pull request #376 from cBournhonesque/cb/rebroadcast-inputs ([`90be0dd`](https://github.com/cBournhonesque/lightyear/commit/90be0dd80e0f4c88bb0e4c06cdd771297bc2c9da))
    - Fix rustdoc ([`5755ca0`](https://github.com/cBournhonesque/lightyear/commit/5755ca0cbb019ece140097238bf8a7ff8e053fb6))
    - Lints ([`0ecdbd7`](https://github.com/cBournhonesque/lightyear/commit/0ecdbd71d086c59405e7e4b32860fa21e0c025cf))
    - Refactor inputs ([`e37f9e1`](https://github.com/cBournhonesque/lightyear/commit/e37f9e1ce5d3c1bd1dc74fd418714cd7dad3edc1))
    - Merge pull request #374 from cBournhonesque/cb/debug-rollback ([`b99cfdd`](https://github.com/cBournhonesque/lightyear/commit/b99cfdd7bb3ecb25fdacbab8da0591e12a53121f))
    - Fix action-state for other players ([`7dcc36d`](https://github.com/cBournhonesque/lightyear/commit/7dcc36d9543aac80b1663ff40423e8ae7da2e977))
    - Merge pull request #372 from cBournhonesque/cb/add-simple-example ([`fd6099f`](https://github.com/cBournhonesque/lightyear/commit/fd6099f4d96e6e6ecccfa1130edd281380fe069f))
    - Add simple_setup example ([`4f048f0`](https://github.com/cBournhonesque/lightyear/commit/4f048f0f82b56993ce82e9f65b9dd2fb22a55640))
    - Merge pull request #371 from cBournhonesque/cb/wasm-web-worker-updates ([`b43a63f`](https://github.com/cBournhonesque/lightyear/commit/b43a63f88130c45663845f260206c309cf8fa166))
    - Add WebPlugin to keep running in background ([`f783018`](https://github.com/cBournhonesque/lightyear/commit/f7830182d4f3b5e3fa623b35940b12abee4930ea))
    - Merge pull request #368 from cBournhonesque/cb/add-run-conditions ([`0f6a25e`](https://github.com/cBournhonesque/lightyear/commit/0f6a25eb9e98ef78bf79f0531a5898d5ecf08140))
    - Fix ([`e715e28`](https://github.com/cBournhonesque/lightyear/commit/e715e28921c0473c616dd47548991da4f459a7db))
    - Add run conditions ([`20984d9`](https://github.com/cBournhonesque/lightyear/commit/20984d9d0c604cde7c74b5f37c1eb910dce8760a))
    - Merge pull request #366 from cBournhonesque/cb/subscribe-acks ([`c9eb376`](https://github.com/cBournhonesque/lightyear/commit/c9eb376e5b3a104a204da1e49f98927787ca4285))
    - Add subscribe_ack to reliable sender, add subscribe_nack to all channels ([`ef1b200`](https://github.com/cBournhonesque/lightyear/commit/ef1b2008506dc7da78f1036a7e463f4d3226b058))
    - Merge pull request #365 from cBournhonesque/cb/eventual-consistency ([`842b693`](https://github.com/cBournhonesque/lightyear/commit/842b693f43a5edc94e49321205a1c78fd1460f7a))
    - Lint ([`04c4974`](https://github.com/cBournhonesque/lightyear/commit/04c497412e5fb01a495bb1638e5e7239274f97da))
    - Make the new behaviour configurable ([`546f90b`](https://github.com/cBournhonesque/lightyear/commit/546f90bdbc5dc74b35ccad60a9548e0b4aed1710))
    - Add unit test around send_ticks/ack_ticks ([`3cc8129`](https://github.com/cBournhonesque/lightyear/commit/3cc81298f6a8e591a70da9e0ae48c1d2ebe7cfd7))
    - Vary ReplicationSend behaviour depending on if bandwidth cap is enabled or not ([`771a811`](https://github.com/cBournhonesque/lightyear/commit/771a8110f1247ae417f87e24155af08f9c4cc4f7))
    - Send updates since we last sent an update, not since we last acked ([`868116c`](https://github.com/cBournhonesque/lightyear/commit/868116c743d14398d935392e8d1fd2051aca9a6a))
    - Merge pull request #363 from cBournhonesque/cb/despawn-recursive ([`71308af`](https://github.com/cBournhonesque/lightyear/commit/71308affc88b25d53e4bb1d4d4514a3306890395))
    - Despawn recursive predicted/interpolated entities ([`c9c468b`](https://github.com/cBournhonesque/lightyear/commit/c9c468b248e022ba4c847e410d6adb6bbab2c054))
    - Fix log spam when server or client disconnects with websockets' ([`bce8fd1`](https://github.com/cBournhonesque/lightyear/commit/bce8fd16a3d55f3ac371f92b1bfc33fff6b463a6))
    - Merge pull request #362 from cBournhonesque/cb/fix-examples ([`2e53e0b`](https://github.com/cBournhonesque/lightyear/commit/2e53e0b57f7eaca8e6415f0bbd9a672f430534bc))
    - Fix examples ([`17cac34`](https://github.com/cBournhonesque/lightyear/commit/17cac34d810d1f0b4521921a5d770d1d82b06624))
    - Merge pull request #361 from cBournhonesque/cb/publish-lightyear-common ([`3fd624e`](https://github.com/cBournhonesque/lightyear/commit/3fd624e7eb1ef75ecc0f1a4e78db0cf2ea6a5b37))
    - Improve examples common ([`2a454aa`](https://github.com/cBournhonesque/lightyear/commit/2a454aa8109987af937a53cc064323b5633db3ff))
    - Fix ([`3462022`](https://github.com/cBournhonesque/lightyear/commit/3462022311f7e973a6ce0edc52291e6fd08fdd8c))
    - 0.15 ([`5bde955`](https://github.com/cBournhonesque/lightyear/commit/5bde955c613f8044af1c14bfd563754eeb4361a8))
    - Merge pull request #358 from cBournhonesque/cb/fix-bullet-prespawn ([`b102546`](https://github.com/cBournhonesque/lightyear/commit/b10254691d1f0addc1a1c9c687b06e4957c52493))
    - Merge pull request #357 from msvbg/host-is-server ([`d3179ae`](https://github.com/cBournhonesque/lightyear/commit/d3179aed6cee247d5b1a7af07d98485d6f5eac15))
    - Fix ([`9ee6572`](https://github.com/cBournhonesque/lightyear/commit/9ee6572af2914cb5b08b0ce8a52accaf2e5b7108))
    - Merge pull request #353 from cBournhonesque/cb/split-replicate ([`0d2c59a`](https://github.com/cBournhonesque/lightyear/commit/0d2c59a59b13eba48e30959307ffbdf794893c6f))
    - Fix docs ([`a4a1002`](https://github.com/cBournhonesque/lightyear/commit/a4a1002a361fae3f0b8b6b2e853f74217bcd70a9))
    - Merge pull request #354 from msvbg/better-rollback-debug-logs ([`46580c1`](https://github.com/cBournhonesque/lightyear/commit/46580c1fc69ea5a62ebcd04687ac3c280c582a4c))
    - Fix all examples ([`e325de4`](https://github.com/cBournhonesque/lightyear/commit/e325de41b4b22fea100d5c7f7814651cbbfb4f96))
    - Let host-server have server network identity ([`279ad6a`](https://github.com/cBournhonesque/lightyear/commit/279ad6a7fb17d422a79d8496303893a6b33e4797))
    - Better rollback debug logs ([`e884d0b`](https://github.com/cBournhonesque/lightyear/commit/e884d0ba87cdae1ead05a73d0b6ca0672a3545f5))
    - Rename ReplicateClientToServer to Replicate ([`7cf7ab4`](https://github.com/cBournhonesque/lightyear/commit/7cf7ab402f5be3dbc349c5dc0cc5352804d30aa1))
    - All tests pass ([`5b80d38`](https://github.com/cBournhonesque/lightyear/commit/5b80d38ca3d073c067e4df1b28244919dbc7218a))
    - Fix ([`1857232`](https://github.com/cBournhonesque/lightyear/commit/1857232ecd4245142ce8baa8ca0523e4041950fa))
    - Compiles ([`9176567`](https://github.com/cBournhonesque/lightyear/commit/91765675794f0695dc65af9f149223b53cfd6f11))
    - Split client and server replicate ([`bbe5b9c`](https://github.com/cBournhonesque/lightyear/commit/bbe5b9c7525318492a6b0fb4e2f53f070e29f14e))
    - Merge pull request #352 from cBournhonesque/cb/fix-web ([`1475df6`](https://github.com/cBournhonesque/lightyear/commit/1475df6d87facc738306e849fac8d2846e5a77f8))
    - More fixes ([`359de51`](https://github.com/cBournhonesque/lightyear/commit/359de5118cc14422c02f23240921148b7cd664a6))
    - Fix wasm ([`e20e1c1`](https://github.com/cBournhonesque/lightyear/commit/e20e1c16a4821830c7045ec6f5f5a1e57184ade9))
    - Merge pull request #351 from cBournhonesque/cb/fix-doc ([`7da67eb`](https://github.com/cBournhonesque/lightyear/commit/7da67eb1ddfc223d58b439f9f93b3923b6384c14))
    - Fix ([`4de50d6`](https://github.com/cBournhonesque/lightyear/commit/4de50d6d9f6db6142b912d4329ea2dd3f19f1b6e))
    - Improve docs ([`0c867fd`](https://github.com/cBournhonesque/lightyear/commit/0c867fd664111a838c0f6649b8bfc75c50ca3068))
    - Merge pull request #348 from cBournhonesque/cb/check-partial-eq-before-update ([`d8942e5`](https://github.com/cBournhonesque/lightyear/commit/d8942e5a2a272afded1ee751fd7b170cdb70d056))
    - Clippy ([`717087e`](https://github.com/cBournhonesque/lightyear/commit/717087ed9bc08cebff63932f6cc32b4c368bbcc0))
    - Add client->server replication test ([`f929584`](https://github.com/cBournhonesque/lightyear/commit/f92958486a1983921d46bbb8eb631c41735aa30f))
    - Merge pull request #347 from cBournhonesque/cb/refactor-replicate ([`830fb6c`](https://github.com/cBournhonesque/lightyear/commit/830fb6c385d6ddc153330cc879745d473961d825))
    - Fix test + clippy ([`b1eb6b6`](https://github.com/cBournhonesque/lightyear/commit/b1eb6b6401284a9a02203342dee1aca53a91380b))
    - Refactor all examples ([`3dc8ce2`](https://github.com/cBournhonesque/lightyear/commit/3dc8ce2509ee218c8995f9ba8cd153c268379dd1))
    - Add per component replication components ([`6bd0a4f`](https://github.com/cBournhonesque/lightyear/commit/6bd0a4ff5cb0209133af2e4c222fe89759e36807))
    - Try handling replication update ([`9f160fb`](https://github.com/cBournhonesque/lightyear/commit/9f160fb0e69ace562deea2d6fa57763cd0115b10))
    - Add visibility cache. TODO: update replication cache for target ([`d88b8e3`](https://github.com/cBournhonesque/lightyear/commit/d88b8e39a270f9d67c435d3e99b076103827e605))
    - Add more replication tests ([`8dfbb82`](https://github.com/cBournhonesque/lightyear/commit/8dfbb821230f1b4733121478d7df343be6061f60))
    - All tests pass ([`e8dad57`](https://github.com/cBournhonesque/lightyear/commit/e8dad57c3b214c7a4124a1f22df4500e87c8e917))
    - Replication compiles again ([`f7db7e5`](https://github.com/cBournhonesque/lightyear/commit/f7db7e587f3fef7448f4ece15fc2d20b91c37eb7))
    - Update replication component_update, entity_despawn, component_remove ([`5ab71ba`](https://github.com/cBournhonesque/lightyear/commit/5ab71ba930fbb4e5d0fd9febf635554f6dcc515f))
    - Move prepare-entity-spawn out of shared replication systems ([`876e465`](https://github.com/cBournhonesque/lightyear/commit/876e4658263d42d60dc1430b1ba5bea85a93f25c))
    - Splitting replicate into multiple parts ([`115a48c`](https://github.com/cBournhonesque/lightyear/commit/115a48c3d6a7b07bebd0541ec9f4ddf0c42c12ff))
    - Merge pull request #344 from cBournhonesque/cb/rename-rollback-check ([`6f02f61`](https://github.com/cBournhonesque/lightyear/commit/6f02f6179180a16cb70148b424569241eca6f1e3))
    - Merge pull request #345 from cBournhonesque/cb/message-to-room ([`b0d2331`](https://github.com/cBournhonesque/lightyear/commit/b0d2331e2ea455a94d539bf1a44f54e6520b47ed))
    - Make room entities/clients public, and add send_message_to_room ([`01aba5d`](https://github.com/cBournhonesque/lightyear/commit/01aba5d039ab6603927536b290e9bfd89827a952))
    - Rename the rollback check fn ([`b7c5223`](https://github.com/cBournhonesque/lightyear/commit/b7c52235495e6179b77ccbc90911c09d3bef85ae))
    - Merge pull request #343 from cBournhonesque/cb/add-controlled-component ([`650bbb0`](https://github.com/cBournhonesque/lightyear/commit/650bbb02214503f116bece71627f66d81eefcbaa))
    - Taplo ([`8c3ddd3`](https://github.com/cBournhonesque/lightyear/commit/8c3ddd33a36ebaf6451d3d8da21fee87cb15b6a8))
    - Fix examples ([`579334f`](https://github.com/cBournhonesque/lightyear/commit/579334f213196f8956693b8c464a33eeab8f2266))
    - Merge branch 'main' into cb/add-controlled-component ([`d27b972`](https://github.com/cBournhonesque/lightyear/commit/d27b97204780b62109ef4bac4069d74d2e0f21c1))
    - Merge pull request #342 from cBournhonesque/cb/debug-disconnect ([`b2d26ad`](https://github.com/cBournhonesque/lightyear/commit/b2d26adba4c0bc43c984548323c0530b9e781edb))
    - Lint ([`35398d8`](https://github.com/cBournhonesque/lightyear/commit/35398d8305ae72363bddf93a2373f45edd308de3))
    - Io task errors in websocket/webtransport now cause a netcode disconnect ([`4d09ea6`](https://github.com/cBournhonesque/lightyear/commit/4d09ea68d293d4166035ed322c69861568f449e6))
    - Client disconnects stop the corresponding io task ([`65b5bc4`](https://github.com/cBournhonesque/lightyear/commit/65b5bc4f0487bf47dfd1459b43c53a1b80b52c9b))
    - Update server to handle disconnect events from netcode ([`5939b21`](https://github.com/cBournhonesque/lightyear/commit/5939b21b5129c8a2c2555f1dd2ecb8037549ec53))
    - Working examples ([`d5c6af5`](https://github.com/cBournhonesque/lightyear/commit/d5c6af58aeef8b216c70472e6ad2ee7f0a522be6))
    - Wip overhaul netcode/io ([`896a545`](https://github.com/cBournhonesque/lightyear/commit/896a545e126207ea7bd859faf9616327ed3736c4))
    - Wip add Controlled component ([`ab1fc5a`](https://github.com/cBournhonesque/lightyear/commit/ab1fc5a4222f467d46456f087fbe97240950513d))
    - Merge pull request #340 from cBournhonesque/cb/webtransport-logs ([`18a4456`](https://github.com/cBournhonesque/lightyear/commit/18a4456fb1638d6c35c1b2237dcc02f0d84b7695))
    - Clean ([`8bb4370`](https://github.com/cBournhonesque/lightyear/commit/8bb4370f88afd7d9b66eeea0715e9185981d2f08))
    - Improve io ([`1e8521a`](https://github.com/cBournhonesque/lightyear/commit/1e8521ab20c08bbf7bdcfc690a1f9174ea18773b))
    - Update io ([`fb6f56e`](https://github.com/cBournhonesque/lightyear/commit/fb6f56eef338ff2311d4f4ab741e4b6152eb8c49))
    - Update readme ([`5239752`](https://github.com/cBournhonesque/lightyear/commit/5239752b40f4da9ad56346c434d6a144a0403977))
    - Merge pull request #338 from cBournhonesque/cb/override-rollback-check ([`6ede0cb`](https://github.com/cBournhonesque/lightyear/commit/6ede0cb5c7c8ab5dda900545027aef2cb226c3b8))
    - Update rollback to use the registered rollback check function ([`3143653`](https://github.com/cBournhonesque/lightyear/commit/314365384a60e0a13dc096cf752dea6b2040a6d0))
    - Add docs ([`4cfd353`](https://github.com/cBournhonesque/lightyear/commit/4cfd3539da8c4a94919d36cff35a48cbf8fe0165))
    - Merge pull request #337 from cBournhonesque/cb/visual-interpolation-rollback ([`5c334ae`](https://github.com/cBournhonesque/lightyear/commit/5c334ae0ca55e6098f711069771e6d950655f449))
    - Restore visual interpolation's actual component value before rollback check ([`ed03180`](https://github.com/cBournhonesque/lightyear/commit/ed031802dfd7cc6cbb58eca2cde1b42ab8e83809))
    - Merge pull request #336 from cBournhonesque/cb/use-debug-assert ([`2846776`](https://github.com/cBournhonesque/lightyear/commit/28467768938f109bfe449454dd7ab53c1f2f0667))
    - Use debug assert ([`91b5101`](https://github.com/cBournhonesque/lightyear/commit/91b5101fdc70311f6a5cc7e5946a4ff01e504790))
    - Merge pull request #335 from cBournhonesque/cb/fix-visibility ([`33076e0`](https://github.com/cBournhonesque/lightyear/commit/33076e0ce35eebcebab01282693246bb16381d0c))
    - Clippy ([`3da81ce`](https://github.com/cBournhonesque/lightyear/commit/3da81ceb31c82dd320b14c613941cab5795382dd))
    - Fix logs ([`a29165b`](https://github.com/cBournhonesque/lightyear/commit/a29165bbf3b9f23c6403789fbf1e68cb62349355))
    - Merge pull request #332 from msvbg/visual-interpolate-marker ([`8113e20`](https://github.com/cBournhonesque/lightyear/commit/8113e202b4d1cd86e5bc58d739ce198873d2a629))
    - Remove VisualInterpolateMarker ([`a4729b7`](https://github.com/cBournhonesque/lightyear/commit/a4729b7def1b8a334ee8fc99d34e2aa7df068e5a))
    - Small warning fixes ([`7830fbc`](https://github.com/cBournhonesque/lightyear/commit/7830fbc95c1d353fd2d476f575247af1b1f322e7))
    - Merge pull request #331 from cBournhonesque/cb/update-per-component-metadata ([`f2a1fb7`](https://github.com/cBournhonesque/lightyear/commit/f2a1fb7afa572c1c0585348c3e49cb8e6b684145))
    - Update component metadata target override to not user intersection ([`e32affb`](https://github.com/cBournhonesque/lightyear/commit/e32affbb696555498ef0e78fd5c47442e389743f))
    - Merge pull request #330 from cBournhonesque/cb/update-server-config-default ([`aeacfb3`](https://github.com/cBournhonesque/lightyear/commit/aeacfb36ebe91712ed698296568eb87b045281db))
    - Make private key compulsory on server config ([`dee06b6`](https://github.com/cBournhonesque/lightyear/commit/dee06b61cf25e68833c3297b56db56fd3b830264))
    - Merge pull request #327 from cBournhonesque/cb/test-harness ([`4c6a501`](https://github.com/cBournhonesque/lightyear/commit/4c6a5013cf0f49aae0b74d4d9eeb4decaf4a1b8f))
    - Lobby more-or-less works, still seems to be a room problem ([`d95bf9d`](https://github.com/cBournhonesque/lightyear/commit/d95bf9ded42a5bfcb4c0258ba79e33b94596efc0))
    - Wip ([`7c061a4`](https://github.com/cBournhonesque/lightyear/commit/7c061a4973b6e98d4cca4b04f25129e902007294))
    - Merge pull request #326 from cBournhonesque/cb/plugin-groups ([`3c198c1`](https://github.com/cBournhonesque/lightyear/commit/3c198c1312b45fea469dc34fdd61d32575c22a6c))
    - Fix rust doc ([`a5a3f7b`](https://github.com/cBournhonesque/lightyear/commit/a5a3f7b14bf905af353eb56d0d422ada1785ed7c))
    - Remove disable option on prediction plugin ([`651c62d`](https://github.com/cBournhonesque/lightyear/commit/651c62dea81ceda737946b98ffcabcda175f9aaa))
    - Fix ([`77f61ad`](https://github.com/cBournhonesque/lightyear/commit/77f61ad4a3cf507d95bbb58ea1802e7936237605))
    - Make clientplugins and serverplugins plugin groups ([`b4b7753`](https://github.com/cBournhonesque/lightyear/commit/b4b77533886c403a51d4587402162698939c20c0))
    - Merge pull request #325 from cBournhonesque/cb/update-visibility ([`5a1dcea`](https://github.com/cBournhonesque/lightyear/commit/5a1dcea36fae0e716e9d6f6e17d65b9d442b8a79))
    - Remove logs ([`822e487`](https://github.com/cBournhonesque/lightyear/commit/822e4874f6ce7894790a1d9943a16b35f5036363))
    - Wip ([`153132d`](https://github.com/cBournhonesque/lightyear/commit/153132d669db0d12ac5b0a361228e59c121db7e1))
    - Merge pull request #323 from msvbg/sparse-set-corrections ([`d52a070`](https://github.com/cBournhonesque/lightyear/commit/d52a070f8cd8c099be911465a8027b04d3ae9243))
    - Use sparse set storage for corrections ([`3da346b`](https://github.com/cBournhonesque/lightyear/commit/3da346b0933e87bd0410af688ad58e7d4f679b65))
    - Merge pull request #320 from cBournhonesque/cb/improve-docs ([`23a1ed8`](https://github.com/cBournhonesque/lightyear/commit/23a1ed82371c370ce82db8c9ca9a8b99115fdae4))
    - Improve protocol ergonomics ([`f0caa4f`](https://github.com/cBournhonesque/lightyear/commit/f0caa4f3d4850e9935767ca00237b79994815f31))
    - Merge pull request #319 from cBournhonesque/cb/lint ([`cf19073`](https://github.com/cBournhonesque/lightyear/commit/cf19073be306c1e8fdfe3c88adc234ebc8ff4264))
    - Taplot fmt ([`5b4f6f6`](https://github.com/cBournhonesque/lightyear/commit/5b4f6f6a32598091bc9b5972f1b1f1bef96cedb0))
    - Remove render features, fix rustdocs ([`bc7982c`](https://github.com/cBournhonesque/lightyear/commit/bc7982c602537cafd770c0c02f52b9bd8cf1df22))
    - More taplo ([`049f5ac`](https://github.com/cBournhonesque/lightyear/commit/049f5acca83f5e335b61af9004ed279ee51627e3))
    - Clippy + lints ([`8a5f3b9`](https://github.com/cBournhonesque/lightyear/commit/8a5f3b9f670de609bfde0671a473b656955cb989))
    - Merge pull request #318 from cBournhonesque/cb/transfer-replication ([`c35b3de`](https://github.com/cBournhonesque/lightyear/commit/c35b3de3494279c3024a6424d4d2dcd74c7a9ebe))
    - Enable re-using target entity when starting replication ([`53e0a7b`](https://github.com/cBournhonesque/lightyear/commit/53e0a7b7c0ba23509ee47c21535ad2af9cc672a6))
    - Merge branch 'main' into cb/transfer-replication ([`ba7c654`](https://github.com/cBournhonesque/lightyear/commit/ba7c654c7979fd9ca3cd2c6fa6f9b0e0db07f8a3))
    - Merge pull request #317 from cBournhonesque/cb/update-wtransport ([`9e261ef`](https://github.com/cBournhonesque/lightyear/commit/9e261efe82df0b9a7f92e61febb02ccb28c0febf))
    - Update all examples to work with wtransport 0.11.3 ([`2bf233b`](https://github.com/cBournhonesque/lightyear/commit/2bf233b956ec20fc186442d195da500087c324f7))
    - Wip ([`c523da3`](https://github.com/cBournhonesque/lightyear/commit/c523da3f23a3550406435d66537a6f1701054fc1))
    - Merge pull request #314 from cBournhonesque/dependabot/cargo/base64-0.22.1 ([`8982e3f`](https://github.com/cBournhonesque/lightyear/commit/8982e3f58aedd2d75e3be32943fbfaa39af25692))
    - Merge pull request #313 from cBournhonesque/dependabot/cargo/wtransport-eq-0.1.13 ([`4ecf31c`](https://github.com/cBournhonesque/lightyear/commit/4ecf31c1ba371707e7c79d0fd2ad3c497802c9c1))
    - Update base64 requirement from 0.21.5 to 0.22.1 ([`db23c59`](https://github.com/cBournhonesque/lightyear/commit/db23c59536fc537561eb89bd6fac025a7754335b))
    - Update wtransport requirement from =0.1.11 to =0.1.13 ([`9074541`](https://github.com/cBournhonesque/lightyear/commit/90745415fb7822af9d4fe69db0326572ccc2b5a1))
    - Merge pull request #312 from cBournhonesque/cb/provide-auth-thread ([`9834e12`](https://github.com/cBournhonesque/lightyear/commit/9834e128db27c37d40991c8447f37d8b7a99239a))
    - Merge pull request #311 from msvbg/xpbd-f64 ([`1bde835`](https://github.com/cBournhonesque/lightyear/commit/1bde8356513774e8c76154099f6f72977402e72a))
    - Addded client tcp stream ([`9f0c23f`](https://github.com/cBournhonesque/lightyear/commit/9f0c23f9ca2072fdab40598c67ed2aee582313fa))
    - Support bevy_xpbd with double precision ([`0153b15`](https://github.com/cBournhonesque/lightyear/commit/0153b153245c5869005689c741d0b9da8d3b88fa))
    - Fix doc ([`7ab78f8`](https://github.com/cBournhonesque/lightyear/commit/7ab78f82b1930dbe592bbc7000ec922e75587881))
    - Merge pull request #310 from cBournhonesque/cb/clarify-connect-token ([`2f087fc`](https://github.com/cBournhonesque/lightyear/commit/2f087fc5704f8cdd3e6467945e575392d78ed625))
    - Clarify connect token docs ([`0f6f892`](https://github.com/cBournhonesque/lightyear/commit/0f6f8926a919673669599b66e378f0562f30da78))
    - Try to get zstd working in wasm ([`38543cd`](https://github.com/cBournhonesque/lightyear/commit/38543cdc20e8687c1ad62c7f2f370d1d0f90d3bb))
    - Try to compile zstd for wasm ([`15deeb3`](https://github.com/cBournhonesque/lightyear/commit/15deeb3cc81792d5be6ecfbc58be9e0fa546e8eb))
    - Merge pull request #306 from cBournhonesque/cb/book-preprediction ([`fa8fd83`](https://github.com/cBournhonesque/lightyear/commit/fa8fd83e7d114e92fcaa6e68f7f012127a86b554))
    - Fix ([`ba8e493`](https://github.com/cBournhonesque/lightyear/commit/ba8e49317a79cf9d91456a6ebaf4ee491535209a))
    - Merge pull request #305 from cBournhonesque/cb/remove-clone-bound ([`abd5fd3`](https://github.com/cBournhonesque/lightyear/commit/abd5fd349b10dfd2f3d6328b96090b9af54c106e))
    - Remove clone bound on bitserializable ([`b4ff7a5`](https://github.com/cBournhonesque/lightyear/commit/b4ff7a566f5336410a91f3b42dd35d3e746d1b66))
    - Merge pull request #303 from cBournhonesque/cb/bidirectional-resource-replicate ([`dce3362`](https://github.com/cBournhonesque/lightyear/commit/dce33620a707827c674d7afa38aad920ab1b9148))
    - Lints ([`b3b977a`](https://github.com/cBournhonesque/lightyear/commit/b3b977a4f0c217d93879930fe9ec0e5a7726aa36))
    - Fix examples ([`9e770cc`](https://github.com/cBournhonesque/lightyear/commit/9e770cc132c13de0ee8400b72a3119ece9a265d2))
    - Working bidirectional resource replication, but change detection is disabled ([`070416e`](https://github.com/cBournhonesque/lightyear/commit/070416e27178feb882b0246a353f6020241e2d5b))
    - Tests pass, but there must be a system ordering issue? ([`493b77b`](https://github.com/cBournhonesque/lightyear/commit/493b77bb2275956a810544b970c5af903885731f))
    - Wip replace resource replication via messages ([`0ca8f4e`](https://github.com/cBournhonesque/lightyear/commit/0ca8f4e2e0b6b1440923295623560b29655ddb83))
    - Wip ([`a381c04`](https://github.com/cBournhonesque/lightyear/commit/a381c04f8f52f76ecb4e1f8d05d78a4595f153f1))
    - Merge pull request #299 from cBournhonesque/cb/investigate-packet-warn ([`1a12aa6`](https://github.com/cBournhonesque/lightyear/commit/1a12aa63e1679e024307163b0f188c463c6893a5))
    - Add compression ([`80bc873`](https://github.com/cBournhonesque/lightyear/commit/80bc873c238c5870d512027cd1248164c252bab3))
    - Merge pull request #298 from cBournhonesque/cb/improve-interpolation-parallelism ([`6c67599`](https://github.com/cBournhonesque/lightyear/commit/6c67599462eb7a59c09767087b4c20437e8fc4ff))
    - Make interpolation systems run in parallel ([`507d05e`](https://github.com/cBournhonesque/lightyear/commit/507d05ec765ae2756c3da629103f68b17489945a))
    - Merge pull request #297 from cBournhonesque/cb/improve-prediction-parallel ([`992ef80`](https://github.com/cBournhonesque/lightyear/commit/992ef80da00c1689667a896ea9a7621176518bdb))
    - Add prediction unit tests and simplify prediction spawn ([`13a21bc`](https://github.com/cBournhonesque/lightyear/commit/13a21bcf1a4725470055ebb8deaae5ec66c29b48))
    - Merge pull request #295 from cBournhonesque/cb/rollback-parallelis ([`c979f3c`](https://github.com/cBournhonesque/lightyear/commit/c979f3c810f015f9a66b4c036cc2a8c516db8857))
    - Fix ([`9b5dee1`](https://github.com/cBournhonesque/lightyear/commit/9b5dee161b31c946cb45872e3314f47fc9f9778d))
    - Improve rollback parallelism by adding a RwLock around the rollback state ([`0af28f1`](https://github.com/cBournhonesque/lightyear/commit/0af28f187b4c845f36cbe02daafd2e2815a2b24c))
    - Merge pull request #294 from cBournhonesque/cb/update-room-id ([`703592e`](https://github.com/cBournhonesque/lightyear/commit/703592ea5b1ce758a89f1db72635ae76e7dfc3e7))
    - Update RoomId to be u64 ([`dd478ee`](https://github.com/cBournhonesque/lightyear/commit/dd478ee29c39e952637b7e567529285b3d90535f))
    - Merge pull request #293 from cBournhonesque/cb/add-pred-tests ([`fc14b9d`](https://github.com/cBournhonesque/lightyear/commit/fc14b9d038421e603e50b7b8cb6a9ed620517a62))
    - Add tests for updating prediction history ([`c5d57e1`](https://github.com/cBournhonesque/lightyear/commit/c5d57e10350406ccb5a4958bcc8ad1722eff9844))
    - Merge pull request #292 from cBournhonesque/cb/add-pred-tests ([`8f28894`](https://github.com/cBournhonesque/lightyear/commit/8f28894d662b86b1bba8bd5f7e1225382688d3a7))
    - Add_prediction_history tests ([`5e8999d`](https://github.com/cBournhonesque/lightyear/commit/5e8999d2efd8543143d6bfc0e4fea78200a712b7))
    - Clean prediction imports and add prediction-history test ([`5efd11d`](https://github.com/cBournhonesque/lightyear/commit/5efd11d19c707360b1c74eace611e8b958bb95fc))
    - Merge pull request #291 from cBournhonesque/cb/replication-bug ([`c5dc985`](https://github.com/cBournhonesque/lightyear/commit/c5dc985734c8a82ed62a24b3369263176c6a2b32))
    - Fmt ([`a7994e3`](https://github.com/cBournhonesque/lightyear/commit/a7994e3621a6044a3710076424f7077b772022f9))
    - Release 0.14.1 ([`3f988b5`](https://github.com/cBournhonesque/lightyear/commit/3f988b5ff66ea8df5a38b851c00b80dcb6868030))
    - Run prediction after EmitEvents ([`f5255d1`](https://github.com/cBournhonesque/lightyear/commit/f5255d19738c401adeb83657ea501223c61a1891))
    - Fix ([`5041fa3`](https://github.com/cBournhonesque/lightyear/commit/5041fa3351b1dfe24081fa3230b7015b496cea31))
    - Merge pull request #289 from cBournhonesque/cb/remove-internal ([`d538adc`](https://github.com/cBournhonesque/lightyear/commit/d538adc45035fa1684190dc42027bb5a33beb3c6))
    - Comment ([`b16368d`](https://github.com/cBournhonesque/lightyear/commit/b16368d4c60fbcef53e3a8df351fb7d5fe8d15b8))
    - Remove internal and fix benchmarks ([`9fc02e3`](https://github.com/cBournhonesque/lightyear/commit/9fc02e3908b0c0c93d68884ef5a235d95a4d3f16))
    - Merge pull request #288 from cBournhonesque/cb/remove-copy ([`16b1c72`](https://github.com/cBournhonesque/lightyear/commit/16b1c723f19d97a6e410482f6d49c36e43722cb2))
    - Fix ([`a39a02c`](https://github.com/cBournhonesque/lightyear/commit/a39a02c20db5bf697f680621e7ddced98265b871))
    - Rename buffers to BitcodeReader and BitcodeWriter ([`fed5af3`](https://github.com/cBournhonesque/lightyear/commit/fed5af32a078afe05e81a762b60bed5d89a077be))
    - Fix ([`011c619`](https://github.com/cBournhonesque/lightyear/commit/011c619e98e3fbca48cc595c82b7d72f17b65c91))
    - 0.14 ([`3990776`](https://github.com/cBournhonesque/lightyear/commit/3990776210dbba7224984aabd2faf2f868b8b85e))
    - Fix ([`21565f6`](https://github.com/cBournhonesque/lightyear/commit/21565f67279d2a77edbcf75317d3d4c64c63fe18))
    - Merge pull request #285 from cBournhonesque/cb/fix-prespawn-disconnect-bug ([`1ebe333`](https://github.com/cBournhonesque/lightyear/commit/1ebe3335e1fcb2c4680578b514e6f65be9760cc3))
    - Rustfmt ([`40e39db`](https://github.com/cBournhonesque/lightyear/commit/40e39db7ce18e83763f6e295804ee9d8b09e7e90))
    - Set synced to false upon disconnect ([`b8da6a7`](https://github.com/cBournhonesque/lightyear/commit/b8da6a77481e8ebcc1fcd92fbe420730ae7f6804))
    - Fix saturating diff bug ([`5550e49`](https://github.com/cBournhonesque/lightyear/commit/5550e490b5d768fac507fd5d1ddcc1602d3f3929))
    - Merge pull request #284 from cBournhonesque/cb/clean-imports ([`4b06c9f`](https://github.com/cBournhonesque/lightyear/commit/4b06c9fee9abedd55c7a5ae9bd34c44ba27003d2))
    - Optimize imports ([`86a0b26`](https://github.com/cBournhonesque/lightyear/commit/86a0b268eb307a452a6e52ad92b9e605e32a4b8a))
    - Merge pull request #278 from cBournhonesque/cb/message-kind ([`2326a70`](https://github.com/cBournhonesque/lightyear/commit/2326a70d28ce139adddb9749fcdd5382984224a8))
    - Fix lints ([`c4b4c20`](https://github.com/cBournhonesque/lightyear/commit/c4b4c20868db0c53af68af4d7ea7b76270ba79e4))
    - Merge branch 'main' into cb/message-kind ([`8de8a50`](https://github.com/cBournhonesque/lightyear/commit/8de8a505ba3be6178b898f74c656bb37e2c6205c))
    - Update book ([`2995934`](https://github.com/cBournhonesque/lightyear/commit/2995934d835e0f50ffd0073eadc1ca200b120adb))
    - Separate Replicate and ReplicateVisibility ([`eb4cb33`](https://github.com/cBournhonesque/lightyear/commit/eb4cb334725cd7154d25e6bd5ad050eb5e4023a4))
    - Fix inteprolation ([`113191c`](https://github.com/cBournhonesque/lightyear/commit/113191cb3fe7dbcd9d0af1d754063c8c60092bdc))
    - Improvements ([`41526e6`](https://github.com/cBournhonesque/lightyear/commit/41526e6c3991c7f656b3fb81615e9d1cd55b5ee7))
    - Introduce Replicated component ([`43fd2b4`](https://github.com/cBournhonesque/lightyear/commit/43fd2b42db73f655d0d313581b21ad7b6efa3eab))
    - Remove custom_interpolation option in interpolation_config ([`9f149c3`](https://github.com/cBournhonesque/lightyear/commit/9f149c3a181d8205ef75d33ae3fc3c096ef12ee4))
    - Add custom interpolation ([`774445e`](https://github.com/cBournhonesque/lightyear/commit/774445ed8cd6dfa53a7d9a74ee3689570eb11a2b))
    - Fix emessage rebroardcast for MapEntities ([`b944df7`](https://github.com/cBournhonesque/lightyear/commit/b944df7d93b0fa3341e53440329646dff0bfb69b))
    - Refactor protocol to re-use serialize fns ([`8437585`](https://github.com/cBournhonesque/lightyear/commit/843758595ce747d3a64e71bcdefe134928a29f16))
    - All examples compile ([`0233006`](https://github.com/cBournhonesque/lightyear/commit/0233006ce9f7c8150cef03afeb4fa3d7eb2e670e))
    - Fixed interpolation; host-server mode fails ([`aadf5c4`](https://github.com/cBournhonesque/lightyear/commit/aadf5c4c70ea9279305e2c49cd9c727d568dac1a))
    - Fix more events; prediction works but not interpolation ([`a3b1ebf`](https://github.com/cBournhonesque/lightyear/commit/a3b1ebf93a25f353ae6ad44a996fc3154c574b52))
    - Improve events handling ([`64cf094`](https://github.com/cBournhonesque/lightyear/commit/64cf09447949a0426ec9f9612d27060492848ad4))
    - Fix leafwing tests ([`1d3db5c`](https://github.com/cBournhonesque/lightyear/commit/1d3db5c5aaecf1df7154ddd8f23ef9763c0c3e11))
    - Tests compile ([`d780cbb`](https://github.com/cBournhonesque/lightyear/commit/d780cbb003b8568972a332db18adde12072c8205))
    - Fix tests ([`54fc68b`](https://github.com/cBournhonesque/lightyear/commit/54fc68b81ff24cb084d704d45bbb2135b659ea36))
    - Basic replication working ([`d553c82`](https://github.com/cBournhonesque/lightyear/commit/d553c82998773c4b65a9eebab5af5ddacd992b82))
    - Bug where the netids don't match? ([`399561b`](https://github.com/cBournhonesque/lightyear/commit/399561b05e43e70ee5a16f3562cf3dc84b5fc8d7))
    - Benchmark compiles ([`dcd5545`](https://github.com/cBournhonesque/lightyear/commit/dcd554502824685b6096a097cec6d6973439b646))
    - Add prediction/interpolation systems to the component registry ([`819bbff`](https://github.com/cBournhonesque/lightyear/commit/819bbff7e5d4fdeb5959d21d01d7dc138595d22b))
    - Remove protocol from prediction/interpolation ([`4c41a88`](https://github.com/cBournhonesque/lightyear/commit/4c41a882a6f20bb557fde31de290716301d203c3))
    - Remove protocol apart from prediction/interpolation ([`20b5081`](https://github.com/cBournhonesque/lightyear/commit/20b5081d17b2b91a6b10920122950873c188a119))
    - Add remove/write fn ([`88a3897`](https://github.com/cBournhonesque/lightyear/commit/88a38977e2bc570974e2a0ad99ccf0caa1de7219))
    - Enable message map_entities and rebroadcast ([`334a650`](https://github.com/cBournhonesque/lightyear/commit/334a6508f9546ad722080a389fe959c2cbaf6776))
    - Wip update replication systems ([`15d2f53`](https://github.com/cBournhonesque/lightyear/commit/15d2f5310047ae8a7291261e5268f3afeb79bbc9))
    - Add serialize test for ZST ([`9ce0816`](https://github.com/cBournhonesque/lightyear/commit/9ce0816ef9cc6fc82dc7bdf2b649e308977eb782))
    - Remove component protocol wip ([`03c09fa`](https://github.com/cBournhonesque/lightyear/commit/03c09fa0db31e8fc21797a3e82b1b752b83ee610))
    - Create component protocol ([`99ebafb`](https://github.com/cBournhonesque/lightyear/commit/99ebafb5c0c6e1066458775f943ec869db5b6659))
    - Port leafwing inputs ([`1cdec82`](https://github.com/cBournhonesque/lightyear/commit/1cdec82db28d6d2f1e803ed8e9d2d4be9f0baa32))
    - Fixed native input handling ([`3826b22`](https://github.com/cBournhonesque/lightyear/commit/3826b22d8d132f5b293d8c021ab91bfe30ceddbd))
    - Remove input protocol ([`e736477`](https://github.com/cBournhonesque/lightyear/commit/e736477a244f5481855a04aa5e97de429f387759))
    - Merge pull request #275 from cBournhonesque/cb/expose-ping-manager ([`a857c66`](https://github.com/cBournhonesque/lightyear/commit/a857c66dac69b682f4a56e3e840cdcf3453a9fbb))
    - Expose ping manager and final stats ([`74db39f`](https://github.com/cBournhonesque/lightyear/commit/74db39fd9ce78310e7a266c8ab794b968ad29379))
    - Update input ([`3baa7bd`](https://github.com/cBournhonesque/lightyear/commit/3baa7bdf2ddf6e77e93f00445610ccfa147e7b95))
    - Server can receive messages from client ([`e7b275c`](https://github.com/cBournhonesque/lightyear/commit/e7b275cf5d647f7f23995ebb9b55663b91f35389))
    - Send message works ([`876ce9a`](https://github.com/cBournhonesque/lightyear/commit/876ce9a3d918c7776bbad6a716aff62cd457dee6))
    - Wip remove message enum ([`9ce8e27`](https://github.com/cBournhonesque/lightyear/commit/9ce8e27bd5379a900fa59ae7974b718e13b30f22))
    - Merge pull request #272 from cBournhonesque/cb/debug-wasm-freeze ([`6425a6a`](https://github.com/cBournhonesque/lightyear/commit/6425a6a87e3e9bec984de5ed7a091a66fa06c55b))
    - Taplo fmt ([`d72a38d`](https://github.com/cBournhonesque/lightyear/commit/d72a38d8685d97171848c3ebbe746efee66b9b0c))
    - Cancel wasm webtransport tasks if the connection is closed ([`36b8d3e`](https://github.com/cBournhonesque/lightyear/commit/36b8d3e00f0c45ae01a5c407550538d314828c3f))
    - Merge pull request #271 from cBournhonesque/cb/fix-room-despawn ([`f5907c3`](https://github.com/cBournhonesque/lightyear/commit/f5907c390afc3870940e6a5cc01471a3722c3da2))
    - Do not despawn entities for clients who are not in the same room as the entity ([`6d53610`](https://github.com/cBournhonesque/lightyear/commit/6d536103444345234ce04722a7447f7fbc4fcbe9))
    - Merge branch 'dependabot/cargo/bevy-inspector-egui-0.24' of https://github.com/cBournhonesque/lightyear into dependabot/cargo/bevy-inspector-egui-0.24 ([`2285f46`](https://github.com/cBournhonesque/lightyear/commit/2285f46fd81f2b6b9361f49f9077ea448171c58e))
    - Merge pull request #267 from cBournhonesque/dependabot/cargo/xwt-core-0.3 ([`67a0093`](https://github.com/cBournhonesque/lightyear/commit/67a0093f15f063a5ebec56c8b8a66b43d32f0ac8))
    - Taplo ([`6240fda`](https://github.com/cBournhonesque/lightyear/commit/6240fda594a8ab9751f8467612aa4da7218be62c))
    - Upgrade xwt versions ([`a1e88bc`](https://github.com/cBournhonesque/lightyear/commit/a1e88bcb57ebfb40e3112f36e61c20a5eef66825))
    - Merge branch 'main' into dependabot/cargo/xwt-core-0.3 ([`a674455`](https://github.com/cBournhonesque/lightyear/commit/a674455510960b8de5e69b7aa9d9d2c6bfeede40))
    - Merge pull request #240 from cBournhonesque/cb/lobby-example ([`823e8c7`](https://github.com/cBournhonesque/lightyear/commit/823e8c75785b51d6638bc786379407a89254307d))
    - Disconnect client if the io fails ([`b314429`](https://github.com/cBournhonesque/lightyear/commit/b314429efbf0c5d2005fb2d7a85cc1a95b7b7cc6))
    - Improvements ([`cdc19a3`](https://github.com/cBournhonesque/lightyear/commit/cdc19a3c495fcd3bb79d5e853428b8abf73770ec))
    - Almost fully working demo ([`f3c91d3`](https://github.com/cBournhonesque/lightyear/commit/f3c91d3d8781e1808142546a89f16d0e4f9059df))
    - Add lobby list vs lobby ([`d4415ce`](https://github.com/cBournhonesque/lightyear/commit/d4415ce987828e77bdfce8f3d5c6f74ecb3aedbc))
    - Wip ([`7a67cec`](https://github.com/cBournhonesque/lightyear/commit/7a67cecaedba5f47ec24b196c60c5f341b304e82))
    - Merge branch 'main' into cb/lobby-example ([`965636c`](https://github.com/cBournhonesque/lightyear/commit/965636c161a5d0d3aa799c3818225e2b5dfd3aad))
    - Update xwt-core requirement from 0.2 to 0.3 ([`90950d9`](https://github.com/cBournhonesque/lightyear/commit/90950d90ebf1b577de20b300d97e40e8d7e65976))
    - Merge pull request #262 from cBournhonesque/cb/tick-wrap ([`600b3ef`](https://github.com/cBournhonesque/lightyear/commit/600b3ef874eda724c5663de65804c35bf2c8ebc9))
    - Run tick cleanup in all cases ([`9cec6f4`](https://github.com/cBournhonesque/lightyear/commit/9cec6f417d0fb87834557f8ca418297d76e07868))
    - Lobby works ([`9049a21`](https://github.com/cBournhonesque/lightyear/commit/9049a216dc5a2e436db5221819ea6afa3efb3d6d))
    - Merge branch 'main' into cb/lobby-example ([`4c7cccc`](https://github.com/cBournhonesque/lightyear/commit/4c7ccccf73ef2b30dfc105e90a2cf3b39edada2f))
    - Merge pull request #261 from cBournhonesque/cb/despawn-upon-disconnect ([`0fe6cf6`](https://github.com/cBournhonesque/lightyear/commit/0fe6cf61816fcbdae7533a54c8e62bcfdb7bfea5))
    - Improve test ([`71cb6b1`](https://github.com/cBournhonesque/lightyear/commit/71cb6b1e5e18806548e8a33a84146e9b435117ff))
    - Despawn resources on disconnect ([`adb39c3`](https://github.com/cBournhonesque/lightyear/commit/adb39c3157118ed2adb3caf5faa4f4ca3c8f93cb))
    - Merge branch 'main' into cb/lobby-example ([`732d597`](https://github.com/cBournhonesque/lightyear/commit/732d5979aa24a7ab5e17b14764526bec0e3ae704))
    - Merge pull request #256 from cBournhonesque/cb/io-connect-non-block ([`d8a46cb`](https://github.com/cBournhonesque/lightyear/commit/d8a46cb35961857131691576fd47b4ee7743eff7))
    - Disconnecting the netclient also disconnects the io early ([`685a7b8`](https://github.com/cBournhonesque/lightyear/commit/685a7b87b305dd75db9fdf43c17bac191436a1c5))
    - Add wasm ([`b4961c8`](https://github.com/cBournhonesque/lightyear/commit/b4961c8e1b1833eb6adb8d7e37fd8c712d63cf33))
    - Compiles. Pass a channel to receive the error from io conenction ([`1be1e43`](https://github.com/cBournhonesque/lightyear/commit/1be1e4300f311b4201f862f34ce317d4d50ec085))
    - Merge pull request #245 from cBournhonesque/dependabot/cargo/xwt-web-sys-0.9 ([`aac41eb`](https://github.com/cBournhonesque/lightyear/commit/aac41eba4cac7e25fb5c675bb84b35b1db16d4a1))
    - Wasm works ([`7a7b8e7`](https://github.com/cBournhonesque/lightyear/commit/7a7b8e7c0aec6315389560df2255dbdf57b5d97b))
    - Merge pull request #252 from simbleau/docs-update ([`c8aac52`](https://github.com/cBournhonesque/lightyear/commit/c8aac52b8d7f115b0e8deabd68ea49119d9da327))
    - Fix typos, update ring ([`f4985d9`](https://github.com/cBournhonesque/lightyear/commit/f4985d9f1c6c3fec718f11925060448f22c8be93))
    - Merge pull request #249 from cBournhonesque/cb/replicate-resources ([`b2cea72`](https://github.com/cBournhonesque/lightyear/commit/b2cea728c6a326a198233df7748f7b103e181011))
    - Improve docs ([`489fa68`](https://github.com/cBournhonesque/lightyear/commit/489fa684620467d04fe2d1847c86c1f2242efa28))
    - Clippy ([`ea66473`](https://github.com/cBournhonesque/lightyear/commit/ea66473037cfe3f6e07bcb61662935bba5bb5454))
    - Fix ComponentUpdate bug and finish ResourceReplication by adding commands ([`02d1fcc`](https://github.com/cBournhonesque/lightyear/commit/02d1fcc9a68057972f38fc5e446753bac15f580b))
    - Wip resource ([`8481009`](https://github.com/cBournhonesque/lightyear/commit/8481009ae4c6fba438835a3019b8ffe78f92223b))
    - Update xwt-web-sys requirement from 0.6 to 0.9 ([`009388e`](https://github.com/cBournhonesque/lightyear/commit/009388eb7c4b6499dcf30716a82f3f66ab783a57))
    - Wip replicate resources ([`dbeafa3`](https://github.com/cBournhonesque/lightyear/commit/dbeafa343f4159e1b0ce58d729e7a810ab21076c))
    - Merge pull request #244 from cBournhonesque/cb/simplify-protocolize ([`c788f55`](https://github.com/cBournhonesque/lightyear/commit/c788f55aaedaef856d8a8dd2be63883939763233))
    - Tests work ([`0f62781`](https://github.com/cBournhonesque/lightyear/commit/0f62781724322e437374590d60b78749e9d620f2))
    - Rework connect to only consist of changing the state ([`76ed549`](https://github.com/cBournhonesque/lightyear/commit/76ed5490a2879b98d78c220b806e9ee2aa6829c5))
    - Change state directly ([`34d9d56`](https://github.com/cBournhonesque/lightyear/commit/34d9d560b4b38c09798bec960c5a49835301e55e))
    - Merge pull request #242 from cBournhonesque/cb/fix-steam ([`0bb918c`](https://github.com/cBournhonesque/lightyear/commit/0bb918cf7daca3dedfb8fb46cbf990f22350178a))
    - Taplo ([`0943b4f`](https://github.com/cBournhonesque/lightyear/commit/0943b4fc48587772aba17d65a9e15ec9e3c981bd))
    - Tidy ([`0e92640`](https://github.com/cBournhonesque/lightyear/commit/0e9264020326e4b85d65a65216d0c12e607661e3))
    - Fix ([`b91f1e4`](https://github.com/cBournhonesque/lightyear/commit/b91f1e40b595b33a51ff256d516511b4e923ae40))
    - Fix ([`71f40ee`](https://github.com/cBournhonesque/lightyear/commit/71f40ee3c73d1773f0d1577759285fe6cf3851cc))
    - Fix ([`d6a8bb6`](https://github.com/cBournhonesque/lightyear/commit/d6a8bb64bbf74ade6532a5fd08114dceab02c35d))
    - Fix lifetimes ([`93124fd`](https://github.com/cBournhonesque/lightyear/commit/93124fdcb8ff125e40ea9f6e2dd4ff3070c534b3))
    - Try calling callbacks even when steam client is not connected ([`56ec2c7`](https://github.com/cBournhonesque/lightyear/commit/56ec2c75a3e11fe754eb5dc0de9bdad61b31945a))
    - Call callbacks even when not init ([`eb6a06f`](https://github.com/cBournhonesque/lightyear/commit/eb6a06fe13e94768b199d6f628f65d31eca6d5ca))
    - Add log ([`6745462`](https://github.com/cBournhonesque/lightyear/commit/67454620d694367e23de8b03ffbbfca00b035ce4))
    - Use sync wrapper ([`b634619`](https://github.com/cBournhonesque/lightyear/commit/b634619b1b256131c9c7b4acf5777494694a7045))
    - Use OnceLock ([`d8a6891`](https://github.com/cBournhonesque/lightyear/commit/d8a689124938168a3223f8223bfad9cd19796580))
    - Lazy static ([`4ea7eff`](https://github.com/cBournhonesque/lightyear/commit/4ea7eff10093cb5075d8542bfe5565bbccf449d6))
    - Drop client ([`a9874c7`](https://github.com/cBournhonesque/lightyear/commit/a9874c77def9ca9b38f578c8e4b63adbbb0f1687))
    - Disable options on client ([`5c9a903`](https://github.com/cBournhonesque/lightyear/commit/5c9a9037b4dac455c321a658221da3e8f382a4b3))
    - Wip ([`6f861b2`](https://github.com/cBournhonesque/lightyear/commit/6f861b24cf9f02dfba9c618f8f16fdedbfdc366f))
    - Remove other float option ([`bd93cc3`](https://github.com/cBournhonesque/lightyear/commit/bd93cc350f29dbfbd250f956c3a168a3b6b25493))
    - Wip ([`1575751`](https://github.com/cBournhonesque/lightyear/commit/1575751e9cd18f56a1e66ce25060f3c4cf08f9a1))
    - Wip ([`a0327cd`](https://github.com/cBournhonesque/lightyear/commit/a0327cd9b949b17442187c8b2d4db6de0dff89a3))
    - Add ClientConnectionExt ([`e95a0cf`](https://github.com/cBournhonesque/lightyear/commit/e95a0cfb19a38efab8077bef1f0397d7ceaaea3d))
    - Readd send_interval on server ([`aaa64ee`](https://github.com/cBournhonesque/lightyear/commit/aaa64eefde038913e39f288afa2d227946352cd6))
    - Connect using &mut World directly instead of a SystemParam ([`4139f37`](https://github.com/cBournhonesque/lightyear/commit/4139f37abf1bd7854e5f488aee295dc6a284c134))
    - Add more run conditions ([`f829b2f`](https://github.com/cBournhonesque/lightyear/commit/f829b2f0e6200d9927718ad659c65b727e59b9e2))
    - Add example ([`f389c3b`](https://github.com/cBournhonesque/lightyear/commit/f389c3b86ef97c387ca13ed8c729e7918afa04e7))
    - Adding server run conditions ([`c7ee523`](https://github.com/cBournhonesque/lightyear/commit/c7ee523b907faa67499d1a0249fe1992ee6eb344))
    - Wip ([`4c50bcb`](https://github.com/cBournhonesque/lightyear/commit/4c50bcb222ffdba7d46421ea41bd39c6427a2a5a))
    - Merge pull request #234 from cBournhonesque/cb/lobby-example ([`64226bf`](https://github.com/cBournhonesque/lightyear/commit/64226bfcddcba09d2047ce57c3e6ce70f7bf0326))
    - Fix tests ([`dad0898`](https://github.com/cBournhonesque/lightyear/commit/dad08982f181a8e31e0aa46bd3648ad42f0aa150))
    - Fix host server ([`756b30f`](https://github.com/cBournhonesque/lightyear/commit/756b30fb44b6fb0824e85a20eaf7252f9ac357e7))
    - Add connect button to simplebox example ([`451d410`](https://github.com/cBournhonesque/lightyear/commit/451d410a9d8911ca2c230251e35835a9a6efe67d))
    - Support cases where the client fails to connect ([`8e870c9`](https://github.com/cBournhonesque/lightyear/commit/8e870c9d44ef214d49107d02ac9a29fc69a13406))
    - Disable interpolation plugin when client is not synced ([`9519c00`](https://github.com/cBournhonesque/lightyear/commit/9519c0071bc6e5b694b6cbdd0b1a852f9d8da732))
    - Make predict work ([`4758455`](https://github.com/cBournhonesque/lightyear/commit/4758455e8f11c5941493597f4d8f4962f7ca54d7))
    - Fix ([`de67ed2`](https://github.com/cBournhonesque/lightyear/commit/de67ed27b50554e9881ad93e83e75e086964c79c))
    - Readd prediciton ([`dcc2453`](https://github.com/cBournhonesque/lightyear/commit/dcc24533187805d653803926a9a4fea416561e7f))
    - Debug ([`c2f53fb`](https://github.com/cBournhonesque/lightyear/commit/c2f53fb13293fe3286e6e92ac72935ee2a00b75f))
    - Merge branch 'main' into cb/lobby-example ([`cb4ed49`](https://github.com/cBournhonesque/lightyear/commit/cb4ed4951d6319d16d1e93a408881838d359519f))
    - Merge pull request #236 from cBournhonesque/cb/update-tutorial ([`604d886`](https://github.com/cBournhonesque/lightyear/commit/604d8869fd4279d66e523b1f46b89f5ac97be255))
    - Fix doc ([`e102477`](https://github.com/cBournhonesque/lightyear/commit/e1024779842a327f12f5ccaa595bf97208a430db))
    - Improve tutorial ([`65878e6`](https://github.com/cBournhonesque/lightyear/commit/65878e68ce7fdda913b76ca7ab99481c58b487b8))
    - Add missing reflect ([`0188108`](https://github.com/cBournhonesque/lightyear/commit/0188108f350aa8352ea6a842d2ef2aa318f316e2))
    - Merge branch 'main' into cb/lobby-example ([`e0e2143`](https://github.com/cBournhonesque/lightyear/commit/e0e21433d6a49ec19623c26f10cb7343d35f1355))
    - Connect/disconnect works, prediction doesn't ([`2659134`](https://github.com/cBournhonesque/lightyear/commit/26591347dc909e8b899368d6c3053e2802701e0d))
    - Add token expiry ([`e67c516`](https://github.com/cBournhonesque/lightyear/commit/e67c516858341251a1d080b18f6630209108defb))
    - Add state() api ([`53a9279`](https://github.com/cBournhonesque/lightyear/commit/53a9279091579ae3a12e7aa003f731d9467042eb))
    - Wip ([`7792792`](https://github.com/cBournhonesque/lightyear/commit/7792792b6fb1ae4d9bab8fde3ba2b0a227522a6b))
    - Fix ([`2b1b10c`](https://github.com/cBournhonesque/lightyear/commit/2b1b10c64f4082c578b8de368b9fcda6dd73d051))
    - Add connectio nstates ([`92138ba`](https://github.com/cBournhonesque/lightyear/commit/92138ba49f657b5f0c1a4fedf2f0e2495987dc31))
    - Merge pull request #235 from cBournhonesque/cb/add-reflect-bounds ([`f2bd1ff`](https://github.com/cBournhonesque/lightyear/commit/f2bd1ff8e8c27713f25f4753959b5cbb76863f32))
    - Add client Reflect bounds ([`0e5c4d4`](https://github.com/cBournhonesque/lightyear/commit/0e5c4d47dd070789a1ee15b6f0396ce257ba7600))
    - Merge pull request #233 from cBournhonesque/cb/add-example-docs ([`ba6dd0f`](https://github.com/cBournhonesque/lightyear/commit/ba6dd0fc2f0a5d465e3ddc4404134d431bccff94))
    - Add example docs ([`6d121f9`](https://github.com/cBournhonesque/lightyear/commit/6d121f93b0d309aa64b7c6a992295a26ff96c0f6))
    - Optimize imports ([`e673954`](https://github.com/cBournhonesque/lightyear/commit/e673954f68de55c4f016a4db83f3660a5c7e5802))
    - Merge pull request #232 from cBournhonesque/dependabot/cargo/mock_instant-0.4.0 ([`c48bc6f`](https://github.com/cBournhonesque/lightyear/commit/c48bc6f9165a4e7db8bd1081f33103e4987630a4))
    - Make test less flaky ([`6d6198f`](https://github.com/cBournhonesque/lightyear/commit/6d6198fa2944e805bc74e22baa0ab03fdba1039c))
    - Update mock_instant requirement from 0.3.1 to 0.4.0 ([`df7ee65`](https://github.com/cBournhonesque/lightyear/commit/df7ee658125ac9eb73a70db4a5906d08589ee3d1))
    - Quick fix ([`8ab1b3a`](https://github.com/cBournhonesque/lightyear/commit/8ab1b3a5fac8693c83e1270c7d69ce3c842ad834))
    - Merge pull request #231 from cBournhonesque/cb/add-server-disconnect ([`f66ead4`](https://github.com/cBournhonesque/lightyear/commit/f66ead4ed8849a6210f69f338506bd5402ceb990))
    - Clippy ([`5bcfbc7`](https://github.com/cBournhonesque/lightyear/commit/5bcfbc74190d4cc7fb246e103d18aacbd06a917e))
    - Add NetServer functions stop and disconnect ([`7a7e84e`](https://github.com/cBournhonesque/lightyear/commit/7a7e84e246570aa0be1ffce3fc634747768f82a3))
    - Wip ([`b6aeb3e`](https://github.com/cBournhonesque/lightyear/commit/b6aeb3e1d13d847d588d7a2572ab8f0f807dbafb))
    - Merge pull request #229 from cBournhonesque/cb/add-interp-immediately ([`5353dc5`](https://github.com/cBournhonesque/lightyear/commit/5353dc522de32bd5480eba895f6baaefc34394c4))
    - Add interpolation component value if enough time has passed ([`8dffd52`](https://github.com/cBournhonesque/lightyear/commit/8dffd5224af62b907022e00880b0a32904e8290c))
    - Wip ([`bbbdbdc`](https://github.com/cBournhonesque/lightyear/commit/bbbdbdc7fc41084b2ed58dae5c6e218a52710eae))
    - Add disconnect and stop functions to the server ([`0f26d6d`](https://github.com/cBournhonesque/lightyear/commit/0f26d6d86f647df1e10dac34b1b6168e5a25b261))
    - Disable MapEntities for PrePredicted and enable for LeafwingInputMessage ([`c59f2b1`](https://github.com/cBournhonesque/lightyear/commit/c59f2b11aaef39f851a2e7329dccb1b9cd42c726))
    - Merge pull request #227 from cBournhonesque/cb/fix-examples ([`1746c8e`](https://github.com/cBournhonesque/lightyear/commit/1746c8ef2fb5cc8b29ff067f5a45e698c4b4bff7))
    - Fix ([`9cffac2`](https://github.com/cBournhonesque/lightyear/commit/9cffac2d39cc9b648e81535fe6460878315cc17e))
    - Merge pull request #225 from cBournhonesque/cb/circumvent-orphan-rule ([`b07368d`](https://github.com/cBournhonesque/lightyear/commit/b07368ddeea9cb8082801aa15f1b708d7540ca6b))
    - Fix tests by adding run conditions to only run the networking systems if io is connected ([`0167cde`](https://github.com/cBournhonesque/lightyear/commit/0167cde1171adcc664e7ac3277d28aa18c4d9d63))
    - Some tests failing still ([`05835d2`](https://github.com/cBournhonesque/lightyear/commit/05835d2fcb4d08e2de48a6f5c6dbffae1c35da8d))
    - Update all exampels ([`ad619b8`](https://github.com/cBournhonesque/lightyear/commit/ad619b8c5d65038c9454d8183ebb02825ef3185e))
    - Require Message to be BitSerializable ([`f5c1065`](https://github.com/cBournhonesque/lightyear/commit/f5c106518ea3262f0746da9d592b253a054a96a3))
    - Remove named ([`d27fbcb`](https://github.com/cBournhonesque/lightyear/commit/d27fbcb33ef86d541badc7e0828c167344dc3c9b))
    - Tests pass ([`4e8c568`](https://github.com/cBournhonesque/lightyear/commit/4e8c568755380377b3df30db957bc861e540548c))
    - Add map_entities(custom) ([`8d0e6f8`](https://github.com/cBournhonesque/lightyear/commit/8d0e6f8570c071817a1ddece3c55dc4779059968))
    - Remove LightyearMapEntities, instead use ExternalMapper ([`8aaab8a`](https://github.com/cBournhonesque/lightyear/commit/8aaab8a1ffcccc28e495bacfc5852be69423975d))
    - Update macro ([`8291466`](https://github.com/cBournhonesque/lightyear/commit/829146676d0c4d0b335fea2106d67b8bbc3e34f6))
    - Merge pull request #222 from cBournhonesque/cb/add-client-disconnect ([`5de62df`](https://github.com/cBournhonesque/lightyear/commit/5de62df6aab04d09ec12495840d955bfe9ae05c6))
    - Fix tests, lint, etc. ([`a171d9f`](https://github.com/cBournhonesque/lightyear/commit/a171d9f572a721e6f70a1b31ea39c6feaa0e7165))
    - Working in all platforms ([`c2caaba`](https://github.com/cBournhonesque/lightyear/commit/c2caabaeee80115eae5b29df2d011a048553e039))
    - Wip ([`c1a58e5`](https://github.com/cBournhonesque/lightyear/commit/c1a58e500a8c2e4db1534c93e1d004eb42cf976b))
    - Wip on traits ([`0eee87a`](https://github.com/cBournhonesque/lightyear/commit/0eee87aeb3d45bf20945071bed91a1d7134028a9))
    - Wip ([`9809e8a`](https://github.com/cBournhonesque/lightyear/commit/9809e8ae38967560de9e762b61a1ea259d9a9008))
    - Add close method to packet sender / packet receiver ([`ba64c1f`](https://github.com/cBournhonesque/lightyear/commit/ba64c1f649c57b57aa6f726d6cb9de2e03128f8f))
    - Add disconnect function on NetClient ([`f4cfdad`](https://github.com/cBournhonesque/lightyear/commit/f4cfdad7bb8251873adc71555c40ac8198f20ebe))
    - Merge pull request #219 from cBournhonesque/cb/0.13 ([`9e3b053`](https://github.com/cBournhonesque/lightyear/commit/9e3b0532a7bb0296f6a5265b3b447dfee38509fe))
    - Clippy lint ([`0b188fe`](https://github.com/cBournhonesque/lightyear/commit/0b188fe57ce42af2c1d9a5cc2357c4584d13b015))
    - 0.13 ([`12d75c7`](https://github.com/cBournhonesque/lightyear/commit/12d75c7281ca79440633ed048ea9c00410b5521f))
    - Merge pull request #218 from cBournhonesque/cb/fix-wasm ([`b29db4f`](https://github.com/cBournhonesque/lightyear/commit/b29db4f1fd927a4ce4d73d65149acc3af863d0d8))
    - Fix wasm ([`0bc4ef8`](https://github.com/cBournhonesque/lightyear/commit/0bc4ef819471d676e98536b6011e43e13280136e))
    - Revert "0.13" ([`be9bd0a`](https://github.com/cBournhonesque/lightyear/commit/be9bd0a91692a429ed45abcf1bd9f68f7028604f))
    - 0.13 ([`5bb7631`](https://github.com/cBournhonesque/lightyear/commit/5bb763119a28e5f2428ee63f13dafc09d1de0927))
    - Merge pull request #212 from cBournhonesque/cb/listen-server-server-only ([`e23548b`](https://github.com/cBournhonesque/lightyear/commit/e23548b49afa8a5eb70742677621ee3c56a15e6f))
    - Lint/clippy ([`65b2826`](https://github.com/cBournhonesque/lightyear/commit/65b282616586969eacb22da5b5b9ea5acdb2b89f))
    - All examples are working, improve readmes, fix logs ([`eb9e022`](https://github.com/cBournhonesque/lightyear/commit/eb9e0226a1ba3e4888bc72679402bb4d5e431bee))
    - Fix pre-prediction. Order is now prespawn->preprediction->prediction. Local PrePredicted components in host-server get removed. ([`4ab4475`](https://github.com/cBournhonesque/lightyear/commit/4ab447528282e2a3d7cd85a13f4c9fc716d0b96c))
    - Remove client input_leafwing plugin in HostServer mode ([`f57f0ff`](https://github.com/cBournhonesque/lightyear/commit/f57f0fff082400c2a062115edd83fdbd05e32058))
    - Try adding interpolate plugin for custom interpolation ([`33c8239`](https://github.com/cBournhonesque/lightyear/commit/33c8239be8d0741eb6ea9d526e94ca594865f3e8))
    - Adjustments to ConnectEvents ([`55242b2`](https://github.com/cBournhonesque/lightyear/commit/55242b2fd8c88b6639899c30e226cccb602ba1cc))
    - Fix bullet prespawn ([`dd74293`](https://github.com/cBournhonesque/lightyear/commit/dd7429320c10d5091e23e62387873347196f68b9))
    - Wip ([`8875dbd`](https://github.com/cBournhonesque/lightyear/commit/8875dbd35165e44b6896351ff7f9167fd59d68e7))
    - Merge branch 'main' into cb/listen-server-server-only ([`aa42cc1`](https://github.com/cBournhonesque/lightyear/commit/aa42cc18bde0b99a9fc66d6089fb9409bdef5597))
    - Merge pull request #210 from cBournhonesque/cb/client-id-enum ([`7134b29`](https://github.com/cBournhonesque/lightyear/commit/7134b29c4d4844a1438363330a1ea5e896498c11))
    - Fix examples ([`0b8f3f3`](https://github.com/cBournhonesque/lightyear/commit/0b8f3f3cf3565bd8de6886ce3c03bcf5b4b2c59c))
    - Merge pull request #209 from cBournhonesque/cb/client-id-enum ([`30738cf`](https://github.com/cBournhonesque/lightyear/commit/30738cf474fc49ea213ded471dde40f3335e6235))
    - Fix tests ([`94cfe07`](https://github.com/cBournhonesque/lightyear/commit/94cfe072120e0df9525be536922d5b13057c58df))
    - Add ClientId enum, remove GlobalMetadata, add client connection/disconnection events ([`f75b89c`](https://github.com/cBournhonesque/lightyear/commit/f75b89ced4477b2c172810d67dba336a4de10b45))
    - Wip ([`447a873`](https://github.com/cBournhonesque/lightyear/commit/447a873345b16fee22bf1922fd5f8f285a700e4a))
    - Merge pull request #207 from cBournhonesque/cb/unified-with-client-io ([`4969507`](https://github.com/cBournhonesque/lightyear/commit/4969507a5b7c3db164af9671bb49a0f2cd4b804e))
    - Improve bullet_prespawn ([`a6f3e54`](https://github.com/cBournhonesque/lightyear/commit/a6f3e5467d95e7d357a6121bd4bf89d341b88375))
    - Remove Flush sets, fix client_replication example ([`3a0712f`](https://github.com/cBournhonesque/lightyear/commit/3a0712fd65062384774506c21d0405d9c1c6ea54))
    - Improve input handling ([`b6d44ef`](https://github.com/cBournhonesque/lightyear/commit/b6d44ef8c57fbfddb9844df436ff204a268d916d))
    - Separate MainSet for client/server via a marker ([`41d700b`](https://github.com/cBournhonesque/lightyear/commit/41d700bfddf5b0a6553ece9bb3fc3f31668bfa1c))
    - Add clean_buffers function for input, get leafwing input working in unified, improve priority/leafwing_inputs examples ([`1ce2429`](https://github.com/cBournhonesque/lightyear/commit/1ce2429e73712cb6701f2316db94498241224e6b))
    - Make leafwing inputs work with unified ([`d9cf52c`](https://github.com/cBournhonesque/lightyear/commit/d9cf52c14bb09756113f469bd6175f661787ce17))
    - Got interpolation time to be synced to normal time ([`f233b39`](https://github.com/cBournhonesque/lightyear/commit/f233b394e6283eb06097f4be89a800db8a362075))
    - Progress ([`fca0622`](https://github.com/cBournhonesque/lightyear/commit/fca0622f3a86f4cf15eee8c1016d84baef1889bc))
    - Wip update prediction/interpolation plugins ([`dd4565e`](https://github.com/cBournhonesque/lightyear/commit/dd4565e6351a927b27112e0b1c147caab451230d))
    - Add input manager and remove input_buffer from connection manager ([`4ca6e5e`](https://github.com/cBournhonesque/lightyear/commit/4ca6e5edbcf7c9f797e063c0a3e90e5749923ea0))
    - Wip ([`d173438`](https://github.com/cBournhonesque/lightyear/commit/d173438ae366a57cb6f3b1b225c78b1b374559d1))
    - Wip ([`6982a6d`](https://github.com/cBournhonesque/lightyear/commit/6982a6d34a87c18ed4e33e932f7edff02e5ede22))
    - Merge pull request #203 from cBournhonesque/cb/interest-management-bug ([`47aaff7`](https://github.com/cBournhonesque/lightyear/commit/47aaff7e049fe6e5d508a33cf69d26e45bd8c3ea))
    - Lint ([`055d9ad`](https://github.com/cBournhonesque/lightyear/commit/055d9ad64c2d48417807f63fc662ec8dcee32743))
    - Nit ([`3ee0d60`](https://github.com/cBournhonesque/lightyear/commit/3ee0d60f75676da025ea347bb925f1d3e062b11f))
    - Fix wasm ([`14e3aff`](https://github.com/cBournhonesque/lightyear/commit/14e3affcd30347b616d3d826c300660d7059fa3d))
    - Wip ([`f308c16`](https://github.com/cBournhonesque/lightyear/commit/f308c168284d1f7b2a6ecfce90feb2d183bb1017))
    - Fix logic ([`9580330`](https://github.com/cBournhonesque/lightyear/commit/9580330f7f5754c3c33f2b50726e958a089aee2f))
    - Fix room logic ([`4e55e33`](https://github.com/cBournhonesque/lightyear/commit/4e55e33b943382d69eb9a9926c45a7bdfb7ad4b0))
    - Fix wtransport ([`e28103c`](https://github.com/cBournhonesque/lightyear/commit/e28103c762efc87de230e925c433e63c9577926e))
    - Add logs ([`8e9d0b3`](https://github.com/cBournhonesque/lightyear/commit/8e9d0b3c30ca18de71a54d5dc2c590e0626b9eb0))
    - Fix rustdoc ([`d55e35c`](https://github.com/cBournhonesque/lightyear/commit/d55e35c3b4cff640498dfa231065edd038580917))
    - Clippy ([`9630392`](https://github.com/cBournhonesque/lightyear/commit/9630392bdccc37d5c58b865d5d54a44643975f2d))
    - Add test for client and entities moving rooms ([`37a217e`](https://github.com/cBournhonesque/lightyear/commit/37a217e3ef15700b7c1ab1f13dbe64bd6f4d3aed))
    - Merge pull request #191 from df51d/patch-1 ([`d8d0648`](https://github.com/cBournhonesque/lightyear/commit/d8d06483c41864a8518ab645d06f3f7a98ca8703))
    - Merge pull request #192 from cBournhonesque/cb/cleanup-resource ([`4af14a1`](https://github.com/cBournhonesque/lightyear/commit/4af14a1b95eab68e42d200dd52eff7495367790d))
    - Remove the client::resource and server::resource modules ([`972aa5e`](https://github.com/cBournhonesque/lightyear/commit/972aa5eebe9ed55f26dfd7416bce9dd8eb1b113d))
    - Use select instead of channel to notify about closed task ([`2aa4eb0`](https://github.com/cBournhonesque/lightyear/commit/2aa4eb0d7fd0734a826d8093c4873d5d80272a91))
    - Merge pull request #190 from cBournhonesque/cb/sync-optim ([`41e0838`](https://github.com/cBournhonesque/lightyear/commit/41e08389dd0871ced30b691816c58af7ff20e312))
    - Run hierarchy send plugins only at send_interval ([`f57c19d`](https://github.com/cBournhonesque/lightyear/commit/f57c19d46cdcbd61eef3cd7b954e04555b64480f))
    - Merge pull request #187 from cBournhonesque/cb/buffer-pool ([`b3a93f8`](https://github.com/cBournhonesque/lightyear/commit/b3a93f8bc5015c408de2ab8cfafa976be65d1950))
    - Clean PR ([`3a72d55`](https://github.com/cBournhonesque/lightyear/commit/3a72d555b86c88b2176ad610550b33804b7be24c))
    - Refactor reader to immediately decode value, and use buffer pool to re-use memory ([`37f9ce7`](https://github.com/cBournhonesque/lightyear/commit/37f9ce7ae43b7cfb38bc0a1777448fbf733057b7))
    - Merge pull request #158 from cBournhonesque/cb/steam ([`31f67c3`](https://github.com/cBournhonesque/lightyear/commit/31f67c3e8239c985ab6ade06eaea29932071f25f))
    - Convert all examples to steam ([`72fa4c5`](https://github.com/cBournhonesque/lightyear/commit/72fa4c55e8c46cb8e964846bb19133f600a4dbb2))
    - Nit ([`a8d5626`](https://github.com/cBournhonesque/lightyear/commit/a8d5626eeadb7fc784d97e2bcbf75a2287e61663))
    - Merge pull request #182 from zwazel/cb/steam ([`4e29f22`](https://github.com/cBournhonesque/lightyear/commit/4e29f22ef1b082b7cb8b158af593996e7a7f14fa))
    - Steam works ([`1fa71b3`](https://github.com/cBournhonesque/lightyear/commit/1fa71b345b91ef5c6b72df2a1e13cd6b2947f0e9))
    - Merge pull request #181 from cBournhonesque/cb/clean-todos ([`eb87ad9`](https://github.com/cBournhonesque/lightyear/commit/eb87ad9f8a206f13bcf9c0bc683a485e3bfd17f3))
    - Clean up some todo comments ([`aae1e42`](https://github.com/cBournhonesque/lightyear/commit/aae1e42c59195f5aefdbcf703339a486eb293a5d))
    - Lint ([`14778f0`](https://github.com/cBournhonesque/lightyear/commit/14778f09a3db24ce30d6b7fc8c2200323dadf9ec))
    - Merge branch 'main' into cb/steam ([`303c766`](https://github.com/cBournhonesque/lightyear/commit/303c766e70a2d969242ffd9e17b0446fe910b6b0))
    - Fix ([`f66b4a6`](https://github.com/cBournhonesque/lightyear/commit/f66b4a6b3df10519568a8116ae78c0a341e8e842))
    - Merge pull request #179 from cBournhonesque/cb/0.12.0 ([`5077a58`](https://github.com/cBournhonesque/lightyear/commit/5077a58374e6a2b487c16cb923a7e1fb29889ee2))
    - 0.12.0 release ([`d35eded`](https://github.com/cBournhonesque/lightyear/commit/d35eded128cdcdf99f9f91c07e3576727ee1cb58))
    - Merge pull request #177 from cBournhonesque/cb/listen-server ([`42eb069`](https://github.com/cBournhonesque/lightyear/commit/42eb06913b9103f8313151d2aac8cae4b5a6a334))
    - Update all examples to support listen-server mode ([`7844178`](https://github.com/cBournhonesque/lightyear/commit/7844178c22c94e534db7e9d58b4e7f2e5748cbee))
    - Merge pull request #169 from cBournhonesque/cb/multi-transport ([`28a18db`](https://github.com/cBournhonesque/lightyear/commit/28a18dbad5f35922cb1207bf17f0b896956186ce))
    - Clean PR ([`18cc5fd`](https://github.com/cBournhonesque/lightyear/commit/18cc5fde47f58e56845a77fb5a9afe63c10d0106))
    - Fix leafwing input ([`c1fa81d`](https://github.com/cBournhonesque/lightyear/commit/c1fa81d6d4f88e118639968806b6a3aa20aefc29))
    - Wip ([`1a0f894`](https://github.com/cBournhonesque/lightyear/commit/1a0f894ba617a893629613296406663377ce2c02))
    - Address comments and updated all examples ([`578c967`](https://github.com/cBournhonesque/lightyear/commit/578c967be88c9a374cbad9d6c08aa0ff38ba1de7))
    - Merge pull request #171 from cBournhonesque/cb/qualify-protocolize ([`80fa6ee`](https://github.com/cBournhonesque/lightyear/commit/80fa6ee7f65d1a177a06dc239c202fa5b12b26cc))
    - Fix qualify ([`e4fad6e`](https://github.com/cBournhonesque/lightyear/commit/e4fad6e4edbf75d6d0296599b7de3f2ba9049e6a))
    - Add unit test for multi transport ([`953c3b0`](https://github.com/cBournhonesque/lightyear/commit/953c3b02312f7d5da0c0cf0008ce7e0dd1308317))
    - Update steppers ([`30b30f4`](https://github.com/cBournhonesque/lightyear/commit/30b30f40b842b1d00c5104fc972434421e75f740))
    - Add GlobalMetadata so that the example works even if two clients have colliding ClientIds ([`8767fae`](https://github.com/cBournhonesque/lightyear/commit/8767fae3109e702011208f1f3ba54f4bb926d146))
    - Wip update settings ([`4dc75d6`](https://github.com/cBournhonesque/lightyear/commit/4dc75d6deeadb7e7820de0f54bbad45dfb20d85b))
    - Fix id map ([`456cab7`](https://github.com/cBournhonesque/lightyear/commit/456cab7a425e2c92f1884760fbcd90219ae5a4ec))
    - Basic version working ([`7ff0e03`](https://github.com/cBournhonesque/lightyear/commit/7ff0e03f77b9260004b95b12e2628807c83fc176))
    - Merge pull request #162 from cBournhonesque/cb/remove-replicate-command ([`4e2dcea`](https://github.com/cBournhonesque/lightyear/commit/4e2dceab75d17d00a57d7e2280ad47df7e0fa7e3))
    - Fix ([`05176ac`](https://github.com/cBournhonesque/lightyear/commit/05176ac0ce64dde4d28360c400a8e6205d17b40d))
    - Remove ([`ec9d555`](https://github.com/cBournhonesque/lightyear/commit/ec9d5559bd3269f4859753c704592b04364b5f9f))
    - Reorg plugins ([`2efc108`](https://github.com/cBournhonesque/lightyear/commit/2efc1080c119fcadac273e0217955e4bbb3505e6))
    - Merge pull request #159 from molixianggu/main ([`1dde3fc`](https://github.com/cBournhonesque/lightyear/commit/1dde3fcbb1b1bfbd1561322a028e92b8c5a4ff87))
    - Add more logs ([`144f9ec`](https://github.com/cBournhonesque/lightyear/commit/144f9ec33da51d51f079bdf95e80af445bf1da1d))
    - Add logs ([`cd460e0`](https://github.com/cBournhonesque/lightyear/commit/cd460e01ab9aa45c42b10db7c32dd9190afaa8ef))
    - Add io to steam client ([`7f6160d`](https://github.com/cBournhonesque/lightyear/commit/7f6160d967f4f475a62fd813617ddc0f4010620b))
    - Merge branch 'main' into cb/steam ([`a1870d8`](https://github.com/cBournhonesque/lightyear/commit/a1870d87ca873b6dc4ed651f496e70353516b370))
    - Merge pull request #161 from cBournhonesque/cb/upgrade-deps ([`e798f0a`](https://github.com/cBournhonesque/lightyear/commit/e798f0a92f931b7307699ca316a304e0a1e5933c))
    - Upgrade xwt ([`b2549d0`](https://github.com/cBournhonesque/lightyear/commit/b2549d0fa9d0b9d32d9b2ebf0360acf22a6744a4))
    - Fix ([`72483c2`](https://github.com/cBournhonesque/lightyear/commit/72483c2e57f2aa74c76e69051d27d8b0547b62de))
    - Fix ([`5ab02bf`](https://github.com/cBournhonesque/lightyear/commit/5ab02bf66f6c90ea1d0a2a52fe70b443085bff97))
    - Add try_from_bytes method to ConnectToken ([`86f20c7`](https://github.com/cBournhonesque/lightyear/commit/86f20c79f6930d19ecc3cbf5b97a7e36b6b5b7a7))
    - Fix small bugs ([`891556e`](https://github.com/cBournhonesque/lightyear/commit/891556e61c75180691cc5f3aaef881cf3acf6b6e))
    - Fix small bugs ([`b5a77b6`](https://github.com/cBournhonesque/lightyear/commit/b5a77b684eba58372b88a16a70f880fa53012903))
    - Add steam prototype ([`041ac34`](https://github.com/cBournhonesque/lightyear/commit/041ac34b645109eb6e24e0d532ce129c3f54d597))
    - Add steam ([`5329b90`](https://github.com/cBournhonesque/lightyear/commit/5329b90408025103b4b4725be6b4e4f58401dba3))
    - Merge pull request #151 from cBournhonesque/cb/0.13 ([`491ff18`](https://github.com/cBournhonesque/lightyear/commit/491ff18e07ca56b1d17cae2edd76f5a4009bfa32))
    - Uprade version ([`03f0531`](https://github.com/cBournhonesque/lightyear/commit/03f0531e6fafe7cdae376e53751735077cc74537))
    - Fix benchmark ([`9f29345`](https://github.com/cBournhonesque/lightyear/commit/9f2934540315b50e1225c5a30cea1c199e74512d))
    - Clean logs ([`e7aa368`](https://github.com/cBournhonesque/lightyear/commit/e7aa368f35fb986df0970994384792f274d6677c))
    - Continue removing FixedUpdate ([`ab0fda5`](https://github.com/cBournhonesque/lightyear/commit/ab0fda581807beb8eb18c37c13d5ec17c93db8a6))
    - Remove FixedUpdateSet::Main SystemSet and instead use FixedPreUpdate and FixedPostUpdate ([`ba0e874`](https://github.com/cBournhonesque/lightyear/commit/ba0e87476e6f41c1ebf06e5a1ba214cafea3fe6c))
    - Wip ([`137b7c3`](https://github.com/cBournhonesque/lightyear/commit/137b7c37bc27600650de9680344e2cc478b2482d))
    - Upgrade leafwing ([`2cd8a6f`](https://github.com/cBournhonesque/lightyear/commit/2cd8a6f794bb42a0673cf41490cd55ba8ecc36c9))
    - Add hierarchy plugin to replicate hierarchies ([`01d0122`](https://github.com/cBournhonesque/lightyear/commit/01d0122feb97b11d1f8c4b92ffc33b9878acc5a0))
    - Apply ant's changes ([`2dd56e0`](https://github.com/cBournhonesque/lightyear/commit/2dd56e00f702b3be1db2e040d180286569e55cfc))
    - Fix more stuff ([`dbefd6f`](https://github.com/cBournhonesque/lightyear/commit/dbefd6f8e3056c91b0dbb43bc7ed7bb5ebe625f7))
    - Wip move to 0.13 ([`e1d1b79`](https://github.com/cBournhonesque/lightyear/commit/e1d1b795426d1a3313bb6b936e3e9334f7b691f8))
    - Merge pull request #150 from cBournhonesque/cb/fix-examples ([`4895238`](https://github.com/cBournhonesque/lightyear/commit/48952388c4c28ad7b1dd4e0740d98c1bff52e4cd))
    - Fix examples ([`ae03eb8`](https://github.com/cBournhonesque/lightyear/commit/ae03eb8209c03109116d0960db84c82af70e2d84))
    - Merge pull request #149 from cBournhonesque/cb/mid-tick-interpolate ([`ca308d6`](https://github.com/cBournhonesque/lightyear/commit/ca308d602927f38aeee5e233eafd956e478a64a3))
    - Clean PR ([`6950d4b`](https://github.com/cBournhonesque/lightyear/commit/6950d4b0f7c274b0c0990cf1bdce64b75b460987))
    - Update example and log level ([`edcc2a6`](https://github.com/cBournhonesque/lightyear/commit/edcc2a6a6acac17f9edcb4c60dbc62a2db227426))
    - Add visual interpolation ([`5bec5f1`](https://github.com/cBournhonesque/lightyear/commit/5bec5f1257bb19748773ecfc184dbcff67b8a101))
    - Add plugin ([`93bd6ff`](https://github.com/cBournhonesque/lightyear/commit/93bd6ff36be77724be09c53d75b44bc8886fdefa))
    - Add unit tests for visual interpolation ([`fcfb4e7`](https://github.com/cBournhonesque/lightyear/commit/fcfb4e7f6ae26c917f2c457607c5342561c01ebc))
    - Wip visual interp ([`68f5ecc`](https://github.com/cBournhonesque/lightyear/commit/68f5ecc74469806085da0b912b65b140a38a6dca))
    - Merge pull request #147 from cBournhonesque/0.9.0 ([`808a3f6`](https://github.com/cBournhonesque/lightyear/commit/808a3f6780785e6846141a677150ab86b2c6455f))
    - Pdate ([`ae33781`](https://github.com/cBournhonesque/lightyear/commit/ae337818cdfdd3590226995367ea6af3560e1dbe))
    - Update versions ([`727b81c`](https://github.com/cBournhonesque/lightyear/commit/727b81cc0925daf60a87a054069b028a0cce13f0))
    - Remove Eq bound for inputs ([`9de6333`](https://github.com/cBournhonesque/lightyear/commit/9de6333d36f39a0d8d4c9ff9a71c51c527d50a08))
    - Correctly drop tasks ([`b2237fe`](https://github.com/cBournhonesque/lightyear/commit/b2237fe7458d01828b5decb9e57179a5bd99f19a))
    - Revert xwt version ([`173f41d`](https://github.com/cBournhonesque/lightyear/commit/173f41df8cc6fe501372f4fa8c11729a20ff41b8))
    - Fixes ([`a53788e`](https://github.com/cBournhonesque/lightyear/commit/a53788e7bcf24f92948212349609579411ba3482))
    - Merge pull request #141 from cBournhonesque/cb/enable-token-later ([`4fe4ccf`](https://github.com/cBournhonesque/lightyear/commit/4fe4ccf6fd2cb4d7bda2a80fad83045e4bcad45b))
    - Fmt ([`3cac5a8`](https://github.com/cBournhonesque/lightyear/commit/3cac5a88ef4a3dd94e49db6d7d90fc8b28f42845))
    - Clean PR ([`9bce81d`](https://github.com/cBournhonesque/lightyear/commit/9bce81dcef5488fd3e25d582096d6e837ebb70a3))
    - Update all examples to stop using tokio runtime ([`e69f86a`](https://github.com/cBournhonesque/lightyear/commit/e69f86ab69ce4c2f49965d358f680478d716353d))
    - Working example without tokio runtime! and async-compat ([`93810dd`](https://github.com/cBournhonesque/lightyear/commit/93810ddd62ae59c69158e00a71d0d1c3fa6cd1f5))
    - Try async compat ([`5e6ddbc`](https://github.com/cBournhonesque/lightyear/commit/5e6ddbc787986ae5e1f5ea0404de6461ac79858b))
    - Initialize plugin using IoConfig, not io, and create the io only at connection time ([`a0b1459`](https://github.com/cBournhonesque/lightyear/commit/a0b1459894e3ec9caad1a7c6f40a96e577871b64))
    - Wip being able to connect later on ([`eeea194`](https://github.com/cBournhonesque/lightyear/commit/eeea19456c110bf3c7faf661409421f06573aedd))
    - Fix ([`1417513`](https://github.com/cBournhonesque/lightyear/commit/14175133111e1551c170908c18d12b074f904ed7))
    - Fix ([`e40221a`](https://github.com/cBournhonesque/lightyear/commit/e40221afab47d90f25c1c55c9705ca4b02de1a61))
    - Merge pull request #139 from cBournhonesque/cb/add-leafwing-input-set ([`d71f5d2`](https://github.com/cBournhonesque/lightyear/commit/d71f5d2552eeaef78f05fbd6bf20ca4ec3d021f0))
    - Add a ToggleActions resource on the client to control whether or not to run leafwing input plugin systems ([`a5ce769`](https://github.com/cBournhonesque/lightyear/commit/a5ce769b4cb20c512d48bbad707b7af8e7015c57))
    - Merge pull request #128 from cBournhonesque/feat-add-websocket-transport ([`fc5dbcb`](https://github.com/cBournhonesque/lightyear/commit/fc5dbcb71c0189fd4e7e7070263a69f22ad0dea1))
    - Sending now after opening socket and uncomment packetsender impl code ([`f96c5fa`](https://github.com/cBournhonesque/lightyear/commit/f96c5fa8fcaabb61d884baf1ee9ce6d6f37d5322))
    - Merge pull request #134 from cBournhonesque/dependabot/cargo/xwt-web-sys-0.4.7 ([`135f3ff`](https://github.com/cBournhonesque/lightyear/commit/135f3ff8bf14c21570b95330e1d4b8cc2e99e0c2))
    - Merge branch 'main' into feat-add-websocket-transport ([`41c2c6d`](https://github.com/cBournhonesque/lightyear/commit/41c2c6d3b9d6e35087e05ec6d20ba8a84b812fc9))
    - Update xwt-web-sys requirement from 0.3.1 to 0.4.7 ([`a2ca8cf`](https://github.com/cBournhonesque/lightyear/commit/a2ca8cf71a403f3a340dc54938e600cfceb11e8a))
    - Fix' ([`b2d2119`](https://github.com/cBournhonesque/lightyear/commit/b2d2119ea8964e1ca2a692ff6e0340709858ac57))
    - Merge pull request #132 from cBournhonesque/cb/add-tests ([`7b944a4`](https://github.com/cBournhonesque/lightyear/commit/7b944a4b915c51817adf12a24a79b63aa74a63fc))
    - Remove tracing subscriber ([`c010373`](https://github.com/cBournhonesque/lightyear/commit/c0103735ef4204339837668aefbbd1fab64abf86))
    - Enable tests with leafwing feature on ([`0da173a`](https://github.com/cBournhonesque/lightyear/commit/0da173a06760e761f0f237a63d29d938cb5f0785))
    - Merge pull request #129 from cBournhonesque/cb/improve-docs ([`1eb7d77`](https://github.com/cBournhonesque/lightyear/commit/1eb7d77211380cf37addbbd966065bcb1d063c10))
    - Ignore tests ([`cf8c715`](https://github.com/cBournhonesque/lightyear/commit/cf8c715b77c7d493875727628958de3e1d66e263))
    - Doc improvements ([`65314c3`](https://github.com/cBournhonesque/lightyear/commit/65314c39adae28dff2394385fc7394a08257768c))
    - Remove rivet stuff and make example compile with wasm ([`62c9b10`](https://github.com/cBournhonesque/lightyear/commit/62c9b10909d29c5fa37d11d6e56f4e425c90a2cf))
    - Use io task pool instead of tokio::spawn and change receiver / sender ([`c3f1faf`](https://github.com/cBournhonesque/lightyear/commit/c3f1faf98d770390323f1821861bf80e3e9d0415))
    - Merge pull request #123 from cBournhonesque/feat-add-websocket-transport ([`d025041`](https://github.com/cBournhonesque/lightyear/commit/d02504193cebcc878b8b6d85b2595f870a8cd812))
    - Adding websockets to all examples ([`f3c495e`](https://github.com/cBournhonesque/lightyear/commit/f3c495e8c0561470ea791ec9352577bd4c83fde7))
    - Merge branch 'main' into feat-add-websocket-transport ([`3ce4196`](https://github.com/cBournhonesque/lightyear/commit/3ce4196f360e2c35b17f32421777331ccfd6a331))
    - Nagle's algo, handle connection closes ([`8e04fb1`](https://github.com/cBournhonesque/lightyear/commit/8e04fb13a8d69d54bb54088a71609100c7863778))
    - Implement wasm & fix small mistakes ([`d11f597`](https://github.com/cBournhonesque/lightyear/commit/d11f597140a81b73656dabb585e2ecca62fd208b))
    - Merge pull request #108 from cBournhonesque/cb/rivet ([`5b9176e`](https://github.com/cBournhonesque/lightyear/commit/5b9176e131504142866fa36654675a7de9e30797))
    - Format ([`dfe8e93`](https://github.com/cBournhonesque/lightyear/commit/dfe8e9313ade238c9b048a8fa9b8bf6a659cb376))
    - Fix all examples ([`4e00d68`](https://github.com/cBournhonesque/lightyear/commit/4e00d68f67be9e5f696e72f698b8ce4e1dd04b66))
    - Merge branch 'main' into cb/rivet ([`c55dca8`](https://github.com/cBournhonesque/lightyear/commit/c55dca8b59fb731292757c19c781ec15a4f73051))
    - Update replication group api ([`44777f0`](https://github.com/cBournhonesque/lightyear/commit/44777f0c15aa6b96a7f147a9ba04320336014b76))
    - Merge pull request #106 from cBournhonesque/cb/priority ([`76d7e03`](https://github.com/cBournhonesque/lightyear/commit/76d7e03364943ce3d81400f7b7a38b9b25ff684d))
    - Fix warns ([`7f2e8a0`](https://github.com/cBournhonesque/lightyear/commit/7f2e8a07187bf71db4d2c7efcd1eaa650ca83735))
    - Update readme ([`4e9827c`](https://github.com/cBournhonesque/lightyear/commit/4e9827cbd7974ebebab2c0ebb84a7de26eff5022))
    - Merge pull request #124 from cBournhonesque/Nul-led-patch-1 ([`797846e`](https://github.com/cBournhonesque/lightyear/commit/797846ebe7aa91cf910b4268f3f7f165b830d669))
    - Small ownership issues ([`4103090`](https://github.com/cBournhonesque/lightyear/commit/4103090a52afc050c1de6dcbd7eb278af5ffe94a))
    - Recv impl ([`2293aed`](https://github.com/cBournhonesque/lightyear/commit/2293aed17794c1a2017a78145258bdf9a84bca6a))
    - Fix typo in docs ([`307f947`](https://github.com/cBournhonesque/lightyear/commit/307f947d03668b272beab1a4285dc9cc3463b867))
    - Working ([`ccba5f9`](https://github.com/cBournhonesque/lightyear/commit/ccba5f9aff64c7dc1d08f01142c6d22ca71b1df2))
    - Add non-working native ws impl ([`7781029`](https://github.com/cBournhonesque/lightyear/commit/7781029b8c17f1558400b7095cedeb85c6f269c9))
    - Fix tests and CI ([`bd4eb23`](https://github.com/cBournhonesque/lightyear/commit/bd4eb23ed3f8fe1dd71971f8f0403183b4aff6e0))
    - Merge branch 'main' into cb/priority ([`b463b41`](https://github.com/cBournhonesque/lightyear/commit/b463b41b52cd09648a9017b11ddae18ad610852c))
    - Merge pull request #122 from cBournhonesque/cb/debug-bullet-prespawn ([`51d8125`](https://github.com/cBournhonesque/lightyear/commit/51d8125779e014eb4c45e12447de2a4d5007b172))
    - Clean logs ([`82d6c4d`](https://github.com/cBournhonesque/lightyear/commit/82d6c4d7845083b55292bf13aeef2c3d585934da))
    - Fix ([`daf0c04`](https://github.com/cBournhonesque/lightyear/commit/daf0c04627c4edbdd465a8c8cfe78c4162fb5671))
    - Merge pull request #120 from cBournhonesque/cb/0.8.1 ([`effd73f`](https://github.com/cBournhonesque/lightyear/commit/effd73fb4fc3589cf398fd35a18c07bc356881a0))
    - Upgrade to 0.8.1 ([`904e778`](https://github.com/cBournhonesque/lightyear/commit/904e778329ee0da3ad7a6a44f9480e30be93ce1b))
    - Do not panic when entity map fails ([`4aba3ff`](https://github.com/cBournhonesque/lightyear/commit/4aba3ff709874e8dc1d6049b7db30b9ab134b3c0))
    - Check for timeouts before receiving packets so we can timeout even if the connection is faulty ([`128a568`](https://github.com/cBournhonesque/lightyear/commit/128a56842a505e8a0638e701be6f20224b5cb404))
    - Reduce default client timeout to 3 seconds ([`4473519`](https://github.com/cBournhonesque/lightyear/commit/447351976d421188225a368c2a78f25cdfd12fc5))
    - Spawn backend rivet task as part of server ([`6f07d76`](https://github.com/cBournhonesque/lightyear/commit/6f07d765fbb0f3e3b85ea4173f2bdced22445eb7))
    - Fix error ([`98e14ca`](https://github.com/cBournhonesque/lightyear/commit/98e14caa152091b188bdf624bc65777cd693da43))
    - Improving dockerfile and adding Connection option in simple_box example to choose how to create the connection ([`890826b`](https://github.com/cBournhonesque/lightyear/commit/890826b3d88d8642050177559310dfb70ecf10c7))
    - Merge branch 'main' into cb/rivet ([`921e5fa`](https://github.com/cBournhonesque/lightyear/commit/921e5fa7db598d02b80330ca5b771845b9082005))
    - Merge pull request #113 from cBournhonesque/cb/fix-interpolation ([`f8746fd`](https://github.com/cBournhonesque/lightyear/commit/f8746fd8d4e7d64965a02ff608426a7cacf86fbc))
    - Separate interpolation systems that insert the component vs update the component ([`ed9f321`](https://github.com/cBournhonesque/lightyear/commit/ed9f3217249c47054ccbef2c514ac07e36fe393c))
    - Merge pull request #111 from cBournhonesque/cb/fix-webtransport-server ([`6ef1710`](https://github.com/cBournhonesque/lightyear/commit/6ef171097b7e0544811908b20c47b51066042ff2))
    - Remove interp logs ([`ab1d78f`](https://github.com/cBournhonesque/lightyear/commit/ab1d78f3b62fa62e8084b9da962fc534b54611c7))
    - Add todo for wt serverR ([`e6cac03`](https://github.com/cBournhonesque/lightyear/commit/e6cac037dfa54213f70ed98ebc0d411ba11a0efc))
    - Remove tokio::select from WT server, and cancel the tasks if the quic connection dies ([`9b69b40`](https://github.com/cBournhonesque/lightyear/commit/9b69b401cd183c6c538f9f0e964eb1b6043b0e21))
    - Merge pull request #110 from cBournhonesque/cb/fix-replication-groups ([`7f2d735`](https://github.com/cBournhonesque/lightyear/commit/7f2d735dac067947b31ecb7642bdc89841f8a564))
    - Map entities for updates ([`4a77b6e`](https://github.com/cBournhonesque/lightyear/commit/4a77b6ee1bd94ecbd6654bcbb91cc8c6f9cf11c7))
    - Merge pull request #109 from cBournhonesque/cb/fix-examples ([`b65c0ff`](https://github.com/cBournhonesque/lightyear/commit/b65c0ffa3228dc44d470d350a78f8ef81150d944))
    - Fix examples ([`a73ed64`](https://github.com/cBournhonesque/lightyear/commit/a73ed646efbcd1c3a4aa95e26db0bdc7c954173d))
    - Fix rivet client call ([`aad25e0`](https://github.com/cBournhonesque/lightyear/commit/aad25e00149409f3b66dac81aa95ffa91985ef29))
    - Fix all CI ([`e6c774e`](https://github.com/cBournhonesque/lightyear/commit/e6c774e6d50ab7e5ccaf461ad79cb0e0a09f9b3f))
    - Improving tests ([`140e95b`](https://github.com/cBournhonesque/lightyear/commit/140e95b3d43981c9efda233ce0347ee93c525704))
    - Tests work apart from diagnostics ([`822f76b`](https://github.com/cBournhonesque/lightyear/commit/822f76b734fb3a0f05932c9d9c81bdda2adf7e7b))
    - Introduce the ClientConnection anad ServerConnection conneciton wrappers ([`f011445`](https://github.com/cBournhonesque/lightyear/commit/f011445fa4c3f7a7f75ca1996044018f36c234df))
    - Wip ([`03d0d85`](https://github.com/cBournhonesque/lightyear/commit/03d0d850ba17dd6b5780078f13580d00b3255349))
    - Start adding rivet support ([`08c3b02`](https://github.com/cBournhonesque/lightyear/commit/08c3b02149871f08bfbd35558e292a5ec94eabc3))
    - Update cargo.toml ([`010e225`](https://github.com/cBournhonesque/lightyear/commit/010e225dcb769164c8b7642cd69b65c4bc9271d9))
    - Merge pull request #107 from cBournhonesque/cb/release-0.8.0 ([`79399ed`](https://github.com/cBournhonesque/lightyear/commit/79399ed30e82ff65c9ee78519dfc3a36638910b6))
    - New release ([`57b7708`](https://github.com/cBournhonesque/lightyear/commit/57b770817ec93539050fb8340a5dc2c4a861f2ea))
    - Remove info! logs ([`4f0f81c`](https://github.com/cBournhonesque/lightyear/commit/4f0f81caf06cc3df0fae858eef4755bf38169f4d))
    - Fix test ([`1b4f859`](https://github.com/cBournhonesque/lightyear/commit/1b4f85980a38730554a3a4821b04df320b133b54))
    - Clean up the example ([`0c27e30`](https://github.com/cBournhonesque/lightyear/commit/0c27e302a91c3fcd165fc3cedb2c6baf54e28676))
    - Trying xxhash ([`b2dbe9f`](https://github.com/cBournhonesque/lightyear/commit/b2dbe9f390a134f0cccbf643ce8f6a3635debfbe))
    - Log hash for prespawn ([`1d48513`](https://github.com/cBournhonesque/lightyear/commit/1d485130cd5d6496ee9c262cef951dd0cff0c759))
    - Stop using the hash of TypeId for prespawning ([`9a746f2`](https://github.com/cBournhonesque/lightyear/commit/9a746f2a229ae49adeb5d3d194dc5f7560ac6d7f))
    - Merge pull request #16 from cBournhonesque/cb/wasm-support ([`258a673`](https://github.com/cBournhonesque/lightyear/commit/258a673223d594a0d9a86470c8eb5fed99a5b157))
    - Fix gh actions ([`eb2285a`](https://github.com/cBournhonesque/lightyear/commit/eb2285a86b980aa74383a8d8fe06f58e6a7bbd12))
    - Replace all std::time ([`3409c1e`](https://github.com/cBournhonesque/lightyear/commit/3409c1efb926311bb4d0e0ce7c551d4f034a2653))
    - Working wasm, but with mispredictions ([`8df1bcb`](https://github.com/cBournhonesque/lightyear/commit/8df1bcbb61531bf8f1dbd4895800efe13bda71ff))
    - Improve exampel ([`d1c2e5c`](https://github.com/cBournhonesque/lightyear/commit/d1c2e5c8e7703cb398da52fd0293ea653ddf2dde))
    - Add book page about priority ([`bfad77d`](https://github.com/cBournhonesque/lightyear/commit/bfad77df293d382af00a3d4cf5ac69532cf43630))
    - Adding priority example ([`703bc13`](https://github.com/cBournhonesque/lightyear/commit/703bc1339907737dc07c1d954b1bae9a5ef41f56))
    - Merge branch 'main' into cb/wasm-support ([`6987be4`](https://github.com/cBournhonesque/lightyear/commit/6987be4078410f396e92c504c18659331732a84b))
    - Update more std::time to bevy::utils::time ([`43cf9c7`](https://github.com/cBournhonesque/lightyear/commit/43cf9c757ebc3f97bde63640e7262a4a5ce51534))
    - Set the basis for priority accumulation for replication send ([`8c12c78`](https://github.com/cBournhonesque/lightyear/commit/8c12c78a6bfed6393f65b10fb06c5c01665c8e51))
    - Wip ([`ad7fff9`](https://github.com/cBournhonesque/lightyear/commit/ad7fff9b8120fbaf587e45a290a1eef31254a63d))
    - Merge pull request #102 from Ant59/main ([`4dbd2b4`](https://github.com/cBournhonesque/lightyear/commit/4dbd2b4c19c611c759dd7184e16898e984c54f9a))
    - Merge pull request #104 from cBournhonesque/cb/fix-bench ([`6ac6310`](https://github.com/cBournhonesque/lightyear/commit/6ac6310bde8e3a9f30d0d4a963d6d4395947bf9d))
    - Update benchmark with the new resources ([`374bd60`](https://github.com/cBournhonesque/lightyear/commit/374bd609a526b5a44c7197b8e0578b785d2280c8))
    - Wip ([`dcfd4b8`](https://github.com/cBournhonesque/lightyear/commit/dcfd4b8ab116bf2f486e8d9995d1135dfd995c2f))
    - Allow WebTransport clients to gracefully disconnect ([`d77966f`](https://github.com/cBournhonesque/lightyear/commit/d77966f0ace3e5dc039db376d00bf1230500a604))
    - Set original tick in the single-data for when the message was created. Quota kind of works, but the end result is pretty bad; probably because they are all part of the same rpelicatio ngroup? ([`001ed4d`](https://github.com/cBournhonesque/lightyear/commit/001ed4ddcc3789f17e8d7cee3f92f0b5f042f334))
    - Kind of works? but we get a panic with packet too big ([`eac1a39`](https://github.com/cBournhonesque/lightyear/commit/eac1a39682108d71f6022c552c930445f8953661))
    - Wip priotity ([`31af428`](https://github.com/cBournhonesque/lightyear/commit/31af428431011bbf09cea6283cf0364aa1b3d370))
    - Merge pull request #91 from cBournhonesque/cb/object-pre-spawning ([`7ca8ba0`](https://github.com/cBournhonesque/lightyear/commit/7ca8ba0a50be6189ddb5fa50ee718c72db91a3d6))
    - Add missing files ([`3ca9a5b`](https://github.com/cBournhonesque/lightyear/commit/3ca9a5b87b3d2f34405b3d190f909a97654dc99f))
    - Improving interpolation behaviour for low send interval: only spawn interpolate components once there is 2 server updates ([`52a9e75`](https://github.com/cBournhonesque/lightyear/commit/52a9e75ba6a70f5fbe0620d99a091e1be6194dc2))
    - Merge pull request #96 from cBournhonesque/cb/despawn-recursive ([`d3e8109`](https://github.com/cBournhonesque/lightyear/commit/d3e81091ec39fc06aa5fe7b158b7fcc3fda461bc))
    - Add unit tests ([`4adee75`](https://github.com/cBournhonesque/lightyear/commit/4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7))
    - Kind of working version? just some caveats (run at FixedUpdate, and remember to use Pressed+Consume) ([`15ee59a`](https://github.com/cBournhonesque/lightyear/commit/15ee59a5d13bca17e8395414defac4cde1d1bffe))
    - Add rollback for prespawned entities ([`01535ff`](https://github.com/cBournhonesque/lightyear/commit/01535ff456bfcb80b0e82c6b7244f027455cf789))
    - Add despawn recursive ([`0206f5f`](https://github.com/cBournhonesque/lightyear/commit/0206f5f8afb069848b5535bb5071f50971d674c8))
    - Add files ([`8cbb26b`](https://github.com/cBournhonesque/lightyear/commit/8cbb26bb6a8abaebbc74c8f4d02f801558183f39))
    - Merge branch 'main' into cb/object-pre-spawning ([`12853d6`](https://github.com/cBournhonesque/lightyear/commit/12853d6f365d7895857e91f2a184da37bf1e20ea))
    - Wip ([`cbbe097`](https://github.com/cBournhonesque/lightyear/commit/cbbe097a96ab084cb1eb2d8b68691249963cb8f3))
    - Wip prototype ([`8b6b75b`](https://github.com/cBournhonesque/lightyear/commit/8b6b75bd5447d651d7fef0541346ea25a1ccacd2))
    - Add server leafwing input manage to handle just-pressed correctly ([`c5654c8`](https://github.com/cBournhonesque/lightyear/commit/c5654c85b4c71ba994407ea5713fa81871cb18ce))
    - Add server leafwing input manage to handle just-pressed correctly ([`cd93302`](https://github.com/cBournhonesque/lightyear/commit/cd9330295fe7bba2cd71e5b76afbe8d966994c1e))
    - Release 0.7.0 ([`7a55d38`](https://github.com/cBournhonesque/lightyear/commit/7a55d38f8f952a382e7bb61db9611fd295279f3e))
    - Merge pull request #94 from cBournhonesque/cb/sync-test ([`71f7551`](https://github.com/cBournhonesque/lightyear/commit/71f7551b6126859eb0236dd819728e4d06382bc4))
    - Update gitignore and clean PR ([`5d883e5`](https://github.com/cBournhonesque/lightyear/commit/5d883e5faf8635a0dad65d5de78d8bd3bcfcaf8f))
    - Create TickEvent to update the input-buffer ticks upon tick snap ([`2696227`](https://github.com/cBournhonesque/lightyear/commit/2696227f47af6f4a03d45d7b4cb9e68f358c2b52))
    - Add shooter example ([`7fd200e`](https://github.com/cBournhonesque/lightyear/commit/7fd200e50224e8e28a6bc73059cc5d1c55b248f7))
    - Merge pull request #90 from cBournhonesque/cb/debug-sync ([`bf2000b`](https://github.com/cBournhonesque/lightyear/commit/bf2000b9cf0367922e87a6419fb27897d8ea4fe3))
    - Clean PR ([`850c87c`](https://github.com/cBournhonesque/lightyear/commit/850c87cdb8aa59dba8964a11ea743e0f19023fa8))
    - Fix replication not working after tick wrap-around ([`5c0e456`](https://github.com/cBournhonesque/lightyear/commit/5c0e456b2b7aefa00bfe89709881cbd5e61bbed4))
    - Merge pull request #86 from cBournhonesque/cb/sync-bug ([`10a6687`](https://github.com/cBournhonesque/lightyear/commit/10a66873977a3f2dd99b0f228ac756b7a68afa35))
    - Fix ([`c7acf3d`](https://github.com/cBournhonesque/lightyear/commit/c7acf3dac5863a4f05ddbcecdb0fc3f743c4e738))
    - Merge pull request #80 from cBournhonesque/cb/debug-sync-wraparound ([`0ff9a5e`](https://github.com/cBournhonesque/lightyear/commit/0ff9a5ee3d361b11ee5cc66b245bc446c60e31b5))
    - Clean PR ([`0ff8632`](https://github.com/cBournhonesque/lightyear/commit/0ff863269e318c2511d7419536b038e718da2f24))
    - Clean logs ([`02096e0`](https://github.com/cBournhonesque/lightyear/commit/02096e0781d7cd2f9a99d1eec033ac4fc4a4adf1))
    - Starting to fix sync ([`616f061`](https://github.com/cBournhonesque/lightyear/commit/616f0617893fb77141b2361664a0d6f0ba50bec6))
    - Merge pull request #79 from cBournhonesque/cb/decouple ([`2f58023`](https://github.com/cBournhonesque/lightyear/commit/2f5802338941d21ebe1f276348fb06aedc8d6cc4))
    - Rename ConnectionManager ([`3e0b851`](https://github.com/cBournhonesque/lightyear/commit/3e0b85148c908b48a723c5ba3cb1188d748bed08))
    - Merge pull request #78 from cBournhonesque/cb/decouple ([`6aa68ea`](https://github.com/cBournhonesque/lightyear/commit/6aa68ea2f5439b7e6efacd793932c4f3b6c6c844))
    - Lints ([`f67cda9`](https://github.com/cBournhonesque/lightyear/commit/f67cda9076ca58451b08fa17ae7b00dc49da5ae9))
    - Fix all examples ([`1189703`](https://github.com/cBournhonesque/lightyear/commit/1189703642f5c14e7501e53221e5a0320bc7e993))
    - Tests pass ([`7958d6b`](https://github.com/cBournhonesque/lightyear/commit/7958d6b95de9479dacd28f61c3324785aa8db35e))
    - Fix client/server receive systems ([`c6f5e6e`](https://github.com/cBournhonesque/lightyear/commit/c6f5e6ec7a272a5df4ad4fabbc6496757a858849))
    - Wip decouple the different resources ([`3bc2232`](https://github.com/cBournhonesque/lightyear/commit/3bc22327c4312354089bf2ab27c95f99be92e110))
    - Merge branch 'main' into cb/wasm-support ([`94de05a`](https://github.com/cBournhonesque/lightyear/commit/94de05a0379f53ba0d344f17599deb598a912856))
    - Merge pull request #75 from cBournhonesque/cb/better-input-handling ([`ef2bb3c`](https://github.com/cBournhonesque/lightyear/commit/ef2bb3c67d47d6185b272946f13da0e5c9039cd2))
    - Lint ([`3e996aa`](https://github.com/cBournhonesque/lightyear/commit/3e996aafb81a5ed6fe553b5e83ccdc2c501c9781))
    - Replicate inputs per entity correctly for all 3 situations ([`19765e1`](https://github.com/cBournhonesque/lightyear/commit/19765e1d7e129bb6232c5dea36953d0583734feb))
    - Improve input entity mapping. For some reason it does not work if the InputBuffer is only on a predicted entity ([`066307f`](https://github.com/cBournhonesque/lightyear/commit/066307fd4b7d18e15bb11501921b0f3f849cad5c))
    - Debug ([`26efcb8`](https://github.com/cBournhonesque/lightyear/commit/26efcb8810d25cf8bf263de3c248bd1d610dbeeb))
    - Get example working ([`0e4b1b0`](https://github.com/cBournhonesque/lightyear/commit/0e4b1b0478a8d3738c8f2d6924aecdfdbef2db7c))
    - Fix build ([`15902cf`](https://github.com/cBournhonesque/lightyear/commit/15902cf38035318c718e61decb81a202d99cb60f))
    - Merge branch 'main' into cb/wasm-support ([`40a3fd3`](https://github.com/cBournhonesque/lightyear/commit/40a3fd3fc6f6ece3a40908843cae94c53da6d636))
    - Merge pull request #74 from cBournhonesque/cb/correction-easing ([`f4342a2`](https://github.com/cBournhonesque/lightyear/commit/f4342a29ffd70e5591b9cd912a16e767ec651688))
    - Add easing for correction, add diagnostics on client ([`ba67c37`](https://github.com/cBournhonesque/lightyear/commit/ba67c3760dd3cbe0f5841fc7c15c073f90507ac2))
    - Merge pull request #72 from cBournhonesque/cb/upgrade-xpbd ([`b904339`](https://github.com/cBournhonesque/lightyear/commit/b904339e358fe328490979e514b85ce7f4781a8a))
    - Update bevy xpbd to 0.3.3 ([`5224d82`](https://github.com/cBournhonesque/lightyear/commit/5224d82cb6f6d60b3c3548e6eee9e61166454d5b))
    - Merge pull request #70 from cBournhonesque/cb/clean-log-example ([`95e7361`](https://github.com/cBournhonesque/lightyear/commit/95e7361891909cf7d871ff9ab6ae9edccdbc4dd9))
    - Remove logs, fix time-wrapped overflow ([`c91998c`](https://github.com/cBournhonesque/lightyear/commit/c91998c4d1e67ec80c9b34cc0528ca0d024ec8dc))
    - Merge pull request #69 from cBournhonesque/cb/fix ([`2e8780c`](https://github.com/cBournhonesque/lightyear/commit/2e8780c7425aa5954997140d5f76f390244cd7ce))
    - Fix examples ([`b27dc76`](https://github.com/cBournhonesque/lightyear/commit/b27dc76072f62055860e087bf167ab1ae74c70fc))
    - Merge pull request #67 from cBournhonesque/cb/release-0.6.0 ([`39bf016`](https://github.com/cBournhonesque/lightyear/commit/39bf0167c24006ef675312184bf97077aba70f45))
    - Fixes ([`d390e0d`](https://github.com/cBournhonesque/lightyear/commit/d390e0d12af8771fe2de1e7f515f5921329291e4))
    - Release 0.6.0 ([`c4fefa6`](https://github.com/cBournhonesque/lightyear/commit/c4fefa6575c486ecb0072646366846bc476c67bc))
    - Merge pull request #55 from cBournhonesque/cb/add-xpbd-to-example ([`027ccdd`](https://github.com/cBournhonesque/lightyear/commit/027ccddc8fd91aab74a0ff2d287d926bf2f2aaeb))
    - Merge branch 'main' into cb/add-xpbd-to-example ([`9938e75`](https://github.com/cBournhonesque/lightyear/commit/9938e7597a7fddc65e964e5424b64212adac22ec))
    - Optimize imports / tests / lints ([`a7d3ae7`](https://github.com/cBournhonesque/lightyear/commit/a7d3ae7703106f0a6fade32189c9f883632bdd1e))
    - Rework Message to be automatically implemented. rework Named ([`8e09548`](https://github.com/cBournhonesque/lightyear/commit/8e09548bed850abad52c32c30aa37d6a63ef3d3d))
    - Use previous visual correction as start of new correction ([`dbce6c0`](https://github.com/cBournhonesque/lightyear/commit/dbce6c0c39fcd5a98633cadd7adcafca661c415b))
    - Simulation seems to be working fairly well ([`045c1e2`](https://github.com/cBournhonesque/lightyear/commit/045c1e237676dc39ba6f857e572347d328befba3))
    - Fix a bug in replication where we were not computing the acked bevy-tick correctly ([`7b6b530`](https://github.com/cBournhonesque/lightyear/commit/7b6b5303c9cae5b03d660559019010d52d4b03e9))
    - Merge pull request #64 from cBournhonesque/dependabot/cargo/metrics-exporter-prometheus-0.13.0 ([`9c93d16`](https://github.com/cBournhonesque/lightyear/commit/9c93d16dbec5bff4289bdb436a73ce01c3bd6962))
    - Reworked sync-component/interpolation so that the components don't need to implement a trait (which is not possible for external types because of orphan rule). Long-term use reflection. Also introduce the concept of Correction to correct the value of a predicted component ([`fdb4ff6`](https://github.com/cBournhonesque/lightyear/commit/fdb4ff67ed6d4aa67860ce0bddc651975af7efa2))
    - Update metrics-exporter-prometheus requirement from 0.12.1 to 0.13.0 ([`a163a65`](https://github.com/cBournhonesque/lightyear/commit/a163a6566cc570ea15f742699176ba0f975327c6))
    - Merge pull request #62 from cBournhonesque/dependabot/cargo/metrics-0.22 ([`53e0685`](https://github.com/cBournhonesque/lightyear/commit/53e0685e1942ea447129c35f11a1368cfd29dab2))
    - Merge pull request #65 from cBournhonesque/dependabot/cargo/metrics-tracing-context-0.15 ([`f2a873b`](https://github.com/cBournhonesque/lightyear/commit/f2a873bf9547209c2ceb30a07c57ec41e029f359))
    - Update metrics-tracing-context requirement from 0.14 to 0.15 ([`79f8b01`](https://github.com/cBournhonesque/lightyear/commit/79f8b01716a258afdf448229a18daf2860f4ce95))
    - Update metrics requirement from 0.21 to 0.22 ([`75db090`](https://github.com/cBournhonesque/lightyear/commit/75db0906a256eb1840beb07abee8098f612d5a1c))
    - Add confirmed tick ([`60f0170`](https://github.com/cBournhonesque/lightyear/commit/60f017078ecff21ab7fe7613e27ce96863d3cc78))
    - Apart from initial sync errors, the simulation is perfectly smooth with other clients if we put enough input delay ([`d3060bf`](https://github.com/cBournhonesque/lightyear/commit/d3060bfcbe894b18c441c3e656a872a5a362ed55))
    - Working delayed inputs ([`5aa6869`](https://github.com/cBournhonesque/lightyear/commit/5aa68693186b5cfaaa2704940cd13caa4ef358d8))
    - Merge branch 'cb/delayed-inputs' into cb/add-xpbd-to-example ([`f4310aa`](https://github.com/cBournhonesque/lightyear/commit/f4310aa58d53dd3ec36eb3b84138ee44bf5b70fc))
    - Wip ([`17d05eb`](https://github.com/cBournhonesque/lightyear/commit/17d05ebe6a15ea6460a1344359b56b8e28dddbe8))
    - Wip ([`1a736ac`](https://github.com/cBournhonesque/lightyear/commit/1a736ac183d0796a55589d5c7b730ed250e68d24))
    - Simplify how to do pre-prediction ([`562b5c2`](https://github.com/cBournhonesque/lightyear/commit/562b5c2352aff14fe000488e60071256914a125e))
    - Enable updating the replication behaviour per component ([`42ea7da`](https://github.com/cBournhonesque/lightyear/commit/42ea7daa3b48b963ba1a102224db7dd810d0b66d))
    - Fix bug by adding both predicted entities to the same replication group ([`7e4184c`](https://github.com/cBournhonesque/lightyear/commit/7e4184c2ccfd5ebf381466a8cc9d8332f35a650d))
    - Improve ([`8eb41f2`](https://github.com/cBournhonesque/lightyear/commit/8eb41f23977e841ab60e070d18ee90bcdfa331b1))
    - Improve example ([`572ded7`](https://github.com/cBournhonesque/lightyear/commit/572ded74148b73f586f4530e0d50bf615d90996f))
    - Improve xpbd example ([`73cce5c`](https://github.com/cBournhonesque/lightyear/commit/73cce5cb3b32c558eabbd7644da802e3ebb64d45))
    - Merge pull request #50 from cBournhonesque/cb/just-pressed ([`e467f0b`](https://github.com/cBournhonesque/lightyear/commit/e467f0bbda670488cc93fe0db8d09a7bfb8f12bc))
    - Lint ([`0294abc`](https://github.com/cBournhonesque/lightyear/commit/0294abc8778eca37a3f05a8a8f2af48c67dbd791))
    - Merge branch 'main' into cb/just-pressed ([`aa84e32`](https://github.com/cBournhonesque/lightyear/commit/aa84e329ebd951e798787a17ebeec6cd07d8f535))
    - Update protocol macro + lints ([`79102da`](https://github.com/cBournhonesque/lightyear/commit/79102dad9df00a44d9f3064d81197e2fe60e778a))
    - Re-organize examples as separate crates with their own deps ([`96af142`](https://github.com/cBournhonesque/lightyear/commit/96af142df926317fbfe71d23a59a005bfd16eab3))
    - Example seems fully working! Now need to polish PR (make sure it works without prediction as well)? ([`a2eb025`](https://github.com/cBournhonesque/lightyear/commit/a2eb02560a53d64b7773854fac85b2f8cc5aab48))
    - Wip ([`989f10f`](https://github.com/cBournhonesque/lightyear/commit/989f10f6e85ada586470160400f6a12df30cc19b))
    - Merge pull request #54 from cBournhonesque/cb/fix-newly-connected-replication ([`b16ae27`](https://github.com/cBournhonesque/lightyear/commit/b16ae2702287ad56de27e188b8f3bb36241ec11a))
    - When a client newly connects, replicate existing entities to them only if they were in the replication-target ([`00aa5bc`](https://github.com/cBournhonesque/lightyear/commit/00aa5bc74ff4855cbd75424e8150535af5c914a2))
    - Wip ([`6000a42`](https://github.com/cBournhonesque/lightyear/commit/6000a42297ba5c653ff50692c353445c44e7d557))
    - Add leafwing input test ([`8629853`](https://github.com/cBournhonesque/lightyear/commit/86298534471bc66bc43e70ab5e2e05923ce5ba96))
    - Starting to add test ([`3676825`](https://github.com/cBournhonesque/lightyear/commit/3676825185c7d52e22915a7b17cf1aba9d0b0bba))
    - Add replicate_once for ActionState ([`57130b5`](https://github.com/cBournhonesque/lightyear/commit/57130b5ef1ed5543d455f443380dc6f10118a5ef))
    - Remove IntoKind for FromType, added ActionType to the ComponentProtocol ([`cae48f4`](https://github.com/cBournhonesque/lightyear/commit/cae48f420018eb72a4c45f1f29f9dc07ad834db7))
    - Example working ([`094982c`](https://github.com/cBournhonesque/lightyear/commit/094982c68f77c899efef6e6e74a9a3afecd57f50))
    - Passing leafwing tests ([`c7cbf5f`](https://github.com/cBournhonesque/lightyear/commit/c7cbf5f67a6e072e768c4c49c626869b3d3e8f76))
    - Wip leafwing ([`00de75d`](https://github.com/cBournhonesque/lightyear/commit/00de75d02f5520eb09d323f7796d21ce6053d890))
    - Merge pull request #48 from cBournhonesque/cb/update-transport ([`a2fdbdd`](https://github.com/cBournhonesque/lightyear/commit/a2fdbddf9e126900e145254f306c3b56e79890f9))
    - Nit ([`1cafe78`](https://github.com/cBournhonesque/lightyear/commit/1cafe7812184c5568801dbb6c20d5cc3fb615c46))
    - Add mpsc transport and make benchmark use them ([`c02cde8`](https://github.com/cBournhonesque/lightyear/commit/c02cde87d8fb8f9fec5bf9162cc32d025162918e))
    - New transport looks good, but somehow the benchmark fails ([`d4eae6b`](https://github.com/cBournhonesque/lightyear/commit/d4eae6b44177a25cce292456267da97588d04047))
    - Improve IoConfig by removing references, add Channels transport for server ([`a8c716f`](https://github.com/cBournhonesque/lightyear/commit/a8c716fa035b31ffa74b4dfb06758121ea058507))
    - Merge pull request #44 from cBournhonesque/cb/sync-bug ([`f42caa3`](https://github.com/cBournhonesque/lightyear/commit/f42caa32471d5c026259068a4c3270c9ae331030))
    - Fix unit tests ([`40f5244`](https://github.com/cBournhonesque/lightyear/commit/40f524444cc2e5dfe0aa4441bd42fc0a5de5f875))
    - Fix sync issues ([`8bf5e28`](https://github.com/cBournhonesque/lightyear/commit/8bf5e28fa9c7da6f599088324175814096e4790a))
    - Merge pull request #39 from cBournhonesque/cb/ignore-component ([`049f03e`](https://github.com/cBournhonesque/lightyear/commit/049f03e61e89e1a9f05742c632e41f8a66b0d7be))
    - Fix doc ([`b155a58`](https://github.com/cBournhonesque/lightyear/commit/b155a58fe7f3337f694ae052f4c25cfed3c9ac4f))
    - Reexport Client<MyProtocol> etc. and the option to not replicate some components for an individual entity ([`d74185c`](https://github.com/cBournhonesque/lightyear/commit/d74185cf486efe2d679ec8f37980e810d667e372))
    - Merge pull request #35 from cBournhonesque/cb/fix-compile ([`d6b7d64`](https://github.com/cBournhonesque/lightyear/commit/d6b7d64ee30a82df22e46abaf3bbb8b88e144962))
    - Fix running ([`90bebdf`](https://github.com/cBournhonesque/lightyear/commit/90bebdfa8cc89fb3c861abc3333e6dc0cfaccd78))
    - Merge pull request #29 from cBournhonesque/cb/server-deploy ([`cfcdff9`](https://github.com/cBournhonesque/lightyear/commit/cfcdff97b2254b3185c468d2f17f48a0f16c9ae3))
    - Remove pre-prediction warning logs ([`48d62e8`](https://github.com/cBournhonesque/lightyear/commit/48d62e8d9fb7c4cfbcea39b5224deb939322c4b7))
    - Convert all examples to be compatible with a headless remote server ([`0e77483`](https://github.com/cBournhonesque/lightyear/commit/0e77483b0a687e31639f737558d57218dad98ecd))
    - Wip ([`92c5b27`](https://github.com/cBournhonesque/lightyear/commit/92c5b27c3358134a4c0ca2e1501ab749db96b486))
    - Merge pull request #28 from cBournhonesque/cb/replication-recv-improvements ([`796f011`](https://github.com/cBournhonesque/lightyear/commit/796f011f3386d39a321a1e914a6f6eb1b2b59200))
    - Apply all changes ([`0ed6bdd`](https://github.com/cBournhonesque/lightyear/commit/0ed6bddf7809c753e99597241a150390e164ee9b))
    - Merge pull request #26 from cBournhonesque/cb/benchmark ([`3b63bce`](https://github.com/cBournhonesque/lightyear/commit/3b63bce85bd176b295342586dc2bc574b489de8e))
    - Add benchmark ([`19ccf8a`](https://github.com/cBournhonesque/lightyear/commit/19ccf8a436b667a7296e00f70bcf6d67383dde33))
    - Merge pull request #25 from cBournhonesque/cb/prepare-0.5.0 ([`b9e7bed`](https://github.com/cBournhonesque/lightyear/commit/b9e7bed3c75838b2cfe50ae9b992dc973ed40946))
    - Release 0.5.0 ([`5b5197a`](https://github.com/cBournhonesque/lightyear/commit/5b5197aa9322d37c075629d9a1db2ecdb978f70d))
    - Merge pull request #24 from cBournhonesque/cb/improve-book ([`e03be0c`](https://github.com/cBournhonesque/lightyear/commit/e03be0cc518109a41a00164dbab1ca6c183817e7))
    - Fix the bug with mistakenly sending pre-predicted entities to other clients ([`0a147e3`](https://github.com/cBournhonesque/lightyear/commit/0a147e322ed03f97f430df07815ca7e9fb70dfd1))
    - All bugs but one fixed ([`e8f405c`](https://github.com/cBournhonesque/lightyear/commit/e8f405c0dba11ce67a1537522c3ecd2bd6569943))
    - Almost everything working ([`ec447e2`](https://github.com/cBournhonesque/lightyear/commit/ec447e2494b50606a4535fd5eb15010659eeca61))
    - Merge pull request #23 from cBournhonesque/cb/client-replication ([`7c638e4`](https://github.com/cBournhonesque/lightyear/commit/7c638e45ee39a8eebea8c8ae0be3baf2eb21e5ee))
    - Fix docs ([`aa85dcc`](https://github.com/cBournhonesque/lightyear/commit/aa85dcc83cc4be583a68edc34fa99b2163c65bf5))
    - Fix lint ([`28b462a`](https://github.com/cBournhonesque/lightyear/commit/28b462a827e60d57b7c384185c4f2efbf3df8060))
    - Client-replication version A and B working in simple-case. Think about entity spawn/despawn. Component insert/remove during prediction ([`c0e7c1b`](https://github.com/cBournhonesque/lightyear/commit/c0e7c1bc6d075c340c5ef1ecbad5ae690b4fcb73))
    - Fix server events ([`f71b6b3`](https://github.com/cBournhonesque/lightyear/commit/f71b6b3cb24613aff72fc6cb9736ad7c007f765a))
    - Create a ConnectionManager for server ([`7323c2d`](https://github.com/cBournhonesque/lightyear/commit/7323c2d70da8b7769be5bd9f3e763f0947fd3ef0))
    - Split replication sender/receiver. Split protocolMessage between ClientMessage and ServerMessage ([`26a0640`](https://github.com/cBournhonesque/lightyear/commit/26a06407e3ddd2f36ef1310eb4ee94b36c04cd3f))
    - Merge pull request #21 from cBournhonesque/cb/sync-groups ([`84a91dc`](https://github.com/cBournhonesque/lightyear/commit/84a91dc02cfd39ec5f579887253b416eb20da2d4))
    - Merge pull request #20 from cBournhonesque/cb/sync-groups ([`e1f3b5b`](https://github.com/cBournhonesque/lightyear/commit/e1f3b5bb48385f3fa562a28a77f1f870562b0ffc))
    - Update versions ([`c3b2d5d`](https://github.com/cBournhonesque/lightyear/commit/c3b2d5da33ec2902259ad2eae8d018c09cbce2aa))
    - Fix ([`83adcad`](https://github.com/cBournhonesque/lightyear/commit/83adcadb66e05c867ae243ce8f9dcb41284e4811))
    - Merge pull request #19 from cBournhonesque/cb/fix-relations ([`06bd9bf`](https://github.com/cBournhonesque/lightyear/commit/06bd9bf3a7e386ca492b6ad6fb9b6f64e003bb6f))
    - Lint ([`291a920`](https://github.com/cBournhonesque/lightyear/commit/291a920129ee4e00e9103ef5b172e185b79f5215))
    - Example is smooth with 400ms ping, 80ms jitter, 10% packet loss ([`a43c566`](https://github.com/cBournhonesque/lightyear/commit/a43c566bbbb29a3055337c51689f0987fa6ecbeb))
    - Both prediction and interpolation are working pretty well ([`c278570`](https://github.com/cBournhonesque/lightyear/commit/c278570880d4326501fbc4573f24737aa9b565c9))
    - Improve interpolation for replication example; fix prediction bug by keeping around inputs that are needed for prediction ([`ec5fc59`](https://github.com/cBournhonesque/lightyear/commit/ec5fc59c048bc82e649949a363272c7503421242))
    - Improve interpolation ([`c3bde49`](https://github.com/cBournhonesque/lightyear/commit/c3bde4922681c9a66c0f6eaf47f6ffcaadd6bd49))
    - Interpolation works wellgau! maybe some slight problem in the interpolation logic; some tearing sometimes ([`b394ac1`](https://github.com/cBournhonesque/lightyear/commit/b394ac105bf5a39c713e718c464430286fbebd72))
    - Make interpolation more flexible ([`86c8382`](https://github.com/cBournhonesque/lightyear/commit/86c8382040dafc7960f2fd04569410a6ee38bd74))
    - In general, it seems to be working, but problems with prediction ([`b057ded`](https://github.com/cBournhonesque/lightyear/commit/b057ded292251f509e1aebe9001245d16b91a28c))
    - Overhaul map-entities to work within replication groups ([`f69b04c`](https://github.com/cBournhonesque/lightyear/commit/f69b04c80b79c49586a5969774fd7648b2f04953))
    - Working snake example, now need to fix client ([`c58f620`](https://github.com/cBournhonesque/lightyear/commit/c58f6205e20eae0b7d576d8771a48fecba9fb978))
    - Update readme ([`bb043f5`](https://github.com/cBournhonesque/lightyear/commit/bb043f55279caf521e5308eddef74b9fae25a353))
    - Wasm support ([`2cfa90c`](https://github.com/cBournhonesque/lightyear/commit/2cfa90cd5ba0086cad7c8934bd34048c26b9abb2))
    - Update example ([`b45806c`](https://github.com/cBournhonesque/lightyear/commit/b45806c78953cb6fc508af99ea72524337e7ab81))
    - Making progress ([`79dcf9f`](https://github.com/cBournhonesque/lightyear/commit/79dcf9fa9cae07e06c2ec1c6febde20cecd5ad4b))
    - Added rustflags ([`603103c`](https://github.com/cBournhonesque/lightyear/commit/603103cd1b99b24b6bf2cf5c0d81732a90a55bc0))
    - Use xwt for native ([`2cefdec`](https://github.com/cBournhonesque/lightyear/commit/2cefdec707370a2c13e50a2f5ed411aa56208a4d))
    - Merge pull request #15 from cBournhonesque/cb/fix-prediction ([`2350566`](https://github.com/cBournhonesque/lightyear/commit/235056699866a4ab6996dea5bd1c6f8c37874b0a))
    - Update format ([`7f677c3`](https://github.com/cBournhonesque/lightyear/commit/7f677c3d62a091e7b90db51a87db6a13a8450ce5))
    - Increment version type ([`26846ae`](https://github.com/cBournhonesque/lightyear/commit/26846ae238ef73ad88e5459366cc562483f6f30f))
    - Clean PR ([`267f101`](https://github.com/cBournhonesque/lightyear/commit/267f10167236e5efbeef2bbb55edabe966846cdb))
    - Fixing most prediction/interpolation bugs ([`64332bb`](https://github.com/cBournhonesque/lightyear/commit/64332bb455b76be10bc8fd52d942d76caf897c28))
    - Added some tests; added Debug derive on P::Components that just transforms to Kind ([`b969258`](https://github.com/cBournhonesque/lightyear/commit/b969258a864ebc2552fde7183fbdb2883d047981))
    - Debugging start ([`9f426d9`](https://github.com/cBournhonesque/lightyear/commit/9f426d9ddae91631564081b4c0bffe997eb2d45d))
    - Tests pass ([`7d06576`](https://github.com/cBournhonesque/lightyear/commit/7d06576592dd82acd4c34fb38c6463cc99cb2026))
    - Builds with new replication system ([`4762a4f`](https://github.com/cBournhonesque/lightyear/commit/4762a4f19bc1ce81b4c3a4b3448e39c5c38b75f6))
    - Wip ([`657426a`](https://github.com/cBournhonesque/lightyear/commit/657426a19780a1e037ce95acc45911326d71d57a))
    - Wip ([`babd966`](https://github.com/cBournhonesque/lightyear/commit/babd966bc1ca7d743a45114f5cf12b88c461f2fe))
    - Wip ([`9042d1d`](https://github.com/cBournhonesque/lightyear/commit/9042d1d5396d80fe42d7500c56bd1d83be0a414d))
    - Merge pull request #11 from cBournhonesque/cb/fix-interpolation ([`57efbfb`](https://github.com/cBournhonesque/lightyear/commit/57efbfb0bda7557fd8948d2651cfa5364b9be6f2))
    - Fix the bug with the interpolation start ([`6db2aa8`](https://github.com/cBournhonesque/lightyear/commit/6db2aa88c4c68c9d2a7dd2923d862b257fab475a))
    - Small bug fix ([`a8b356a`](https://github.com/cBournhonesque/lightyear/commit/a8b356af2ef4723f2efebfc009e3fe0087135180))
    - Small fixes ([`d41d033`](https://github.com/cBournhonesque/lightyear/commit/d41d03379b1bfd0ea8315b073cd5a6f37fc47fe9))
    - Merge pull request #10 from cBournhonesque/cb/add-interest-management ([`f009404`](https://github.com/cBournhonesque/lightyear/commit/f00940408c7f80cb6f3cbf186fafabd325d24443))
    - Update crate version ([`bc2c5c8`](https://github.com/cBournhonesque/lightyear/commit/bc2c5c8ba4ae01da7a884322d3ce357b13b4a7a5))
    - Add scope management via rooms. Also make sure the inserts/removals/updates of an entity are each sent in the same packet. Fix some replication issues ([`209cab7`](https://github.com/cBournhonesque/lightyear/commit/209cab7e6028f62e6d5c054ea47438f9162912c8))
    - Added some tests for room management ([`3ec375c`](https://github.com/cBournhonesque/lightyear/commit/3ec375cdf2385f930d4143c9dd277609002e489f))
    - Build pass. added RoomPlugin ([`7cba16a`](https://github.com/cBournhonesque/lightyear/commit/7cba16a78bdede034690233191c15613a6cb585d))
    - Merge pull request #9 from cBournhonesque/cb/upgrade-wtransport ([`23173d3`](https://github.com/cBournhonesque/lightyear/commit/23173d356828f58b3cc036b2ac8ebe9da270d543))
    - Remove vendored wtransport ([`2e2669c`](https://github.com/cBournhonesque/lightyear/commit/2e2669c4407061452ab054645da64b889bc8eb26))
    - Wip ([`ec829bc`](https://github.com/cBournhonesque/lightyear/commit/ec829bc4e7f2fb7579180438529e262e6efc04d9))
    - Fix fmt ([`df2f7b5`](https://github.com/cBournhonesque/lightyear/commit/df2f7b546081aac93732caeb2b5330ae838b2602))
    - Fix example ([`f48f601`](https://github.com/cBournhonesque/lightyear/commit/f48f60177ad6e8dbbf2301fc5b8ce6cdb182c906))
    - Merge pull request #7 from cBournhonesque/cb/update-book-actions ([`4657ba6`](https://github.com/cBournhonesque/lightyear/commit/4657ba64bd10fa6a6f60e233a54a6e160fd2e383))
    - Update version ([`a2a826e`](https://github.com/cBournhonesque/lightyear/commit/a2a826eea4a493ac7024e0354bd8c0807c4c77cd))
    - Add readme ([`7638259`](https://github.com/cBournhonesque/lightyear/commit/76382592a0a749da626cac96f91c49595c76099b))
    - Add vendored crates ([`7befad6`](https://github.com/cBournhonesque/lightyear/commit/7befad67157323866f6d5ee9d83b5261db0c40b6))
    - Merge pull request #5 from cBournhonesque/cb/fix-world-state-replication ([`1f8d166`](https://github.com/cBournhonesque/lightyear/commit/1f8d16659fb0ec052f46b964f2d1a5ba9c73d43d))
    - Clippy ([`63fe9ca`](https://github.com/cBournhonesque/lightyear/commit/63fe9ca9c06604eb716d3201ef114820c36e14df))
    - Fix entity map ([`bc174f5`](https://github.com/cBournhonesque/lightyear/commit/bc174f5c809b200515b82620b08b18b129d23b4e))
    - Fix replicating world state upon connect ([`0094111`](https://github.com/cBournhonesque/lightyear/commit/0094111bc4bb10653edc69c7b30d1eaefc5d7afb))
    - Merge pull request #4 from cBournhonesque/cb/add-webtransport ([`b43df73`](https://github.com/cBournhonesque/lightyear/commit/b43df731e104dfbbd95cf319e9f0d69ef21b7225))
    - Fix ci ([`4f0ef2c`](https://github.com/cBournhonesque/lightyear/commit/4f0ef2cf5c4f3bf661e9fe0d2a1bc6e449d14767))
    - Enable test coverage with tarpaulin, webtransport seems to work (added to simplebox example) ([`9127181`](https://github.com/cBournhonesque/lightyear/commit/9127181f4701155e84612615ae11d49d30f7a6db))
    - Simple box is working with webtransport ([`ef41551`](https://github.com/cBournhonesque/lightyear/commit/ef4155195f6845fd279912df88b34886a8df6318))
    - Added select loop, compiles ([`d728ab7`](https://github.com/cBournhonesque/lightyear/commit/d728ab7b3c9c6ce7ac435f6549078ad0247c99f3))
    - Update book and add packet stats manager to track packet loss ([`31a59c7`](https://github.com/cBournhonesque/lightyear/commit/31a59c7df8467df4f75c5879b62218fb380cb9f1))
    - Enable entity mapping inside components/messages by making everything implement MapEntities ([`e48fdfc`](https://github.com/cBournhonesque/lightyear/commit/e48fdfcd0c2fde62c2aeb017da46203ef756d929))
    - Fix doctest ([`b74d552`](https://github.com/cBournhonesque/lightyear/commit/b74d5525d5fcbb9e80f04e1b8227a68d0c11c6cb))
    - Fix ([`f3e90ff`](https://github.com/cBournhonesque/lightyear/commit/f3e90ffae081dcb4910a2a6918b7623be8a9ca5a))
    - Fix CI and add component removal test ([`9f28762`](https://github.com/cBournhonesque/lightyear/commit/9f287620f01340004fe1140b3273cd7ed6225a6c))
    - Make ci pass ([`ef5d21f`](https://github.com/cBournhonesque/lightyear/commit/ef5d21f81326037f5f925189ef671ad42a72d6d1))
    - Implement Serializer/Deserializer for LightyearSerde ([`0a7a091`](https://github.com/cBournhonesque/lightyear/commit/0a7a091f734cd2c57d8b4d40b99856d5a13fa32c))
    - Improvements ([`5bc82a8`](https://github.com/cBournhonesque/lightyear/commit/5bc82a890b63a9a089327018d66004c6f949c9cf))
    - Cleanup and move to naia 0.16 ([`15fa3f6`](https://github.com/cBournhonesque/lightyear/commit/15fa3f66bfb279d1f39cc1860bc7ce5ede050787))
    - Add all files ([`bebb48d`](https://github.com/cBournhonesque/lightyear/commit/bebb48d4d287fb93560976e901c920343e518a70))
</details>

## v0.21.0-rc.3 (2025-07-03)

<csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/>
<csr-id-1abda441054255978b6d5bef9da8e538b91aa1ed/>
<csr-id-81341e91707b31a5cba6967d23e230945180a4e8/>
<csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/>
<csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/>
<csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/>
<csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/>
<csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/>
<csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/>
<csr-id-249b40f358977f6f85e269967d3912bfb4080f73/>
<csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/>
<csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/>
<csr-id-411733089f59eb90d405f7ad327b5440b55ef060/>
<csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/>
<csr-id-307f947d03668b272beab1a4285dc9cc3463b867/>
<csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/>
<csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/>
<csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/>
<csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/>

### Chore

 - <csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/> release rc3
 - <csr-id-1abda441054255978b6d5bef9da8e538b91aa1ed/> separate avian into 2 crates, fix delta-compression example, fix book links
 - <csr-id-81341e91707b31a5cba6967d23e230945180a4e8/> release 0.21 rc 2
 - <csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/> add tests for delta-compression
 - <csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/> fix compiletime benchmark
 - <csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/> fix ci
 - <csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/> fix
 - <csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/> cargo fmt
 - <csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/> fix tests, cargo doc, cargo clippy
 - <csr-id-249b40f358977f6f85e269967d3912bfb4080f73/> fix clippy
 - <csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/> cargo fmt
 - <csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/> enable host-server for all examples
 - <csr-id-411733089f59eb90d405f7ad327b5440b55ef060/> enable host-client mode on simple box
 - <csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/> fix warns
 - <csr-id-307f947d03668b272beab1a4285dc9cc3463b867/> fix typo in docs

### Documentation

 - <csr-id-f4985d9f1c6c3fec718f11925060448f22c8be93/> fix typos, update ring

### New Features

 - <csr-id-0bd3fbe9db6d8dfd350a0e014e7beec9392df1de/> enable steam on simple_box example and fix wasm
 - <csr-id-117b0841a25dba5c6ffaadad88a8c4dba09d3cbb/> support BEI inputs
 - <csr-id-d11f597140a81b73656dabb585e2ecca62fd208b/> implement wasm & fix small mistakes
 - <csr-id-7781029b8c17f1558400b7095cedeb85c6f269c9/> add non-working native ws impl

### Bug Fixes

 - <csr-id-7d9dbbf435e94e1bc85e631a1df76951150f5aad/> register prespawned entity in predicted_entity_map during server/client match
 - <csr-id-ae2f4b2a5caf60eabbbd83877a5c5c8a3486588e/> Remove `ring` to fix wasm32 web builds
 - <csr-id-b77f2eeb5e7751016e9a981407710c60a9c75c88/> expose ReplicateToServer
 - <csr-id-86f20c79f6930d19ecc3cbf5b97a7e36b6b5b7a7/> Add try_from_bytes method to ConnectToken
 - <csr-id-f96c5fa8fcaabb61d884baf1ee9ce6d6f37d5322/> sending now after opening socket and uncomment packetsender impl code
 - <csr-id-c3f1faf98d770390323f1821861bf80e3e9d0415/> use io task pool instead of tokio::spawn and change receiver / sender
 - <csr-id-2293aed17794c1a2017a78145258bdf9a84bca6a/> recv impl

### Other

 - <csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/> add option to trigger change detection
   * add option to trigger change detection
   
   * fix bug
 - <csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/> add close method to packet sender / packet receiver
 - <csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/> small ownership issues
 - <csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/> add unit tests

## v0.21.0-rc.2 (2025-07-01)

<csr-id-cedab052a0f47cf91b15267b8d83eb87524a8f4d/>
<csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/>
<csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/>
<csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/>
<csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/>
<csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/>
<csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/>
<csr-id-249b40f358977f6f85e269967d3912bfb4080f73/>
<csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/>
<csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/>
<csr-id-411733089f59eb90d405f7ad327b5440b55ef060/>
<csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/>
<csr-id-307f947d03668b272beab1a4285dc9cc3463b867/>
<csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/>
<csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/>
<csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/>
<csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/>

### Chore

 - <csr-id-cedab052a0f47cf91b15267b8d83eb87524a8f4d/> add release command to ci
 - <csr-id-72ecbb9604bbb7add8e911cf9d72f21fd00eed6c/> add tests for delta-compression
 - <csr-id-cc8433c61122e6f8c712c3463d0e91d5230290e7/> fix compiletime benchmark
 - <csr-id-f9bc3e3d8322d252d80363f716d5e78782520cff/> fix ci
 - <csr-id-9436dd60efc0604f874dc09abe43c4dff12579fb/> fix
 - <csr-id-ade88cad9e463e79f3251e55e8eeb18182deb5e3/> cargo fmt
 - <csr-id-fe0bb4a24112a308eaf9c829fe5cfae0180ef946/> fix tests, cargo doc, cargo clippy
 - <csr-id-249b40f358977f6f85e269967d3912bfb4080f73/> fix clippy
 - <csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/> cargo fmt
 - <csr-id-bc7cf371f822ff7a2667c329b6f77e5a694a93d4/> enable host-server for all examples
 - <csr-id-411733089f59eb90d405f7ad327b5440b55ef060/> enable host-client mode on simple box
 - <csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/> fix warns
 - <csr-id-307f947d03668b272beab1a4285dc9cc3463b867/> fix typo in docs

### Documentation

 - <csr-id-f4985d9f1c6c3fec718f11925060448f22c8be93/> fix typos, update ring

### New Features

 - <csr-id-117b0841a25dba5c6ffaadad88a8c4dba09d3cbb/> support BEI inputs
 - <csr-id-d11f597140a81b73656dabb585e2ecca62fd208b/> implement wasm & fix small mistakes
 - <csr-id-7781029b8c17f1558400b7095cedeb85c6f269c9/> add non-working native ws impl

### Bug Fixes

 - <csr-id-7d9dbbf435e94e1bc85e631a1df76951150f5aad/> register prespawned entity in predicted_entity_map during server/client match
 - <csr-id-ae2f4b2a5caf60eabbbd83877a5c5c8a3486588e/> Remove `ring` to fix wasm32 web builds
 - <csr-id-b77f2eeb5e7751016e9a981407710c60a9c75c88/> expose ReplicateToServer
 - <csr-id-86f20c79f6930d19ecc3cbf5b97a7e36b6b5b7a7/> Add try_from_bytes method to ConnectToken
 - <csr-id-f96c5fa8fcaabb61d884baf1ee9ce6d6f37d5322/> sending now after opening socket and uncomment packetsender impl code
 - <csr-id-c3f1faf98d770390323f1821861bf80e3e9d0415/> use io task pool instead of tokio::spawn and change receiver / sender
 - <csr-id-2293aed17794c1a2017a78145258bdf9a84bca6a/> recv impl

### Other

 - <csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/> add option to trigger change detection
   * add option to trigger change detection
   
   * fix bug
 - <csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/> add close method to packet sender / packet receiver
 - <csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/> small ownership issues
 - <csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/> add unit tests

## v0.21.0-rc.1 (2025-06-08)

<csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/>
<csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/>
<csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/>
<csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/>
<csr-id-f241c9deba7c584a345cd2e267a60ab95e0aeb70/>
<csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/>
<csr-id-307f947d03668b272beab1a4285dc9cc3463b867/>

Release of refactor

### Other

 - <csr-id-87806d0c9cb6fef22978cf7b170089e37711d329/> add option to trigger change detection
   * add option to trigger change detection
   
   * fix bug
 - <csr-id-ba64c1f649c57b57aa6f726d6cb9de2e03128f8f/> add close method to packet sender / packet receiver
 - <csr-id-4103090a52afc050c1de6dcbd7eb278af5ffe94a/> small ownership issues
 - <csr-id-4adee75a35e02cc8f02a87fefc4dc6ee8e178fa7/> add unit tests

### Bug Fixes

 - <csr-id-7d9dbbf435e94e1bc85e631a1df76951150f5aad/> register prespawned entity in predicted_entity_map during server/client match
 - <csr-id-ae2f4b2a5caf60eabbbd83877a5c5c8a3486588e/> Remove `ring` to fix wasm32 web builds
 - <csr-id-b77f2eeb5e7751016e9a981407710c60a9c75c88/> expose ReplicateToServer
 - <csr-id-86f20c79f6930d19ecc3cbf5b97a7e36b6b5b7a7/> Add try_from_bytes method to ConnectToken
 - <csr-id-f96c5fa8fcaabb61d884baf1ee9ce6d6f37d5322/> sending now after opening socket and uncomment packetsender impl code
 - <csr-id-c3f1faf98d770390323f1821861bf80e3e9d0415/> use io task pool instead of tokio::spawn and change receiver / sender
 - <csr-id-2293aed17794c1a2017a78145258bdf9a84bca6a/> recv impl

### New Features

 - <csr-id-d11f597140a81b73656dabb585e2ecca62fd208b/> implement wasm & fix small mistakes
 - <csr-id-7781029b8c17f1558400b7095cedeb85c6f269c9/> add non-working native ws impl

### Documentation

 - <csr-id-f4985d9f1c6c3fec718f11925060448f22c8be93/> fix typos, update ring

### Chore

 - <csr-id-f241c9deba7c584a345cd2e267a60ab95e0aeb70/> fix std flag
 - <csr-id-7f2e8a07187bf71db4d2c7efcd1eaa650ca83735/> fix warns
 - <csr-id-307f947d03668b272beab1a4285dc9cc3463b867/> fix typo in docs

