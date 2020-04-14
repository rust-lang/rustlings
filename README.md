![crab pet](https://i.imgur.com/LbZJgmm.gif)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-48-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

# rustlings 🦀❤️ [![Build status](https://badge.buildkite.com/7af93d81dc522c67a1ec8e33ff5705861b1cb36360b774807f.svg)](https://buildkite.com/mokou/rustlings)

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there's several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._

_Note: If you have Xcode 10+ installed, you also need to install the package file found at `/Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://git.io/rustlings | bash
# Or if you want it to be installed to a different path:
curl -L https://git.io/rustlings | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

## Windows

First, set `ExecutionPolicy` to `RemoteSigned`:

```ps
Set-ExecutionPolicy RemoteSigned
```

Then, you can run:

```ps
Invoke-WebRequest https://git.io/rustlings-win | Select-Object -ExpandProperty Content | Out-File $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it.

## Manually

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/rust-lang/rustlings
cd rustlings
git checkout tags/3.0.0 # or whatever the latest version is (find out at https://github.com/rust-lang/rustlings/releases/latest)
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:
```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keep it from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

``` bash
rustlings hint myExercise1
```

## Testing yourself

After every couple of sections, there will be a test that'll test your knowledge on a bunch of sections at once. These tests are found in `exercises/testN.rs`.

## Completion

Rustlings isn't done; there are a couple of sections that are very experimental and don't have proper documentation. These include:

- Errors (`exercises/errors/`)
- Option (`exercises/option/`)
- Result (`exercises/result/`)
- Move Semantics (could still be improved, `exercises/move_semantics/`)

Additionally, we could use exercises on a couple of topics:

- Structs
- Better ownership stuff
- `impl`
- ??? probably more

If you are interested in improving or adding new ones, please feel free to contribute! Read on for more information :)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="http://carol-nichols.com"><img src="https://avatars2.githubusercontent.com/u/193874?v=4" width="100px;" alt=""/><br /><sub><b>Carol (Nichols &#124;&#124; Goulding)</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=carols10cents" title="Code">💻</a> <a href="#content-carols10cents" title="Content">🖋</a></td>
    <td align="center"><a href="https://twitter.com/QuietMisdreavus"><img src="https://avatars2.githubusercontent.com/u/5217170?v=4" width="100px;" alt=""/><br /><sub><b>QuietMisdreavus</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=QuietMisdreavus" title="Code">💻</a> <a href="#content-QuietMisdreavus" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/robertlugg"><img src="https://avatars0.githubusercontent.com/u/6054540?v=4" width="100px;" alt=""/><br /><sub><b>Robert M Lugg</b></sub></a><br /><a href="#content-robertlugg" title="Content">🖋</a></td>
    <td align="center"><a href="https://hynek.me/about/"><img src="https://avatars3.githubusercontent.com/u/41240?v=4" width="100px;" alt=""/><br /><sub><b>Hynek Schlawack</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=hynek" title="Code">💻</a></td>
    <td align="center"><a href="https://spacekookie.de"><img src="https://avatars0.githubusercontent.com/u/7669898?v=4" width="100px;" alt=""/><br /><sub><b>Katharina Fey</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=spacekookie" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/lukabavdaz"><img src="https://avatars0.githubusercontent.com/u/9624558?v=4" width="100px;" alt=""/><br /><sub><b>lukabavdaz</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=lukabavdaz" title="Code">💻</a> <a href="#content-lukabavdaz" title="Content">🖋</a></td>
    <td align="center"><a href="http://vestera.as"><img src="https://avatars2.githubusercontent.com/u/4187449?v=4" width="100px;" alt=""/><br /><sub><b>Erik Vesteraas</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=evestera" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Delet0r"><img src="https://avatars1.githubusercontent.com/u/23195618?v=4" width="100px;" alt=""/><br /><sub><b>delet0r</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=Delet0r" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="http://phinary.ca"><img src="https://avatars1.githubusercontent.com/u/10522375?v=4" width="100px;" alt=""/><br /><sub><b>Shaun Bennett</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=shaunbennett" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/abagshaw"><img src="https://avatars2.githubusercontent.com/u/8594541?v=4" width="100px;" alt=""/><br /><sub><b>Andrew Bagshaw</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=abagshaw" title="Code">💻</a></td>
    <td align="center"><a href="https://ai6ua.net/"><img src="https://avatars2.githubusercontent.com/u/175578?v=4" width="100px;" alt=""/><br /><sub><b>Kyle Isom</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=kisom" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ColinPitrat"><img src="https://avatars3.githubusercontent.com/u/1541863?v=4" width="100px;" alt=""/><br /><sub><b>Colin Pitrat</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=ColinPitrat" title="Code">💻</a></td>
    <td align="center"><a href="https://zacanger.com"><img src="https://avatars3.githubusercontent.com/u/12520493?v=4" width="100px;" alt=""/><br /><sub><b>Zac Anger</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=zacanger" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/mgeier"><img src="https://avatars1.githubusercontent.com/u/705404?v=4" width="100px;" alt=""/><br /><sub><b>Matthias Geier</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=mgeier" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/cjpearce"><img src="https://avatars1.githubusercontent.com/u/3453268?v=4" width="100px;" alt=""/><br /><sub><b>Chris Pearce</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=cjpearce" title="Code">💻</a></td>
    <td align="center"><a href="https://yvan-sraka.github.io"><img src="https://avatars2.githubusercontent.com/u/705213?v=4" width="100px;" alt=""/><br /><sub><b>Yvan Sraka</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=yvan-sraka" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/dendi239"><img src="https://avatars3.githubusercontent.com/u/16478650?v=4" width="100px;" alt=""/><br /><sub><b>Denys Smirnov</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=dendi239" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/eddyp"><img src="https://avatars2.githubusercontent.com/u/123772?v=4" width="100px;" alt=""/><br /><sub><b>eddyp</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=eddyp" title="Code">💻</a></td>
    <td align="center"><a href="http://about.me/BrianKung"><img src="https://avatars1.githubusercontent.com/u/2836167?v=4" width="100px;" alt=""/><br /><sub><b>Brian Kung</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=briankung" title="Code">💻</a> <a href="#content-briankung" title="Content">🖋</a></td>
    <td align="center"><a href="https://rcousineau.gitlab.io"><img src="https://avatars3.githubusercontent.com/u/281039?v=4" width="100px;" alt=""/><br /><sub><b>Russell</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=miller-time" title="Code">💻</a></td>
    <td align="center"><a href="http://danwilhelm.com"><img src="https://avatars3.githubusercontent.com/u/6137185?v=4" width="100px;" alt=""/><br /><sub><b>Dan Wilhelm</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=danwilhelm" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/Jesse-Cameron"><img src="https://avatars3.githubusercontent.com/u/3723654?v=4" width="100px;" alt=""/><br /><sub><b>Jesse</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=Jesse-Cameron" title="Code">💻</a> <a href="#content-Jesse-Cameron" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/MrFroop"><img src="https://avatars3.githubusercontent.com/u/196700?v=4" width="100px;" alt=""/><br /><sub><b>Fredrik Jambrén</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=MrFroop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/petemcfarlane"><img src="https://avatars3.githubusercontent.com/u/3472717?v=4" width="100px;" alt=""/><br /><sub><b>Pete McFarlane</b></sub></a><br /><a href="#content-petemcfarlane" title="Content">🖋</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/nkanderson"><img src="https://avatars0.githubusercontent.com/u/4128825?v=4" width="100px;" alt=""/><br /><sub><b>nkanderson</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=nkanderson" title="Code">💻</a> <a href="#content-nkanderson" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/ajaxm"><img src="https://avatars0.githubusercontent.com/u/13360138?v=4" width="100px;" alt=""/><br /><sub><b>Ajax M</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=ajaxm" title="Documentation">📖</a></td>
    <td align="center"><a href="https://dylnuge.com"><img src="https://avatars2.githubusercontent.com/u/118624?v=4" width="100px;" alt=""/><br /><sub><b>Dylan Nugent</b></sub></a><br /><a href="#content-Dylnuge" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/vyaslav"><img src="https://avatars0.githubusercontent.com/u/1385427?v=4" width="100px;" alt=""/><br /><sub><b>vyaslav</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=vyaslav" title="Code">💻</a> <a href="#content-vyaslav" title="Content">🖋</a></td>
    <td align="center"><a href="https://join.sfxd.org"><img src="https://avatars1.githubusercontent.com/u/17297466?v=4" width="100px;" alt=""/><br /><sub><b>George</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=gdoenlen" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/nyxtom"><img src="https://avatars2.githubusercontent.com/u/222763?v=4" width="100px;" alt=""/><br /><sub><b>Thomas Holloway</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=nyxtom" title="Code">💻</a> <a href="#content-nyxtom" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/workingjubilee"><img src="https://avatars1.githubusercontent.com/u/46493976?v=4" width="100px;" alt=""/><br /><sub><b>Jubilee</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=workingjubilee" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/WofWca"><img src="https://avatars1.githubusercontent.com/u/39462442?v=4" width="100px;" alt=""/><br /><sub><b>WofWca</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=WofWca" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/jrvidal"><img src="https://avatars0.githubusercontent.com/u/1636604?v=4" width="100px;" alt=""/><br /><sub><b>Roberto Vidal</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=jrvidal" title="Code">💻</a> <a href="https://github.com/fmoko/rustlings/commits?author=jrvidal" title="Documentation">📖</a> <a href="#ideas-jrvidal" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-jrvidal" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/jensim"><img src="https://avatars0.githubusercontent.com/u/3663856?v=4" width="100px;" alt=""/><br /><sub><b>Jens</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=jensim" title="Documentation">📖</a></td>
    <td align="center"><a href="http://rahatah.me/d"><img src="https://avatars3.githubusercontent.com/u/3174006?v=4" width="100px;" alt=""/><br /><sub><b>Rahat Ahmed</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=rahatarmanahmed" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/AbdouSeck"><img src="https://avatars2.githubusercontent.com/u/6490055?v=4" width="100px;" alt=""/><br /><sub><b>Abdou Seck</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=AbdouSeck" title="Code">💻</a> <a href="#content-AbdouSeck" title="Content">🖋</a> <a href="https://github.com/fmoko/rustlings/pulls?q=is%3Apr+reviewed-by%3AAbdouSeck" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://codehearts.com"><img src="https://avatars0.githubusercontent.com/u/2885412?v=4" width="100px;" alt=""/><br /><sub><b>Katie</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=codehearts" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Socratides"><img src="https://avatars3.githubusercontent.com/u/27732983?v=4" width="100px;" alt=""/><br /><sub><b>Socrates</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=Socratides" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/gnodarse"><img src="https://avatars3.githubusercontent.com/u/46761795?v=4" width="100px;" alt=""/><br /><sub><b>gnodarse</b></sub></a><br /><a href="#content-gnodarse" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/harrisonmetz"><img src="https://avatars1.githubusercontent.com/u/7883408?v=4" width="100px;" alt=""/><br /><sub><b>Harrison Metzger</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=harrisonmetz" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/TorbenJ"><img src="https://avatars2.githubusercontent.com/u/9077102?v=4" width="100px;" alt=""/><br /><sub><b>Torben Jonas</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=TorbenJ" title="Code">💻</a> <a href="#content-TorbenJ" title="Content">🖋</a></td>
    <td align="center"><a href="http://paulbissex.com/"><img src="https://avatars0.githubusercontent.com/u/641?v=4" width="100px;" alt=""/><br /><sub><b>Paul Bissex</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=pbx" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/sjmann"><img src="https://avatars0.githubusercontent.com/u/6589896?v=4" width="100px;" alt=""/><br /><sub><b>Steven Mann</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=sjmann" title="Code">💻</a> <a href="#content-sjmann" title="Content">🖋</a></td>
    <td align="center"><a href="https://smmdb.net/"><img src="https://avatars2.githubusercontent.com/u/5855071?v=4" width="100px;" alt=""/><br /><sub><b>Mario Reder</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=Tarnadas" title="Code">💻</a> <a href="#content-Tarnadas" title="Content">🖋</a></td>
    <td align="center"><a href="https://keybase.io/skim"><img src="https://avatars0.githubusercontent.com/u/47347?v=4" width="100px;" alt=""/><br /><sub><b>skim</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=sl4m" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/sanjaykdragon"><img src="https://avatars1.githubusercontent.com/u/10261698?v=4" width="100px;" alt=""/><br /><sub><b>Sanjay K</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=sanjaykdragon" title="Code">💻</a> <a href="#content-sanjaykdragon" title="Content">🖋</a></td>
    <td align="center"><a href="http://www.rohanjain.in"><img src="https://avatars1.githubusercontent.com/u/343499?v=4" width="100px;" alt=""/><br /><sub><b>Rohan Jain</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=crodjer" title="Code">💻</a></td>
    <td align="center"><a href="https://www.saidaspen.se"><img src="https://avatars1.githubusercontent.com/u/7727687?v=4" width="100px;" alt=""/><br /><sub><b>Said Aspen</b></sub></a><br /><a href="https://github.com/fmoko/rustlings/commits?author=saidaspen" title="Code">💻</a> <a href="#content-saidaspen" title="Content">🖋</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
