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
