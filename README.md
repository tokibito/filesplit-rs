# filesplit

[![CI](https://github.com/tokibito/filesplit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tokibito/filesplit-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

ファイル分割を行うコマンドツールです。

## 対応OS

- **Linux** (x86_64, aarch64)
  - Ubuntu 20.04以降
  - CentOS 7以降
  - その他のglibc 2.17以降のディストリビューション
- **Windows** (x86_64)
  - Windows 10以降
  - Windows Server 2016以降
- **macOS** (x86_64, Apple Silicon)
  - macOS 10.15 (Catalina)以降

## ビルド手順

cargoコマンドを予めセットアップしておきます。

### デバッグビルド

```bash
cargo build
```

### リリースビルド

最適化を有効にした本番用ビルドを作成します：

```bash
cargo build --release
```

ビルドされたバイナリは `target/release/filesplit-rs` (Linux/macOS) または `target/release/filesplit-rs.exe` (Windows) に生成されます。

### クロスコンパイル

他のプラットフォーム向けにビルドする場合：

```bash
# Linux musl (静的リンク版)
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Windows (Linuxから)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS Apple Silicon (Intel Macから)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

## デバッグビルドの実行

```
cargo run -- -s <size> <filepath>
```

`<filepath>` で指定したファイルを `<size>` で指定したバイト数ごとに分割し、 `<filepath>.001` `<filepath>.002` ... のようなファイルパスで保存します。

## ファイルのマージ（結合）

```
cargo run -- -m <filepath>
```

`-m` オプションを使用すると、分割されたファイルを結合して元のファイルに復元します。`<filepath>` には元のファイル名（拡張子なし）を指定します。`<filepath>.001`、`<filepath>.002` ... のような連番ファイルを自動的に検出して結合します。

## 実行環境

### 最低限必要なファイル

リリースビルドしたアプリケーションを実行するには、以下のファイルのみが必要です：

#### Linux
- `filesplit-rs` (実行ファイル)
- 依存ライブラリ：
  - 通常版: システムのglibc (2.17以降)
  - musl版: 依存なし（完全静的リンク）

#### Windows
- `filesplit-rs.exe` (実行ファイル)
- 依存ライブラリ：
  - Visual C++ ランタイム（通常はOS標準でインストール済み）
  - Windows 10以降では追加インストール不要

#### macOS
- `filesplit-rs` (実行ファイル)
- 依存ライブラリ：システム標準ライブラリのみ

### インストール方法

#### 方法1: バイナリを直接配置

```bash
# Linux/macOS
sudo cp target/release/filesplit-rs /usr/local/bin/
sudo chmod +x /usr/local/bin/filesplit-rs

# Windows (管理者権限のコマンドプロンプト)
copy target\release\filesplit-rs.exe C:\Windows\System32\
```

#### 方法2: 任意のディレクトリに配置してPATHを通す

```bash
# Linux/macOS
mkdir -p ~/bin
cp target/release/filesplit-rs ~/bin/
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows
# 環境変数PATHに実行ファイルのディレクトリを追加
```

### 使用例

```bash
# ファイルを1MBごとに分割
filesplit-rs -s 1048576 largefile.dat

# 分割されたファイルを結合
filesplit-rs -m largefile.dat
```

### トラブルシューティング

#### Linux で「共有ライブラリが見つかりません」エラーが出る場合

```bash
# 依存ライブラリを確認
ldd filesplit-rs

# glibcバージョンを確認
ldd --version

# 解決策: musl版を使用
# x86_64-unknown-linux-musl ターゲットでビルドしたバイナリは
# 依存関係がないため、どの環境でも動作します
```

#### Windows で実行できない場合

1. Visual C++ 再頒布可能パッケージをインストール
2. または、MinGW版（x86_64-pc-windows-gnu）を使用

### Docker での実行

最小限のDockerイメージで実行する例：

```dockerfile
FROM scratch
COPY filesplit-rs /
ENTRYPOINT ["/filesplit-rs"]
```

※ musl版のビルドが必要です
