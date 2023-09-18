<a name="5.6.1"></a>
## 5.6.1 (2023-09-18)

#### Changed

- Converted all exercises with assertions to test mode.

#### Fixed

- `cow1`: Reverted regression introduced by calling `to_mut` where it
  shouldn't have been called, and clarified comment.
- `primitive_types3`: Require at least an array of 100 elements.
- Removed hint comments when no hint exists for the exercise.
- `as_ref_mut`: Fixed a typo in a test function name.
- `enums3`: Fixed formatting with `rustfmt`.

<a name="5.6.0"></a>
## 5.6.0 (2023-09-04)

#### Added

- New exercise: `if3`, teaching the user about `if let` statements.
- `hashmaps2`: Added an extra test function to check if the amount of fruits is higher than zero.
- `enums3`: Added a test for `Message`.
- `if1`: Added a test case to check equal values.
- `if3`: Added a note specifying that there are no test changes needed.

#### Changed

- Swapped the order of threads and smart pointer exercises.
- Rewrote the CLI to use `clap` - it's matured much since we switched to `argh` :)
- `structs3`: Switched from i32 to u32.
- `move_semantics`: Switched 1-4 to tests, and rewrote them to be way simpler, while still teaching about the same 
  concepts.

#### Fixed

- `iterators5`:
  - Removed an outdated part of the hint.
  - Renamed variables to use snake_case.
- `vecs2`: Updated the hint to reference the renamed loop variable.
- `enums3`: Changed message string in test so that it gets properly tested.
- `strings2`: Corrected line number in hint, then removed it (this both happened as part of this release cycle).
- `primitive_types4`: Updated hint to the correct ending index.
- `quiz1`: Removed duplicated sentence from exercise comments.
- `errors4`: Improved comment.
- `from_into`: Fixed test values.
- `cow1`: Added `.to_mut()` to distinguish from the previous test case.
- `threads2`: Updated hint text to reference the correct book heading.

#### Housekeeping

- Cleaned up the explanation paragraphs at the start of each exercise.
- Lots of Nix housekeeping that I don't feel qualified to write about!
- Improved CI workflows, we're now testing on multiple platforms at once.

<a name="5.5.1"></a>
## 5.5.1 (2023-05-17)

#### Fixed

- Reverted `rust-project.json` path generation due to an upstream `rust-analyzer` fix.

<a name="5.5.0"></a>
## 5.5.0 (2023-05-17)

#### Added

- `strings2`: Added a reference to the book chapter for reference conversion
- `lifetimes`: Added a link to the lifetimekata project
- Added a new `tests4` exercises, which teaches about testing for panics
- Added a `!` prefix command to watch mode that runs an external command
- Added a `--success-hints` option to watch mode that shows hints on exercise success

#### Changed

- `vecs2`: Renamed iterator variable bindings for clarify
- `lifetimes`: Changed order of book references
- `hashmaps2`: Clarified instructions in the todo block
- Moved lifetime exercises before test exercises (via the recommended book ordering)
- `options2`: Improved tests for layering options
- `modules2`: Added more information to the hint

#### Fixed

- `errors2`: Corrected a comment wording
- `iterators2`: Fixed a spelling mistake in the hint text
- `variables`: Wrapped the mut keyword with backticks for readability
- `move_semantics2`: Removed references to line numbers
- `cow1`: Clarified the `owned_no_mutation` comments
- `options3`: Changed exercise to panic when no match is found
- `rustlings lsp` now generates absolute paths, which should fix VSCode `rust-analyzer` usage on Windows

#### Housekeeping

- Added a markdown linter to run on GitHub actions
- Split quick installation section into two code blocks

<a name="5.4.1"></a>
## 5.4.1 (2023-03-10)

#### Changed

- `vecs`: Added links to `iter_mut` and `map` to README.md
- `cow1`: Changed main to tests
- `iterators1`: Formatted according to rustfmt

#### Fixed

- `errors5`: Unified undisclosed type notation
- `arc1`: Improved readability by avoiding implicit dereference
- `macros4`: Prevented auto-fix by adding `#[rustfmt::skip]`
- `cli`: Actually show correct progress percentages

<a name="5.4.0"></a>

## 5.4.0 (2023-02-12)

#### Changed

- Reordered exercises
  - Unwrapped `standard_library_types` into `iterators` and `smart_pointers`
  - Moved smart pointer exercises behind threads
  - Ordered `rc1` before `arc1`
- **intro1**: Added a note on `rustlings lsp`
- **threads1**: Panic if threads are not joined
- **cli**:
  - Made progress bar update proportional to amount of files verified
  - Decreased `watch` delay from 2 to 1 second

#### Fixed

- Capitalized "Rust" in exercise hints
- **enums3**: Removed superfluous tuple brackets
- **quiz2, clippy1, iterators1**: Fixed a typo
- **rc1**: Fixed a prompt error
- **cli**:
  - Fixed a typo in a method name
  - Specified the edition in `rustc` commands

#### Housekeeping

- Bumped min Rust version to 1.58 in installation script

<a name="5.3.0"></a>

## 5.3.0 (2022-12-23)

#### Added

- **cli**: Added a percentage display in watch mode
- Added a `flake.nix` for Nix users

#### Changed

- **structs3**: Added an additional test
- **macros**: Added a link to MacroKata in the README

#### Fixed

- **strings3**: Added a link to `std` in the hint
- **threads1**: Corrected a hint link
- **iterators1**: Clarified hint steps
- **errors5**: Fix a typo in the hint
- **options1**: Clarified on the usage of the 24-hour system
- **threads2, threads3**: Explicitly use `Arc::clone`
- **structs3**: Clarifed the hint
- **quiz2, as_ref_mut, options1, traits1, traits2**: Clarified hints
- **traits1, traits2, cli**: Tidied up unmatching backticks
- **enums2**: Removed unneccessary indirection of self
- **enums3**: Added an extra tuple comment

#### Housekeeping

- Added a VSCode extension recommendation
- Applied some Clippy and rustfmt formatting
- Added a note on Windows PowerShell and other shell compatibility

<a name="5.2.1"></a>

## 5.2.1 (2022-09-06)

#### Fixed

- **quiz1**: Reworded the comment to actually reflect what's going on in the tests.
  Also added another assert just to make sure.
- **rc1**: Fixed a typo in the hint.
- **lifetimes**: Add quotes to the `println!` output, for readability.

#### Housekeeping

- Fixed a typo in README.md

<a name="5.2.0"></a>

## 5.2.0 (2022-08-27)

#### Added

- Added a `reset` command

#### Changed

- **options2**: Convert the exercise to use tests

#### Fixed

- **threads3**: Fixed a typo
- **quiz1**: Adjusted the explanations to be consistent with
  the tests

<a name="5.1.1"></a>

## 5.1.1 (2022-08-17)

#### Bug Fixes

- Fixed an incorrect assertion in options1

<a name="5.1.0"></a>

## 5.1.0 (2022-08-16)

#### Features

- Added a new `rc1` exercise.
- Added a new `cow1` exercise.

#### Bug Fixes

- **variables5**: Corrected reference to previous exercise
- **functions4**: Fixed line number reference
- **strings3**: Clarified comment wording
- **traits4, traits5**: Fixed line number reference
- **traits5**:
  - Fixed typo in "parameter"
  - Made exercise prefer a traits-based solution
- **lifetimes2**: Improved hint
- **threads3**: Fixed typo in hint
- **box1**: Replaced `unimplemented!` with `todo!`
- **errors5**: Provided an explanation for usage of `Box<dyn Error>`
- **quiz2**: Fixed a typo
- **macros**: Updated the macros book link
- **options1**:
  - Removed unused code
  - Added more granular tests
- Fixed some comment syntax shenanigans in info.toml

#### Housekeeping

- Fixed a typo in .editorconfig
- Fixed a typo in integration_tests.rs
- Clarified manual installation instructions using `cargo install --path .`
- Added a link to our Zulip in the readme file

<a name="5.0.0"></a>

## 5.0.0 (2022-07-16)

#### Features

- Hint comments in exercises now also include a reference to the
  `hint` watch mode subcommand.
- **intro1**: Added more hints to point the user to the source file.
- **variables**: Switched variables3 and variables4.
- Moved `vec` and `primitive_types` exercises before `move_semantics`.
- Renamed `vec` to `vecs` to be more in line with the naming in general.
- Split up the `collections` exercises in their own folders.
- **vec2**: Added a second part of the function that provides an alternative,
  immutable way of modifying vec values.
- **enums3**: Added a hint.
- Moved `strings` before `modules`.
- Added a `strings3` exercise to teach modifying strings.
- Added a `hashmaps3` exercise for some advanced usage of hashmaps.
- Moved the original `quiz2` to be `strings4`, since it only tested strings
  anyways.
- Reworked `quiz2` into a new exercise that tests more chapters.
- Renamed `option` to `options`.
- **options1**: Rewrote parts of the exercise to remove the weird array
  iteration stuff.
- Moved `generics3` to be `quiz3`.
- Moved box/arc exercises behind `iterators`.
- **iterators4**: Added a test for factorials of zero.
- Split `threads1` between two exercises, the first one focusing more on
  `JoinHandle`s.
- Added a `threads3` exercises that uses `std::sync::mpsc`.
- Added a `clippy3` exercises with some more interesting checks.
- **as_ref_mut**: Added a section that actually tests `AsMut`.
- Added 3 new lifetimes exercises.
- Added 3 new traits exercises.

#### Bug Fixes

- **variables2**: Made output messages more verbose.
- **variables5**: Added a nudging hint about shadowing.
- **variables6**: Fixed link to book.
- **functions**: Clarified the README wording. Generally cleaned up
  some hints and added some extra comments.
- **if2**: Renamed function name to `foo_if_fizz`.
- **move_semantics**: Clarified some hints.
- **quiz1**: Renamed the function name to be more verbose.
- **structs1**: Use an integer type instead of strings. Renamed "unit structs"
  to "unit-like structs", as is used in the book.
- **structs3**: Added the `panic!` statement in from the beginning.
- **errors1**: Use `is_empty()` instead of `len() > 0`
- **errors3**: Improved the hint.
- **errors5**: Improved exercise instructions and the hint.
- **errors6**: Provided the skeleton of one of the functions that's supposed
  to be implemented.
- **iterators3**: Inserted `todo!` into `divide()` to keep a compiler error
  from happening.
- **from_str**: Added a hint comment about string error message conversion with
  `Box<dyn Error>`.
- **try_from_into**: Fixed the function name in comment.

#### Removed

- Removed the legacy LSP feature that was using `mod.rs` files.
- Removed `quiz4`.
- Removed `advanced_errs`. These were the last exercises in the recommended
  order, and I've always felt like they didn't quite fit in with the mostly
  simple, book-following style we've had in Rustlings.

#### Housekeeping

- Added missing exercises to the book index.
- Updated spacing in Cargo.toml.
- Added a GitHub actions config so that tests run on every PR/commit.

<a name="4.8.0"></a>

## 4.8.0 (2022-07-01)

#### Features

- Added a progress indicator for `rustlings watch`.
- The installation script now checks for Rustup being installed.
- Added a `rustlings lsp` command to enable `rust-analyzer`.

#### Bug Fixes

- **move_semantics5**: Replaced "in vogue" with "in scope" in hint.
- **if2**: Fixed a typo in the hint.
- **variables1**: Fixed an incorrect line reference in the hint.
- Fixed an out of bounds check in the installation Bash script.

#### Housekeeping

- Replaced the git.io URL with the fully qualified URL because of git.io's sunsetting.
- Removed the deprecated Rust GitPod extension.

<a name="4.7.1"></a>

## 4.7.1 (2022-04-20)

#### Features

- The amount of dependency crates that need to be compiled went down from ~65 to
  ~45 by bumping dependency versions.
- The minimum Rust version in the install scripts has been bumped to 1.56.0 (this isn't in
  the release itself, since install scripts don't really get versioned)

#### Bug Fixes

- **arc1**: A small part has been rewritten using a more functional code style (#968).
- **using_as**: A small part has been refactored to use `sum` instead of `fold`, resulting
  in better readability.

#### Housekeeping

- The changelog will now be manually written instead of being automatically generated by the
  Git log.

<a name="4.7.0"></a>

## 4.7.0 (2022-04-14)

#### Features

- Add move_semantics6.rs exercise (#908) ([3f0e1303](https://github.com/rust-lang/rustlings/commit/3f0e1303e0b3bf3fecc0baced3c8b8a37f83c184))
- **intro:** Add intro section. ([21c9f441](https://github.com/rust-lang/rustlings/commit/21c9f44168394e08338fd470b5f49b1fd235986f))
- Include exercises folder in the project structure behind a feature, enabling rust-analyzer to work (#917) ([179a75a6](https://github.com/rust-lang/rustlings/commit/179a75a68d03ac9518dec2297fb17f91a4fc506b))

#### Bug Fixes

- Fix a few spelling mistakes ([1c0fe3cb](https://github.com/rust-lang/rustlings/commit/1c0fe3cbcca85f90b3985985b8e265ee872a2ab2))
- **cli:**
  - Move long text strings into constants. ([f78c4802](https://github.com/rust-lang/rustlings/commit/f78c48020830d7900dd8d81f355606581670446d))
  - Replace `filter_map()` with `find_map()` ([9b27e8d](https://github.com/rust-lang/rustlings/commit/9b27e8d993ca20232fe38a412750c3f845a83b65))
- **clippy1:**
  - Set clippy::float_cmp lint to deny (#907) ([71a06044](https://github.com/rust-lang/rustlings/commit/71a06044e6a96ff756dc31d7b0ed665ae4badb57))
  - Updated code to test correctness clippy lint with approx_constant lint rule ([f2650de3](https://github.com/rust-lang/rustlings/commit/f2650de369810867d2763e935ac0963c32ec420e))
- **errors1:**
  - Add a comment to make the purpose more clear (#486) ([cbcde345](https://github.com/rust-lang/rustlings/commit/cbcde345409c3e550112e449242848eaa3391bb6))
  - Don't modify tests (#958) ([60bb7cc](https://github.com/rust-lang/rustlings/commit/60bb7cc3931d21d3986ad52b2b302e632a93831c))
- **errors6:** Remove existing answer code ([43d0623](https://github.com/rust-lang/rustlings/commit/43d0623086edbc46fe896ba59c7afa22c3da9f7a))
- **functions5:** Remove wrong new line and small English improvements (#885) ([8ef4869b](https://github.com/rust-lang/rustlings/commit/8ef4869b264094e5a9b50452b4534823a9df19c3))
- **install:** protect path with whitespaces using quotes and stop at the first error ([d114847f](https://github.com/rust-lang/rustlings/commit/d114847f256c5f571c0b4c87e04b04bce3435509))
- **intro1:** Add compiler error explanation. ([9b8de655](https://github.com/rust-lang/rustlings/commit/9b8de65525a5576b78cf0c8e4098cdd34296338f))
- **iterators1:** reorder TODO steps ([0bd7a063](https://github.com/rust-lang/rustlings/commit/0bd7a0631a17a9d69af5746795a30efc9cf64e6e))
- **move_semantics2:** Add comment ([89650f80](https://github.com/rust-lang/rustlings/commit/89650f808af23a32c9a2c6d46592b77547a6a464))
- **move_semantics5:** correct typo (#857) ([46c28d5c](https://github.com/rust-lang/rustlings/commit/46c28d5cef3d8446b5a356b19d8dbc725f91a3a0))
- **quiz1:** update to say quiz covers "If" ([1622e8c1](https://github.com/rust-lang/rustlings/commit/1622e8c198d89739765c915203efff0091bdeb78))
- **structs3:**
  - Add a hint for panic (#608) ([4f7ff5d9](https://github.com/rust-lang/rustlings/commit/4f7ff5d9c7b2d8b045194c1a9469d37e30257c4a))
  - remove redundant 'return' (#852) ([bf33829d](https://github.com/rust-lang/rustlings/commit/bf33829da240375d086f96267fc2e02fa6b07001))
  - Assigned value to `cents_per_gram` in test ([d1ee2da](https://github.com/rust-lang/rustlings/commit/d1ee2daf14f19105e6db3f9c610f44293d688532))
- **structs3.rs:** assigned value to cents_per_gram in test ([d1ee2daf](https://github.com/rust-lang/rustlings/commit/d1ee2daf14f19105e6db3f9c610f44293d688532))
- **traits1:** rename test functions to snake case (#854) ([1663a16e](https://github.com/rust-lang/rustlings/commit/1663a16eade6ca646b6ed061735f7982434d530d))

#### Documentation improvements

- Add hints on how to get GCC installed (#741) ([bc56861](https://github.com/rust-lang/rustlings/commit/bc5686174463ad6f4f6b824b0e9b97c3039d4886))
- Fix some code blocks that were not highlighted ([17f9d74](https://github.com/rust-lang/rustlings/commit/17f9d7429ccd133a72e815fb5618e0ce79560929))

<a name="4.6.0"></a>

## 4.6.0 (2021-09-25)

#### Features

- add advanced_errs2 ([abd6b70c](https://github.com/rust-lang/rustlings/commit/abd6b70c72dc6426752ff41f09160b839e5c449e))
- add advanced_errs1 ([882d535b](https://github.com/rust-lang/rustlings/commit/882d535ba8628d5e0b37e8664b3e2f26260b2671))
- Add a farewell message when quitting `watch` ([1caef0b4](https://github.com/rust-lang/rustlings/commit/1caef0b43494c8b8cdd6c9260147e70d510f1aca))
- add more watch commands ([a7dc080b](https://github.com/rust-lang/rustlings/commit/a7dc080b95e49146fbaafe6922a6de2f8cb1582a), closes [#842](https://github.com/rust-lang/rustlings/issues/842))
- **modules:** update exercises, add modules3 (#822) ([dfd2fab4](https://github.com/rust-lang/rustlings/commit/dfd2fab4f33d1bf59e2e5ee03123c0c9a67a9481))
- **quiz1:** add default function name in comment (#838) ([0a11bad7](https://github.com/rust-lang/rustlings/commit/0a11bad71402b5403143d642f439f57931278c07))

#### Bug Fixes

- Correct small typo in exercises/conversions/from_str.rs ([86cc8529](https://github.com/rust-lang/rustlings/commit/86cc85295ae36948963ae52882e285d7e3e29323))
- **cli:** typo in exercise.rs (#848) ([06d5c097](https://github.com/rust-lang/rustlings/commit/06d5c0973a3dffa3c6c6f70acb775d4c6630323c))
- **from_str, try_from_into:** custom error types ([2dc93cad](https://github.com/rust-lang/rustlings/commit/2dc93caddad43821743e4903d89b355df58d7a49))
- **modules2:** fix typo (#835) ([1c3beb0a](https://github.com/rust-lang/rustlings/commit/1c3beb0a59178c950dc05fe8ee2346b017429ae0))
- **move_semantics5:**
  - change &mut \*y to &mut x (#814) ([d75759e8](https://github.com/rust-lang/rustlings/commit/d75759e829fdcd64ef071cf4b6eae2a011a7718b))
  - Clarify instructions ([df25684c](https://github.com/rust-lang/rustlings/commit/df25684cb79f8413915e00b5efef29369849cef1))
- **quiz1:** Fix inconsistent wording (#826) ([03131a3d](https://github.com/rust-lang/rustlings/commit/03131a3d35d9842598150f9da817f7cc26e2669a))

<a name="4.5.0"></a>

## 4.5.0 (2021-07-07)

#### Features

- Add move_semantics5 exercise. (#746) ([399ab328](https://github.com/rust-lang/rustlings/commit/399ab328d8d407265c09563aa4ef4534b2503ff2))
- **cli:** Add "next" to run the next unsolved exercise. (#785) ([d20e413a](https://github.com/rust-lang/rustlings/commit/d20e413a68772cd493561f2651cf244e822b7ca5))

#### Bug Fixes

- rename result1 to errors4 ([50ab289d](https://github.com/rust-lang/rustlings/commit/50ab289da6b9eb19a7486c341b00048c516b88c0))
- move_semantics5 hints ([1b858285](https://github.com/rust-lang/rustlings/commit/1b85828548f46f58b622b5e0c00f8c989f928807))
- remove trailing whitespaces from iterators1 ([4d4fa774](https://github.com/rust-lang/rustlings/commit/4d4fa77459392acd3581c6068aa8be9a02de12fc))
- add hints to generics1 and generics2 exercises ([31457940](https://github.com/rust-lang/rustlings/commit/31457940846b3844d78d4a4d2b074bc8d6aaf1eb))
- remove trailing whitespace ([d9b69bd1](https://github.com/rust-lang/rustlings/commit/d9b69bd1a0a7a99f2c0d80933ad2eea44c8c71b2))
- **installation:** first PowerShell command ([aa9a943d](https://github.com/rust-lang/rustlings/commit/aa9a943ddf3ae260782e73c26bcc9db60e5894b6))
- **iterators5:** derive Clone, Copy ([91fc9e31](https://github.com/rust-lang/rustlings/commit/91fc9e3118f4af603c9911698cc2a234725cb032))
- **quiz1:** Updated question description (#794) ([d8766496](https://github.com/rust-lang/rustlings/commit/d876649616cc8a8dd5f539f8bc1a5434b960b1e9))
- **try_from_into, from_str:** hints for dyn Error ([11d2cf0d](https://github.com/rust-lang/rustlings/commit/11d2cf0d604dee3f5023c17802d69438e69fa50e))
- **variables5:** confine the answer further ([48ffcbd2](https://github.com/rust-lang/rustlings/commit/48ffcbd2c4cc4d936c2c7480019190f179813cc5))

<a name="4.4.0"></a>

## 4.4.0 (2021-04-24)

#### Bug Fixes

- Fix spelling error in main.rs ([91ee27f2](https://github.com/rust-lang/rustlings/commit/91ee27f22bd3797a9db57e5fd430801c170c5db8))
- typo in default out text ([644c49f1](https://github.com/rust-lang/rustlings/commit/644c49f1e04cbb24e95872b3a52b07d692ae3bc8))
- **collections:** Naming exercises for vectors and hashmap ([bef39b12](https://github.com/rust-lang/rustlings/commit/bef39b125961310b34b34871e480a82e82af4678))
- **from_str:**
  - Correct typos ([5f7c89f8](https://github.com/rust-lang/rustlings/commit/5f7c89f85db1f33da01911eaa479c3a2d4721678))
  - test for error instead of unwrap/should_panic ([15e71535](https://github.com/rust-lang/rustlings/commit/15e71535f37cfaed36e22eb778728d186e2104ab))
  - use trait objects for from_str ([c3e7b831](https://github.com/rust-lang/rustlings/commit/c3e7b831786c9172ed8bd5d150f3c432f242fba9))
- **functions3:** improve function argument type (#687) ([a6509cc4](https://github.com/rust-lang/rustlings/commit/a6509cc4d545d8825f01ddf7ee37823b372154dd))
- **hashmap2:** Update incorrect assertion (#660) ([72aaa15e](https://github.com/rust-lang/rustlings/commit/72aaa15e6ab4b72b3422f1c6356396e20a2a2bb8))
- **info:** Fix typo (#635) ([cddc1e86](https://github.com/rust-lang/rustlings/commit/cddc1e86e7ec744ee644cc774a4887b1a0ded3e8))
- **iterators2:** Moved errors out of tests. ([baf4ba17](https://github.com/rust-lang/rustlings/commit/baf4ba175ba6eb92989e3dd54ecbec4bedc9a863), closes [#359](https://github.com/rust-lang/rustlings/issues/359))
- **iterators3:** Enabled iterators3.rs to run without commented out tests. ([c6712dfc](https://github.com/rust-lang/rustlings/commit/c6712dfccd1a093e590ad22bbc4f49edc417dac0))
- **main:** Let find_exercise work with borrows ([347f30bd](https://github.com/rust-lang/rustlings/commit/347f30bd867343c5ace1097e085a1f7e356553f7))
- **move_semantics4:**
  - Remove redundant "instead" (#640) ([cc266d7d](https://github.com/rust-lang/rustlings/commit/cc266d7d80b91e79df3f61984f231b7f1587218e))
  - Small readbility improvement (#617) ([10965920](https://github.com/rust-lang/rustlings/commit/10965920fbdf8a1efc85bed869e55a1787006404))
- **option2:** Rename uninformative variables (#675) ([b4de6594](https://github.com/rust-lang/rustlings/commit/b4de6594380636817d13c2677ec6f472a964cf43))
- **quiz3:** Force an answer to Q2 (#672) ([0d894e6f](https://github.com/rust-lang/rustlings/commit/0d894e6ff739943901e1ae8c904582e5c2f843bd))
- **structs:** Add 5.3 to structs/README (#652) ([6bd791f2](https://github.com/rust-lang/rustlings/commit/6bd791f2f44aa7f0ad926df767f6b1fa8f12a9a9))
- **structs2:** correct grammar in hint (#663) ([ebdb66c7](https://github.com/rust-lang/rustlings/commit/ebdb66c7bfb6d687a14cc511a559a222e6fc5de4))
- **structs3:**
  - reword heading comment (#664) ([9f3e8c2d](https://github.com/rust-lang/rustlings/commit/9f3e8c2dde645e5264c2d2200e68842b5f47bfa3))
  - add check to prevent naive implementation of is_international ([05a753fe](https://github.com/rust-lang/rustlings/commit/05a753fe6333d36dbee5f68c21dec04eacdc75df))
- **threads1:** line number correction ([7857b0a6](https://github.com/rust-lang/rustlings/commit/7857b0a689b0847f48d8c14cbd1865e3b812d5ca))
- **try_from_into:** use trait objects ([2e93a588](https://github.com/rust-lang/rustlings/commit/2e93a588e0abe8badb7eafafb9e7d073c2be5df8))

#### Features

- Replace clap with argh ([7928122f](https://github.com/rust-lang/rustlings/commit/7928122fcef9ca7834d988b1ec8ca0687478beeb))
- Replace emojis when NO_EMOJI env variable present ([8d62a996](https://github.com/rust-lang/rustlings/commit/8d62a9963708dbecd9312e8bcc4b47049c72d155))
- Added iterators5.rs exercise. ([b29ea17e](https://github.com/rust-lang/rustlings/commit/b29ea17ea94d1862114af2cf5ced0e09c197dc35))
- **arc1:** Add more details to description and hint (#710) ([81be4044](https://github.com/rust-lang/rustlings/commit/81be40448777fa338ebced3b0bfc1b32d6370313))
- **cli:** Improve the list command with options, and then some ([8bbe4ff1](https://github.com/rust-lang/rustlings/commit/8bbe4ff1385c5c169c90cd3ff9253f9a91daaf8e))
- **list:**
  - updated progress percentage ([1c6f7e4b](https://github.com/rust-lang/rustlings/commit/1c6f7e4b7b9b3bd36f4da2bb2b69c549cc8bd913))
  - added progress info ([c0e3daac](https://github.com/rust-lang/rustlings/commit/c0e3daacaf6850811df5bc57fa43e0f249d5cfa4))

<a name="4.3.0"></a>

## 4.3.0 (2020-12-29)

#### Features

- Rewrite default out text ([44d39112](https://github.com/rust-lang/rustlings/commit/44d39112ff122b29c9793fe52e605df1612c6490))
- match exercise order to book chapters (#541) ([033bf119](https://github.com/rust-lang/rustlings/commit/033bf1198fc8bfce1b570e49da7cde010aa552e3))
- Crab? (#586) ([fa9f522b](https://github.com/rust-lang/rustlings/commit/fa9f522b7f043d7ef73a39f003a9272dfe72c4f4))
- add "rustlings list" command ([838f9f30](https://github.com/rust-lang/rustlings/commit/838f9f30083d0b23fd67503dcf0fbeca498e6647))
- **try_from_into:** remove duplicate annotation ([04f1d079](https://github.com/rust-lang/rustlings/commit/04f1d079aa42a2f49af694bc92c67d731d31a53f))

#### Bug Fixes

- update structs README ([bcf14cf6](https://github.com/rust-lang/rustlings/commit/bcf14cf677adb3a38a3ac3ca53f3c69f61153025))
- added missing exercises to info.toml ([90cfb6ff](https://github.com/rust-lang/rustlings/commit/90cfb6ff28377531bfc34acb70547bdb13374f6b))
- gives a bit more context to magic number ([30644c9a](https://github.com/rust-lang/rustlings/commit/30644c9a062b825c0ea89435dc59f0cad86b110e))
- **functions2:** Change signature to trigger precise error message: (#605) ([0ef95947](https://github.com/rust-lang/rustlings/commit/0ef95947cc30482e63a7045be6cc2fb6f6dcb4cc))
- **structs1:** Adjust wording (#573) ([9334783d](https://github.com/rust-lang/rustlings/commit/9334783da31d821cc59174fbe8320df95828926c))
- **try_from_into:**
  - type error ([4f4cfcf3](https://github.com/rust-lang/rustlings/commit/4f4cfcf3c36c8718c7c170c9c3a6935e6ef0618c))
  - Update description (#584) ([96347df9](https://github.com/rust-lang/rustlings/commit/96347df9df294f01153b29d9ad4ba361f665c755))
- **vec1:** Have test compare every element in a and v ([9b6c6293](https://github.com/rust-lang/rustlings/commit/9b6c629397b24b944f484f5b2bbd8144266b5695))

<a name="4.2.0"></a>

## 4.2.0 (2020-11-07)

#### Features

- Add HashMap exercises ([633c00cf](https://github.com/rust-lang/rustlings/commit/633c00cf8071e1e82959a3010452a32f34f29fc9))
- Add Vec exercises ([0c12fa31](https://github.com/rust-lang/rustlings/commit/0c12fa31c57c03c6287458a0a8aca7afd057baf6))
- **primitive_types6:** Add a test (#548) ([2b1fb2b7](https://github.com/rust-lang/rustlings/commit/2b1fb2b739bf9ad8d6b7b12af25fee173011bfc4))
- **try_from_into:** Add tests (#571) ([95ccd926](https://github.com/rust-lang/rustlings/commit/95ccd92616ae79ba287cce221101e0bbe4f68cdc))

#### Bug Fixes

- log error output when inotify limit is exceeded ([d61b4e5a](https://github.com/rust-lang/rustlings/commit/d61b4e5a13b44d72d004082f523fa1b6b24c1aca))
- more unique temp_file ([5643ef05](https://github.com/rust-lang/rustlings/commit/5643ef05bc81e4a840e9456f4406a769abbe1392))
- **installation:** Update the MinRustVersion ([21bfb2d4](https://github.com/rust-lang/rustlings/commit/21bfb2d4777429c87d8d3b5fbf0ce66006dcd034))
- **iterators2:** Update description (#578) ([197d3a3d](https://github.com/rust-lang/rustlings/commit/197d3a3d8961b2465579218a6749b2b2cefa8ddd))
- **primitive_types6:**
  - remove 'unused doc comment' warning ([472d8592](https://github.com/rust-lang/rustlings/commit/472d8592d65c8275332a20dfc269e7ac0d41bc88))
  - missing comma in test ([4fb230da](https://github.com/rust-lang/rustlings/commit/4fb230daf1251444fcf29e085cee222a91f8a37e))
- **quiz3:** Second test is for odd numbers, not even. (#553) ([18e0bfef](https://github.com/rust-lang/rustlings/commit/18e0bfef1de53071e353ba1ec5837002ff7290e6))

<a name="4.1.0"></a>

## 4.1.0 (2020-10-05)

#### Bug Fixes

- Update rustlings version in Cargo.lock ([1cc40bc9](https://github.com/rust-lang/rustlings/commit/1cc40bc9ce95c23d56f6d91fa1c4deb646231fef))
- **arc1:** index mod should equal thread count ([b4062ef6](https://github.com/rust-lang/rustlings/commit/b4062ef6993e80dac107c4093ea85166ad3ee0fa))
- **enums3:** Update Message::ChangeColor to take a tuple. (#457) ([4b6540c7](https://github.com/rust-lang/rustlings/commit/4b6540c71adabad647de8a09e57295e7c7c7d794))
- **exercises:** adding question mark to quiz2 ([101072ab](https://github.com/rust-lang/rustlings/commit/101072ab9f8c80b40b8b88cb06cbe38aca2481c5))
- **generics3:** clarify grade change ([47f7672c](https://github.com/rust-lang/rustlings/commit/47f7672c0307732056e7426e81d351f0dd7e22e5))
- **structs3:** Small adjustment of variable name ([114b54cb](https://github.com/rust-lang/rustlings/commit/114b54cbdb977234b39e5f180d937c14c78bb8b2))
- **using_as:** Add test so that proper type is returned. (#512) ([3286c5ec](https://github.com/rust-lang/rustlings/commit/3286c5ec19ea5fb7ded81d047da5f8594108a490))

#### Features

- Added iterators1.rs exercise ([9642f5a3](https://github.com/rust-lang/rustlings/commit/9642f5a3f686270a4f8f6ba969919ddbbc4f7fdd))
- Add ability to run rustlings on repl.it (#471) ([8f7b5bd0](https://github.com/rust-lang/rustlings/commit/8f7b5bd00eb83542b959830ef55192d2d76db90a))
- Add gitpod support (#473) ([4821a8be](https://github.com/rust-lang/rustlings/commit/4821a8be94af4f669042a06ab917934cfacd032f))
- Remind the user of the hint option (#425) ([816b1f5e](https://github.com/rust-lang/rustlings/commit/816b1f5e85d6cc6e72673813a85d0ada2a8f84af))
- Remind the user of the hint option (#425) ([9f61db5d](https://github.com/rust-lang/rustlings/commit/9f61db5dbe38538cf06571fcdd5f806e7901e83a))
- **cli:** Added 'cls' command to 'watch' mode (#474) ([4f2468e1](https://github.com/rust-lang/rustlings/commit/4f2468e14f574a93a2e9b688367b5752ed96ae7b))
- **try_from_into:** Add insufficient length test (#469) ([523d18b8](https://github.com/rust-lang/rustlings/commit/523d18b873a319f7c09262f44bd40e2fab1830e5))

<a name="4.0.0"></a>

## 4.0.0 (2020-07-08)

#### Breaking Changes

- Add a --nocapture option to display test harnesses' outputs ([8ad5f9bf](https://github.com/rust-lang/rustlings/commit/8ad5f9bf531a4848b1104b7b389a20171624c82f))
- Rename test to quiz, fixes #244 ([010a0456](https://github.com/rust-lang/rustlings/commit/010a04569282149cea7f7a76fc4d7f4c9f0f08dd))

#### Features

- Add traits README ([173bb141](https://github.com/rust-lang/rustlings/commit/173bb14140c5530cbdb59e53ace3991a99d804af))
- Add box1.rs exercise ([7479a473](https://github.com/rust-lang/rustlings/commit/7479a4737bdcac347322ad0883ca528c8675e720))
- Rewrite try_from_into (#393) ([763aa6e3](https://github.com/rust-lang/rustlings/commit/763aa6e378a586caae2d8d63755a85eeba227933))
- Add if2 exercise ([1da84b5f](https://github.com/rust-lang/rustlings/commit/1da84b5f7c489f65bd683c244f13c7d1ee812df0))
- Added exercise structs3.rs ([b66e2e09](https://github.com/rust-lang/rustlings/commit/b66e2e09622243e086a0f1258dd27e1a2d61c891))
- Add exercise variables6 covering const (#352) ([5999acd2](https://github.com/rust-lang/rustlings/commit/5999acd24a4f203292be36e0fd18d385887ec481))

#### Bug Fixes

- Change then to than ([ddd98ad7](https://github.com/rust-lang/rustlings/commit/ddd98ad75d3668fbb10eff74374148aa5ed2344d))
- rename quiz1 to tests1 in info (#420) ([0dd1c6ca](https://github.com/rust-lang/rustlings/commit/0dd1c6ca6b389789e0972aa955fe17aa15c95f29))
- fix quiz naming inconsistency (#421) ([5563adbb](https://github.com/rust-lang/rustlings/commit/5563adbb890587fc48fbbc9c4028642687f1e85b))
- confine the user further in variable exercises ([06ef4cc6](https://github.com/rust-lang/rustlings/commit/06ef4cc654e75d22a526812919ee49b8956280bf))
- update iterator and macro text for typos and clarity ([95900828](https://github.com/rust-lang/rustlings/commit/959008284834bece0196a01e17ac69a7e3590116))
- update generics2 closes #362 ([964c974a](https://github.com/rust-lang/rustlings/commit/964c974a0274199d755073b917c2bc5da0c9b4f1))
- confusing comment in conversions/try_from_into.rs ([c9e4f2cf](https://github.com/rust-lang/rustlings/commit/c9e4f2cfb4c48d0b7451263cfb43b9426438122d))
- **arc1:** Passively introduce attributes (#429) ([113cdae2](https://github.com/rust-lang/rustlings/commit/113cdae2d4e4c55905e8056ad326ede7fd7de356))
- **box1:** fix comment typo (#426) ([bb2ca251](https://github.com/rust-lang/rustlings/commit/bb2ca251106b27a7272d9a30872904dd1376654c))
- **errorsn:** Try harder to confine the user. (#388) ([2b20c8a0](https://github.com/rust-lang/rustlings/commit/2b20c8a0f5774d07c58d110d75879f33fc6273b5))
- **from_into.rs:** typo ([a901499e](https://github.com/rust-lang/rustlings/commit/a901499ededd3ce1995164700514fe4e9a0373ea))
- **generics2:** Guide students to the answer (#430) ([e6bd8021](https://github.com/rust-lang/rustlings/commit/e6bd8021d9a7dd06feebc30c9d5f953901d7b419))
- **installation:**
  - Provide a backup git reference when tag can't be curl ([9e4fb100](https://github.com/rust-lang/rustlings/commit/9e4fb1009f1c9e3433915c03e22c2af422e5c5fe))
  - Check if python is available while checking for git,rustc and cargo ([9cfb617d](https://github.com/rust-lang/rustlings/commit/9cfb617d5b0451b4b51644a1298965390cda9884))
- **option1:**
  - Don't add only zeros to the numbers array ([cce6a442](https://github.com/rust-lang/rustlings/commit/cce6a4427718724a9096800754cd3abeca6a1580))
  - Add cast to usize, as it is confusing in the context of an exercise about Option ([f6cffc7e](https://github.com/rust-lang/rustlings/commit/f6cffc7e487b42f15a6f958e49704c93a8d4465b))
- **option2:** Add TODO to comments (#400) ([10967bce](https://github.com/rust-lang/rustlings/commit/10967bce57682812dc0891a9f9757da1a9d87404))
- **options1:** Add hint about Array Initialization (#389) ([9f75554f](https://github.com/rust-lang/rustlings/commit/9f75554f2a30295996f03f0160b98c0458305502))
- **test2:** name of type String and &str (#394) ([d6c0a688](https://github.com/rust-lang/rustlings/commit/d6c0a688e6a96f93ad60d540d4b326f342fc0d45))
- **variables6:** minor typo (#419) ([524e17df](https://github.com/rust-lang/rustlings/commit/524e17df10db95f7b90a0f75cc8997182a8a4094))

<a name="3.0.0"></a>

## 3.0.0 (2020-04-11)

#### Breaking Changes

- make "compile" exercises print output (#278) ([3b6d5c](https://github.com/fmoko/rustlings/commit/3b6d5c3aaa27a242a832799eb66e96897d26fde3))

#### Bug Fixes

- **primitive_types:** revert primitive_types4 (#296) ([b3a3351e](https://github.com/rust-lang/rustlings/commit/b3a3351e8e6a0bdee07077d7b0382953821649ae))
- **run:** compile clippy exercise files (#295) ([3ab084a4](https://github.com/rust-lang/rustlings/commit/3ab084a421c0f140ae83bf1fc3f47b39342e7373))
- **conversions:**
  - add additional test to meet exercise rules (#284) ([bc22ec3](https://github.com/fmoko/rustlings/commit/bc22ec382f843347333ef1301fc1bad773657f38))
  - remove duplicate not done comment (#292) ([dab90f](https://github.com/fmoko/rustlings/commit/dab90f7b91a6000fe874e3d664f244048e5fa342))
- don't hardcode documentation version for traits (#288) ([30e6af](https://github.com/fmoko/rustlings/commit/30e6af60690c326fb5d3a9b7335f35c69c09137d))

#### Features

- add Option2 exercise (#290) ([86b5c08b](https://github.com/rust-lang/rustlings/commit/86b5c08b9bea1576127a7c5f599f5752072c087d))
- add exercise for option (#282) ([135e5d47](https://github.com/rust-lang/rustlings/commit/135e5d47a7c395aece6f6022117fb20c82f2d3d4))
- add new exercises for generics (#280) ([76be5e4e](https://github.com/rust-lang/rustlings/commit/76be5e4e991160f5fd9093f03ee2ba260e8f7229))
- **ci:** add buildkite config ([b049fa2c](https://github.com/rust-lang/rustlings/commit/b049fa2c84dba0f0c8906ac44e28fd45fba51a71))

<a name="2.2.1"></a>

### 2.2.1 (2020-02-27)

#### Bug Fixes

- Re-add cloning the repo to install scripts ([3d9b03c5](https://github.com/rust-lang/rustlings/commit/3d9b03c52b8dc51b140757f6fd25ad87b5782ef5))

#### Features

- Add clippy lints (#269) ([1e2fd9c9](https://github.com/rust-lang/rustlings/commit/1e2fd9c92f8cd6e389525ca1a999fca4c90b5921))

<a name="2.2.0"></a>

## 2.2.0 (2020-02-25)

#### Bug Fixes

- Update deps to version compatable with aarch64-pc-windows (#263) ([19a93428](https://github.com/rust-lang/rustlings/commit/19a93428b3c73d994292671f829bdc8e5b7b3401))
- **docs:**
  - Added a necessary step to Windows installation process (#242) ([3906efcd](https://github.com/rust-lang/rustlings/commit/3906efcd52a004047b460ed548037093de3f523f))
  - Fixed mangled sentence from book; edited for clarity (#266) ([ade52ff](https://github.com/rust-lang/rustlings/commit/ade52ffb739987287ddd5705944c8777705faed9))
  - Updated iterators readme to account for iterators4 exercise (#273) ([bec8e3a](https://github.com/rust-lang/rustlings/commit/bec8e3a644cbd88db1c73ea5f1d8a364f4a34016))
- **installation:** make fatal errors more obvious (#272) ([17d0951e](https://github.com/rust-lang/rustlings/commit/17d0951e66fda8e11b204d5c4c41a0d5e22e78f7))
- **iterators2:**
  - Remove reference to missing iterators2.rs (#245) ([419f7797](https://github.com/rust-lang/rustlings/commit/419f7797f294e4ce6a2b883199731b5bde77d262))
- **as_ref_mut:** Enable a test and improve per clippy's suggestion (#256) ([dfdf809](https://github.com/rust-lang/rustlings/commit/dfdf8093ebbd4145864995627b812780de52f902))
- **tests1:**
  - Change test command ([fe10e06c](https://github.com/rust-lang/rustlings/commit/fe10e06c3733ddb4a21e90d09bf79bfe618e97ce)
  - Correct test command in tests1.rs comment (#263) ([39fa7ae](https://github.com/rust-lang/rustlings/commit/39fa7ae8b70ad468da49b06f11b2383135a63bcf))

#### Features

- Add variables5.rs exercise (#264) ([0c73609e](https://github.com/rust-lang/rustlings/commit/0c73609e6f2311295e95d6f96f8c747cfc4cba03))
- Show a completion message when watching (#253) ([d25ee55a](https://github.com/rust-lang/rustlings/commit/d25ee55a3205882d35782e370af855051b39c58c))
- Add type conversion and parsing exercises (#249) ([0c85dc11](https://github.com/rust-lang/rustlings/commit/0c85dc1193978b5165491b99cc4922caf8d14a65))
- Created consistent money unit (#258) ([fd57f8f](https://github.com/rust-lang/rustlings/commit/fd57f8f2c1da2af8ddbebbccec214e6f40f4dbab))
- Enable test for exercise test4 (#276) ([8b971ff](https://github.com/rust-lang/rustlings/commit/8b971ffab6079a706ac925f5917f987932b55c07))
- Added traits exercises (#274 but specifically #216, which originally added
  this :heart:) ([b559cdd](https://github.com/rust-lang/rustlings/commit/b559cdd73f32c0d0cfc1feda39f82b3e3583df17))

<a name="2.1.0"></a>

## 2.1.0 (2019-11-27)

#### Bug Fixes

- add line numbers in several exercises and hints ([b565c4d3](https://github.com/rust-lang/rustlings/commit/b565c4d3e74e8e110bef201a082fa1302722a7c3))
- **arc1:** Fix some words in the comment ([c42c3b21](https://github.com/rust-lang/rustlings/commit/c42c3b2101df9164c8cd7bb344def921e5ba3e61))
- **enums:** Add link to chapter on pattern syntax (#242) ([615ce327](https://github.com/rust-lang/rustlings/commit/615ce3279800c56d89f19d218ccb7ef576624feb))
- **primitive_types4:**
  - update outdated hint ([4c5189df](https://github.com/rust-lang/rustlings/commit/4c5189df2bdd9a231f6b2611919ba5aa14da0d3f))
  - update outdated comment ([ded2c034](https://github.com/rust-lang/rustlings/commit/ded2c034ba93fa1e3c2c2ea16b83abc1a57265e8))
- **strings2:** update line number in hint ([a09f684f](https://github.com/rust-lang/rustlings/commit/a09f684f05c58d239a6fc59ec5f81c2533e8b820))
- **variables1:** Correct wrong word in comment ([fda5a470](https://github.com/rust-lang/rustlings/commit/fda5a47069e0954f16a04e8e50945e03becb71a5))

#### Features

- **watch:** show hint while watching ([8143d57b](https://github.com/rust-lang/rustlings/commit/8143d57b4e88c51341dd4a18a14c536042cc009c))

<a name="2.0.0"></a>

## 2.0.0 (2019-11-12)

#### Bug Fixes

- **default:** Clarify the installation procedure ([c371b853](https://github.com/rust-lang/rustlings/commit/c371b853afa08947ddeebec0edd074b171eeaae0))
- **info:** Fix trailing newlines for hints ([795b6e34](https://github.com/rust-lang/rustlings/commit/795b6e348094a898e9227a14f6232f7bb94c8d31))
- **run:** make `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Breaking Changes

- Refactor hint system ([9bdb0a12](https://github.com/rust-lang/rustlings/commit/9bdb0a12e45a8e9f9f6a4bd4a9c172c5376c7f60))
- improve `watch` execution mode ([2cdd6129](https://github.com/rust-lang/rustlings/commit/2cdd61294f0d9a53775ee24ad76435bec8a21e60))
- Index exercises by name ([627cdc07](https://github.com/rust-lang/rustlings/commit/627cdc07d07dfe6a740e885e0ddf6900e7ec336b))
- **run:** makes `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Features

- **cli:** check for rustc before doing anything ([36a033b8](https://github.com/rust-lang/rustlings/commit/36a033b87a6549c1e5639c908bf7381c84f4f425))
- **hint:** Add test for hint ([ce9fa6eb](https://github.com/rust-lang/rustlings/commit/ce9fa6ebbfdc3e7585d488d9409797285708316f))

<a name="1.5.1"></a>

### 1.5.1 (2019-11-11)

#### Bug Fixes

- **errors3:** Update hint ([dcfb427b](https://github.com/rust-lang/rustlings/commit/dcfb427b09585f0193f0a294443fdf99f11c64cb), closes [#185](https://github.com/rust-lang/rustlings/issues/185))
- **if1:** Remove `return` reference ([ad03d180](https://github.com/rust-lang/rustlings/commit/ad03d180c9311c0093e56a3531eec1a9a70cdb45))
- **strings:** Move Strings before Structs ([6dcecb38](https://github.com/rust-lang/rustlings/commit/6dcecb38a4435593beb87c8e12d6314143631482), closes [#204](https://github.com/rust-lang/rustlings/issues/204))
- **structs1:** Remove misleading comment ([f72e5a8f](https://github.com/rust-lang/rustlings/commit/f72e5a8f05568dde04eaeac10b9a69872f21cb37))
- **threads:** Move Threads behind SLT ([fbe91a67](https://github.com/rust-lang/rustlings/commit/fbe91a67a482bfe64cbcdd58d06ba830a0f39da3), closes [#205](https://github.com/rust-lang/rustlings/issues/205))
- **watch:** clear screen before each `verify()` ([3aff590](https://github.com/rust-lang/rustlings/commit/3aff59085586c24196a547c2693adbdcf4432648))

<a name="1.5.0"></a>

## 1.5.0 (2019-11-09)

#### Bug Fixes

- **test1:** Rewrite logic ([79a56942](https://github.com/rust-lang/rustlings/commit/79a569422c8309cfc9e4aed25bf4ab3b3859996b))
- **installation:** Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
- **iterators:** Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
- **iterators2:** Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
- **option1:**
  - Fix arguments passed to assert! macro (#222) ([4c2cf6da](https://github.com/rust-lang/rustlings/commit/4c2cf6da755efe02725e05ecc3a303304c10a6da))
  - Fix arguments passed to assert! macro ([ead4f7af](https://github.com/rust-lang/rustlings/commit/ead4f7af9e10e53418efdde5c359159347282afd))
  - Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
- **primitive_types4:** Fail on a slice covering the wrong area ([5b1e673c](https://github.com/rust-lang/rustlings/commit/5b1e673cec1658afc4ebbbc800213847804facf5))
- **readme:** http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
- **test1:**
  - Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))
  - renamed function name to snake case closes #180 ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))

#### Features

- Add enums exercises ([dc150321](https://github.com/rust-lang/rustlings/commit/dc15032112fc485226a573a18139e5ce928b1755))
- Added exercise for struct update syntax ([1c4c8764](https://github.com/rust-lang/rustlings/commit/1c4c8764ed118740cd4cee73272ddc6cceb9d959))
- **iterators2:** adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

<a name="1.4.1"></a>

### 1.4.1 (2019-08-13)

#### Bug Fixes

- **iterators2:** Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
- **option1:** Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
- **test1:** Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))

<a name="1.4.0"></a>

## 1.4.0 (2019-07-13)

#### Bug Fixes

- **installation:** Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
- **iterators:** Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
- **readme:** http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
- **test1:** renamed function name to snake case ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))
- **cli:** Check if changed exercise file exists before calling verify ([ba85ca3](https://github.com/rust-lang/rustlings/commit/ba85ca32c4cfc61de46851ab89f9c58a28f33c88))
- **structs1:** Fix the irrefutable let pattern warning ([cc6a141](https://github.com/rust-lang/rustlings/commit/cc6a14104d7c034eadc98297eaaa972d09c50b1f))

#### Features

- **changelog:** Use clog for changelogs ([34e31232](https://github.com/rust-lang/rustlings/commit/34e31232dfddde284a341c9609b33cd27d9d5724))
- **iterators2:** adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

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
