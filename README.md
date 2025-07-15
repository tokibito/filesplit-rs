# filesplit

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
