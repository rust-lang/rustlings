<a name="3.0.0"></a>
## 3.0.0 (2020-04-11)

#### Breaking Changes

* make "compile" exercises print output (#278) ([3b6d5c](https://github.com/fmoko/rustlings/commit/3b6d5c3aaa27a242a832799eb66e96897d26fde3))

#### Bug Fixes

* **primitive_types:** revert primitive_types4 (#296) ([b3a3351e](https://github.com/rust-lang/rustlings/commit/b3a3351e8e6a0bdee07077d7b0382953821649ae))
* **run:**  compile clippy exercise files (#295) ([3ab084a4](https://github.com/rust-lang/rustlings/commit/3ab084a421c0f140ae83bf1fc3f47b39342e7373))
* **conversions:**
  * add additional test to meet exercise rules (#284) ([bc22ec3](https://github.com/fmoko/rustlings/commit/bc22ec382f843347333ef1301fc1bad773657f38))
  * remove duplicate not done comment (#292) ([dab90f](https://github.com/fmoko/rustlings/commit/dab90f7b91a6000fe874e3d664f244048e5fa342))
* don't hardcode documentation version for traits (#288) ([30e6af](https://github.com/fmoko/rustlings/commit/30e6af60690c326fb5d3a9b7335f35c69c09137d))

#### Features

*   add Option2 exercise (#290) ([86b5c08b](https://github.com/rust-lang/rustlings/commit/86b5c08b9bea1576127a7c5f599f5752072c087d))
*   add excercise for option (#282) ([135e5d47](https://github.com/rust-lang/rustlings/commit/135e5d47a7c395aece6f6022117fb20c82f2d3d4))
*   add new exercises for generics (#280) ([76be5e4e](https://github.com/rust-lang/rustlings/commit/76be5e4e991160f5fd9093f03ee2ba260e8f7229))
* **ci:**  add buildkite config ([b049fa2c](https://github.com/rust-lang/rustlings/commit/b049fa2c84dba0f0c8906ac44e28fd45fba51a71))

<a name="2.2.1"></a>
### 2.2.1 (2020-02-27)

#### Bug Fixes

*   Re-add cloning the repo to install scripts ([3d9b03c5](https://github.com/rust-lang/rustlings/commit/3d9b03c52b8dc51b140757f6fd25ad87b5782ef5))

#### Features

*   Add clippy lints (#269) ([1e2fd9c9](https://github.com/rust-lang/rustlings/commit/1e2fd9c92f8cd6e389525ca1a999fca4c90b5921))

<a name="2.2.0"></a>
## 2.2.0 (2020-02-25)


#### Bug Fixes

*   Update deps to version compatable with aarch64-pc-windows (#263) ([19a93428](https://github.com/rust-lang/rustlings/commit/19a93428b3c73d994292671f829bdc8e5b7b3401))
* **docs:**
  * Added a necessary step to Windows installation process (#242) ([3906efcd](https://github.com/rust-lang/rustlings/commit/3906efcd52a004047b460ed548037093de3f523f))
  * Fixed mangled sentence from book; edited for clarity (#266) ([ade52ff](https://github.com/rust-lang/rustlings/commit/ade52ffb739987287ddd5705944c8777705faed9)) 
  * Updated iterators readme to account for iterators4 exercise (#273) ([bec8e3a](https://github.com/rust-lang/rustlings/commit/bec8e3a644cbd88db1c73ea5f1d8a364f4a34016))
* **installation:**  make fatal errors more obvious (#272) ([17d0951e](https://github.com/rust-lang/rustlings/commit/17d0951e66fda8e11b204d5c4c41a0d5e22e78f7))
* **iterators2:**
  *  Remove reference to missing iterators2.rs (#245) ([419f7797](https://github.com/rust-lang/rustlings/commit/419f7797f294e4ce6a2b883199731b5bde77d262))
* **as_ref_mut:** Enable a test and improve per clippy's suggestion (#256) ([dfdf809](https://github.com/rust-lang/rustlings/commit/dfdf8093ebbd4145864995627b812780de52f902))
* **tests1:**
  * Change test command ([fe10e06c](https://github.com/rust-lang/rustlings/commit/fe10e06c3733ddb4a21e90d09bf79bfe618e97ce) 
  * Correct test command in tests1.rs comment (#263) ([39fa7ae](https://github.com/rust-lang/rustlings/commit/39fa7ae8b70ad468da49b06f11b2383135a63bcf))

#### Features

*   Add variables5.rs exercise (#264) ([0c73609e](https://github.com/rust-lang/rustlings/commit/0c73609e6f2311295e95d6f96f8c747cfc4cba03))
*   Show a completion message when watching (#253) ([d25ee55a](https://github.com/rust-lang/rustlings/commit/d25ee55a3205882d35782e370af855051b39c58c))
*   Add type conversion and parsing exercises (#249) ([0c85dc11](https://github.com/rust-lang/rustlings/commit/0c85dc1193978b5165491b99cc4922caf8d14a65))
*   Created consistent money unit (#258) ([fd57f8f](https://github.com/rust-lang/rustlings/commit/fd57f8f2c1da2af8ddbebbccec214e6f40f4dbab))
*   Enable test for exercise test4 (#276) ([8b971ff](https://github.com/rust-lang/rustlings/commit/8b971ffab6079a706ac925f5917f987932b55c07))
*   Added traits exercises (#274 but specifically #216, which originally added
    this :heart:) ([b559cdd](https://github.com/rust-lang/rustlings/commit/b559cdd73f32c0d0cfc1feda39f82b3e3583df17))


<a name="2.1.0"></a>
## 2.1.0 (2019-11-27)

#### Bug Fixes

* add line numbers in several exercises and hints ([b565c4d3](https://github.com/rust-lang/rustlings/commit/b565c4d3e74e8e110bef201a082fa1302722a7c3))
* **arc1:**  Fix some words in the comment ([c42c3b21](https://github.com/rust-lang/rustlings/commit/c42c3b2101df9164c8cd7bb344def921e5ba3e61))
* **enums:**  Add link to chapter on pattern syntax (#242) ([615ce327](https://github.com/rust-lang/rustlings/commit/615ce3279800c56d89f19d218ccb7ef576624feb))
* **primitive_types4:**
  *  update outdated hint ([4c5189df](https://github.com/rust-lang/rustlings/commit/4c5189df2bdd9a231f6b2611919ba5aa14da0d3f))
  *  update outdated comment ([ded2c034](https://github.com/rust-lang/rustlings/commit/ded2c034ba93fa1e3c2c2ea16b83abc1a57265e8))
* **strings2:**  update line number in hint ([a09f684f](https://github.com/rust-lang/rustlings/commit/a09f684f05c58d239a6fc59ec5f81c2533e8b820))
* **variables1:**  Correct wrong word in comment ([fda5a470](https://github.com/rust-lang/rustlings/commit/fda5a47069e0954f16a04e8e50945e03becb71a5))

#### Features

* **watch:**  show hint while watching ([8143d57b](https://github.com/rust-lang/rustlings/commit/8143d57b4e88c51341dd4a18a14c536042cc009c))

<a name="2.0.0"></a>
## 2.0.0 (2019-11-12)

#### Bug Fixes

* **default:**  Clarify the installation procedure ([c371b853](https://github.com/rust-lang/rustlings/commit/c371b853afa08947ddeebec0edd074b171eeaae0))
* **info:**  Fix trailing newlines for hints ([795b6e34](https://github.com/rust-lang/rustlings/commit/795b6e348094a898e9227a14f6232f7bb94c8d31))
* **run:**  make `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Breaking Changes

*   Refactor hint system ([9bdb0a12](https://github.com/rust-lang/rustlings/commit/9bdb0a12e45a8e9f9f6a4bd4a9c172c5376c7f60))
*   improve `watch` execution mode ([2cdd6129](https://github.com/rust-lang/rustlings/commit/2cdd61294f0d9a53775ee24ad76435bec8a21e60))
*   Index exercises by name ([627cdc07](https://github.com/rust-lang/rustlings/commit/627cdc07d07dfe6a740e885e0ddf6900e7ec336b))
* **run:**  makes `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Features

* **cli:**  check for rustc before doing anything ([36a033b8](https://github.com/rust-lang/rustlings/commit/36a033b87a6549c1e5639c908bf7381c84f4f425))
* **hint:**  Add test for hint ([ce9fa6eb](https://github.com/rust-lang/rustlings/commit/ce9fa6ebbfdc3e7585d488d9409797285708316f))

<a name="1.5.1"></a>
### 1.5.1 (2019-11-11)

#### Bug Fixes

* **errors3:**  Update hint ([dcfb427b](https://github.com/rust-lang/rustlings/commit/dcfb427b09585f0193f0a294443fdf99f11c64cb), closes [#185](https://github.com/rust-lang/rustlings/issues/185))
* **if1:**  Remove `return` reference ([ad03d180](https://github.com/rust-lang/rustlings/commit/ad03d180c9311c0093e56a3531eec1a9a70cdb45))
* **strings:**  Move Strings before Structs ([6dcecb38](https://github.com/rust-lang/rustlings/commit/6dcecb38a4435593beb87c8e12d6314143631482), closes [#204](https://github.com/rust-lang/rustlings/issues/204))
* **structs1:**  Remove misleading comment ([f72e5a8f](https://github.com/rust-lang/rustlings/commit/f72e5a8f05568dde04eaeac10b9a69872f21cb37))
* **threads:**  Move Threads behind SLT ([fbe91a67](https://github.com/rust-lang/rustlings/commit/fbe91a67a482bfe64cbcdd58d06ba830a0f39da3), closes [#205](https://github.com/rust-lang/rustlings/issues/205))
* **watch:** clear screen before each `verify()`  ([3aff590](https://github.com/rust-lang/rustlings/commit/3aff59085586c24196a547c2693adbdcf4432648))

<a name="1.5.0"></a>
## 1.5.0 (2019-11-09)

#### Bug Fixes

* **test1:** Rewrite logic ([79a56942](https://github.com/rust-lang/rustlings/commit/79a569422c8309cfc9e4aed25bf4ab3b3859996b))
* **installation:**  Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
* **iterators:**  Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
* **iterators2:**  Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
* **option1:**
  *  Fix arguments passed to assert! macro (#222) ([4c2cf6da](https://github.com/rust-lang/rustlings/commit/4c2cf6da755efe02725e05ecc3a303304c10a6da))
  *  Fix arguments passed to assert! macro ([ead4f7af](https://github.com/rust-lang/rustlings/commit/ead4f7af9e10e53418efdde5c359159347282afd))
  *  Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
* **primitive_types4:**  Fail on a slice covering the wrong area ([5b1e673c](https://github.com/rust-lang/rustlings/commit/5b1e673cec1658afc4ebbbc800213847804facf5))
* **readme:**  http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
* **test1:**
  *  Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))
  *  renamed function name to snake case closes #180 ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))

#### Features

*   Add enums exercises ([dc150321](https://github.com/rust-lang/rustlings/commit/dc15032112fc485226a573a18139e5ce928b1755))
*   Added exercise for struct update syntax ([1c4c8764](https://github.com/rust-lang/rustlings/commit/1c4c8764ed118740cd4cee73272ddc6cceb9d959))
* **iterators2:**  adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

<a name="1.4.1"></a>
### 1.4.1 (2019-08-13)


#### Bug Fixes

* **iterators2:**  Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
* **option1:**  Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
* **test1:**  Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))



<a name="1.4.0"></a>
## 1.4.0 (2019-07-13)

#### Bug Fixes

* **installation:**  Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
* **iterators:**  Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
* **readme:**  http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
* **test1:**  renamed function name to snake case ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))
* **cli:** Check if changed exercise file exists before calling verify ([ba85ca3](https://github.com/rust-lang/rustlings/commit/ba85ca32c4cfc61de46851ab89f9c58a28f33c88))
* **structs1:** Fix the irrefutable let pattern warning ([cc6a141](https://github.com/rust-lang/rustlings/commit/cc6a14104d7c034eadc98297eaaa972d09c50b1f))

#### Features

* **changelog:**  Use clog for changelogs ([34e31232](https://github.com/rust-lang/rustlings/commit/34e31232dfddde284a341c9609b33cd27d9d5724))
* **iterators2:**  adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

<a name="1.3.0"></a>
### 1.3.0 (2019-06-05)

#### Features

- Adds a simple exercise for structures (#163, @briankung)

#### Bug Fixes

- Add Result type signature as it is difficult for new comers to understand Generics and Error all at once. (#157, @veggiemonk)
- Rustfmt and whitespace fixes (#161, @eddyp)
- errorsn.rs: Separate also the hints from each other to avoid accidental viewing (#162, @eddyp)
- fixed outdated links (#165, @gushroom)
- Fix broken link (#164, @HanKruiger)
- Remove highlighting and syntect (#167, @komaeda)

<a name="1.2.2"></a>
### 1.2.2 (2019-05-07)

#### Bug Fixes

- Reverted `--nocapture` flag since it was causing tests to pass unconditionally

<a name="1.2.1"></a>
### 1.2.1 (2019-04-22)

#### Bug Fixes

- Fix the `--nocapture` feature (@komaeda)
- Provide a nicer error message for when you're in the wrong directory

<a name="1.2.0"></a>
### 1.2.0 (2019-04-22)

#### Features

- Add errors to exercises that compile without user changes (@yvan-sraka)
- Use --nocapture when testing, enabling `println!` when running (@komaeda)

<a name="1.1.1"></a>
### 1.1.1 (2019-04-14)

#### Bug fixes

- Fix permissions on exercise files (@zacanger, #133)
- Make installation checks more thorough (@komaeda, 1b3469f236bc6979c27f6e1a04e4138a88e55de3)
- Fix order of true/false in tests for executables (@mgeier, #137)
- Stop run from panicking when compile fails (@cjpearce, #141)
- Fix intermittent test failure caused by race condition (@cjpearce, #140)
- Fix links by deleting book version (@diodfr, #142)
- Canonicalize paths to fix path matching (@cjpearce, #143)

<a name="1.1.0"></a>
### 1.1.0 (2019-03-20)

- errors2.rs: update link to Rust book (#124)
- Start verification at most recently modified file (#120)
- Watch for file creation events in watch mode (#117)
- Add standard library types to exercises suite (#119)
- Give a warning when Rustlings isn't run from the right directory (#123)
- Verify that rust version is recent enough to install Rustlings (#131)

<a name="1.0.1"></a>
### 1.0.1 (2019-03-06)

- Adds a way to install Rustlings in one command (`curl -L https://git.io/rustlings | bash`)
- Makes `rustlings watch` react to create file events (@shaunbennett, #117)
- Reworks the exercise management to use an external TOML file instead of just listing them in the code

<a name="1.0.0"></a>
### 1.0.0 (2019-03-06)

Initial release.
