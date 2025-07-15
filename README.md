# filesplit

[![CI](https://github.com/tokibito/filesplit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tokibito/filesplit-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

ファイル分割を行うコマンドツールです。

## ビルド手順

cargoコマンドを予めセットアップしておきます。

```
cargo build
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
