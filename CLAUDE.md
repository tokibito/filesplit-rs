# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

filesplit-rsは、ファイルを指定したバイト数ごとに分割するRust製のコマンドラインツールです。分割されたファイルは、元のファイル名に連番（.001、.002など）を付けて保存されます。

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
- `cargo run -- -s <size> <filepath>` - デバッグモードで実行
- `./target/debug/filesplit-rs -s <size> <filepath>` - ビルド済みバイナリを直接実行

## アーキテクチャ

現在、プロジェクトは初期段階にあり、main.rsのみが存在しています。実装予定の機能：

1. コマンドライン引数の解析（-sオプションでサイズ指定、ファイルパス指定）
2. ファイル読み込みとバッファリング
3. 指定サイズごとのファイル分割ロジック
4. 連番付きファイル名での出力（.001、.002形式）

## 注意事項

- Rust 2024 エディションを使用
- 現在依存関係は追加されていない（今後、コマンドライン引数解析用にclapなどの追加が予想される）