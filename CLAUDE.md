# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

filesplit-rsは、ファイルを指定したバイト数ごとに分割・結合するRust製のコマンドラインツールです。分割されたファイルは、元のファイル名に連番（.001、.002など）を付けて保存されます。また、分割されたファイルを元の状態に結合する機能も提供します。

## 開発コマンド

### ビルドとコンパイル
- `cargo build` - デバッグビルドを実行
- `cargo build --release` - リリースビルドを実行
- `cargo check` - コンパイルチェック（ビルドせずに構文をチェック）

### テスト
- `cargo test` - 全テストを実行
- `cargo test <test_name>` - 特定のテストを実行

### 品質チェック
- `cargo clippy` - Rustのリンターを実行
- `cargo fmt` - コードフォーマット
- `cargo fmt --check` - フォーマットチェック（変更なし）

### プログラムの実行
- `cargo run -- -s <size> <filepath>` - ファイル分割（デバッグモード）
- `cargo run -- -m <filepath>` - ファイル結合（デバッグモード）
- `./target/debug/filesplit-rs -s <size> <filepath>` - ファイル分割（ビルド済みバイナリ）
- `./target/debug/filesplit-rs -m <filepath>` - ファイル結合（ビルド済みバイナリ）

## 実装済み機能

### コマンドライン引数
- `-s, --size <SIZE>`: 分割サイズ指定（バイト単位）
- `-m, --merge`: ファイル結合モード
- `<FILE_PATH>`: 対象ファイルパス（必須）
- `-h, --help`: ヘルプメッセージ表示

### 主要機能
1. **ファイル分割**: 指定されたバイト数でファイルを分割し、.001, .002形式で保存
2. **ファイル結合**: 分割されたファイルを検出し、元のファイルに復元
3. **エラーハンドリング**: 日本語のエラーメッセージで適切なエラー処理
4. **バッファリング**: 大きなファイルの効率的な処理

## 実装済みモジュール構造

```
src/
├── main.rs           # エントリーポイント、CLI実行
├── cli.rs           # コマンドライン引数の解析（clapを使用）
├── splitter.rs      # ファイル分割のコアロジック
├── merger.rs        # ファイル結合のコアロジック
├── io/              # I/O関連モジュール
│   ├── mod.rs       # モジュールの公開API定義
│   ├── reader.rs    # BufferedReaderによるファイル読み込み
│   └── writer.rs    # SplitFileWriterによる分割ファイル書き込み
├── config.rs        # Config構造体とModeによる設定管理
└── error.rs         # FileSplitError型の定義
```

### 各モジュールの実装内容

- **main.rs**: プログラムのエントリーポイント、モード分岐、処理結果の表示
- **cli.rs**: `Cli`構造体によるclap引数パース、-sと-mの排他制御、Config生成
- **splitter.rs**: `Splitter`構造体、chunk_sizeごとのファイル分割実装
- **merger.rs**: `Merger`構造体、連番ファイルの検出と結合、存在チェック
- **io/reader.rs**: `BufferedReader`によるバッファ付き読み込み
- **io/writer.rs**: `SplitFileWriter`による連番ファイル生成と書き込み
- **config.rs**: `Config`構造体、`Mode`列挙型、サイズパース処理
- **error.rs**: `FileSplitError`列挙型、日本語エラーメッセージ、Result型エイリアス

## 実装の特徴

- **言語**: Rust 2024 エディション
- **依存関係**: 
  - `clap = { version = "4.5", features = ["derive"] }` - コマンドライン引数解析
  - `anyhow = "1.0"` - エラーハンドリング補助
- **コメント**: 日本語でのコメントが全モジュールに追加済み
- **エラーメッセージ**: すべて日本語で表示

## 開発時の注意点

- ファイル分割時は元のファイルを保持（上書きしない）
- 結合時は出力ファイルを上書きするため注意が必要
- 大きなファイルも効率的に処理できるようバッファリングを使用