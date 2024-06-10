<div class="oranda-hide">

# rustlings 🦀❤️ Rust繁中簡學!

</div>

> ### *這是 Rustlings 的繁體中文翻譯版本。此版本包含了所有練習的中文註釋和文檔翻譯 **（還在努力...）**。*

歡迎來到 `rustlings`。這個專案包含一些小練習，可幫助您習慣閱讀和編寫 Rust 程式碼。包括閱讀和回應編譯器訊息！

另外，對於 Rust 初學者，還有以下資源可以參考：

- [The Book](https://doc.rust-lang.org/book/index.html) - 最全面的 Rust 學習資源，但有時有點理論性。建議您可以搭配 Rustlings 一起使用！
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - 透過小練習來學習 Rust！和 `rustlings` 類似，但是是線上的。

## 新手上路

_Note: 如果您使用的是 MacOS，請確保您已安裝 Xcode 及其開發人員工具，方法是輸入 `xcode-select --install`._
_Note: 如果您使用的是 Linux，請確保您已安裝 gcc。 Deb: `sudo apt install gcc`. Yum: `sudo yum -y install gcc`._

您需要安裝 Rust。 您可以至 <https://rustup.rs> 取得它。這也將安裝 Cargo（Rust 的套件/專案管理器）。

## 安裝

> 如果您的電腦還沒安裝 Git，請先安裝。
> ### 安裝 Git
> #### macOS
> 1. 安裝 Homebrew（如果還沒有安裝）：<br>
> 打開終端，運行以下命令來安裝 Homebrew：
> ```sh
> /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
> ```
> 2. 使用 Homebrew 安裝 Git：
> ```sh
> brew install git
> ```
> #### Windows
> 可以從 Git 官網下載安裝程序：
> 1. 前往 [Git for Windows](https://gitforwindows.org/) 下載最新版本的安裝程序。
> 2. 運行下載的安裝程序，按照提示完成安裝。
> #### Linux
> 可以使用系統的包管理器來安裝 Git。以下是一些常見的 Linux 發行版的安裝命令：
> * Ubuntu/Debian：
> ```sh
> sudo apt update
> sudo apt install git
> ```
> * Fedora：
> ```sh
> sudo dnf install git
> ```
> * Arch Linux：
> ```sh
> sudo pacman -S git
> ```

安裝好 Git 之後，請依照以下順序執行命令安裝這個翻譯版本的庫：

**1. 打開 `終端機` 或 `命令提示字元` 並導航到你希望克隆倉庫的目錄。例如，如果你希望將倉庫克隆到桌面，可以運行：**
  ```sh
  cd ~/Desktop
  ```
**2. 執行克隆命令：**
  ```sh
  git clone https://github.com/TimLai666/rustlings-zh-TW.git
  ```
**3. 安裝相關套件：**
  ```sh
  cargo install --git https://github.com/TimLai666/rustlings-zh-TW
  ```

## 進行練習

**1. 進入倉庫目錄：**
  ```sh
  cd rustlings-zh-TW
  ```
**2. 運行 Rustlings 命令開始練習，例如：**
  ```sh
  rustlings watch
  ```

練習題按照主題排序，您可以在子目錄 `rustlings-zh-TW/exercises/<主題>` 中找到它們。每個主題都有一個附加的解說文件，其中包含一些資源，可幫助您了解該主題。我們強烈建議您在開始之前先看一下它們。

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
