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