<div class="oranda-hide">

# rustlings 🦀❤️ Rust繁中簡學!

</div>

這是 Rustlings 的繁體中文翻譯版本。此版本包含了所有練習的中文註釋和文檔翻譯 **（還在努力...）**。

## 安裝

使用以下命令安裝此翻譯版本：

```sh
cargo install --git https://github.com/TimLai666/rustlings-zh-TW
```
<hr>

歡迎來到 `rustlings`。這個專案包含一些小練習，可幫助您習慣閱讀和編寫 Rust 程式碼。包括閱讀和回應編譯器訊息！

另外，對於 Rust 初學者，還有以下資源可以參考：

- [The Book](https://doc.rust-lang.org/book/index.html) - 最全面的 Rust 學習資源，但有時有點理論性。建議您可以搭配 Rustlings 一起使用！
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - 透過小練習來學習 Rust！和`rustlings`類似，但是是線上的。

## 新手上路

_Note: 如果您使用的是 MacOS，請確保您已安裝 Xcode 及其開發人員工具，方法是輸入 `xcode-select --install`._
_Note: 如果您使用的是 Linux，請確保您已安裝 gcc。 Deb: `sudo apt install gcc`. Yum: `sudo yum -y install gcc`._

您需要安裝 Rust。 您可以至 <https://rustup.rs> 取得它。這也將安裝 Cargo（Rust 的套件/專案管理器）。

## MacOS/Linux

Just run:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
```

Or if you want it to be installed to a different path:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

### Nix

Basically: Clone the repository at the latest tag, finally run `nix develop` or `nix-shell`.

```bash
# find out the latest version at https://github.com/rust-lang/rustlings/releases/latest (on edit 5.6.1)
git clone -b 5.6.1 --depth 1 https://github.com/rust-lang/rustlings
cd rustlings
# if nix version > 2.3
nix develop
# if nix version <= 2.3
nix-shell
```

## Windows

In PowerShell (Run as Administrator), set `ExecutionPolicy` to `RemoteSigned`:

```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then, you can run:

```ps1
Start-BitsTransfer -Source https://raw.githubusercontent.com/rust-lang/rustlings/main/install.ps1 -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it. Keep in mind that this works best in PowerShell, and any other terminals may give you errors.

If you get a permission denied message, you might have to exclude the directory where you cloned Rustlings in your antivirus.

## Browser

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/rust-lang/rustlings)

[![Open Rustlings On Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new/?repo=rust-lang%2Frustlings&ref=main)

## Manually

Basically: Clone the repository at the latest tag, run `cargo install --locked --path .`.

```bash
# find out the latest version at https://github.com/rust-lang/rustlings/releases/latest (on edit 5.6.1)
git clone -b 5.6.1 --depth 1 https://github.com/rust-lang/rustlings
cd rustlings
cargo install --locked --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:

```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## 進行練習

練習題按照主題排序，您可以在子目錄 `rustlings/exercises/<topic>` 中找到它們。每個主題都有一個附加的解說文件，其中包含一些資源，可幫助您了解該主題。我們強烈建議您在開始之前先看一下它們。

這些任務很簡單。大多數練習包含錯誤，導致它們無法編譯，你的任務就是修復這些錯誤！有些練習也會作為測試運行，但 rustlings 會以相同的方式處理它們。要按照推薦順序執行這些練習，請執行以下命令：

```bash
rustlings watch
```

以上命令將使 rustlings 嘗試按照預定順序（我們認為最適合新手的順序）驗證每個練習的完成情況。每當你更改 `exercises/` 目錄中的文件時，它都會自動重新運行。如果你只想運行一次，可以使用以下命令：

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

Or simply use the following command to run the next unsolved exercise in the course:

```bash
rustlings run next
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

```bash
rustlings hint myExercise1
```

You can also get the hint for the next unsolved exercise with the following command:

```bash
rustlings hint next
```

To check your progress, you can run the following command:

```bash
rustlings list
```

## Testing yourself

After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in `exercises/quizN.rs`.

## Enabling `rust-analyzer`

Run the command `rustlings lsp` which will generate a `rust-project.json` at the root of the project, this allows [rust-analyzer](https://rust-analyzer.github.io/) to parse each exercise.

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use! Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## 解除安裝 Rustlings

如果您想從系統中刪除 Rustlings，有兩個步驟。首先，您需要刪除安裝腳本為您建立的練習資料夾：

```bash
rm -rf rustlings # or your custom folder name, if you chose and or renamed it
```

Second, run `cargo uninstall` to remove the `rustlings` binary:

```bash
cargo uninstall rustlings
```

Now you should be done!

## Contributing

See [CONTRIBUTING.md](https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md).

## Contributors ✨

Thanks goes to the wonderful people listed in [AUTHORS.md](https://github.com/rust-lang/rustlings/blob/main/AUTHORS.md) 🎉
