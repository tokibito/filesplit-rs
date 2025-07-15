// モジュールの宣言
mod cli;
mod config;
mod error;
mod io;
mod merger;
mod splitter;

use config::Mode;
use error::Result;
use std::process;

/// メイン関数
/// 
/// コマンドライン引数を解析し、ファイルの分割または結合を実行する
fn main() {
    if let Err(e) = run() {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // コマンドライン引数を解析
    let config = cli::Cli::parse_args()?;

    // 動作モードに応じて処理を分岐
    match config.mode {
        Mode::Split { size } => {
            // 分割モードの処理
            
            // ファイルの存在確認
            if !config.file_path.exists() {
                return Err(error::FileSplitError::FileNotFound(
                    config.file_path.to_string_lossy().to_string(),
                ));
            }

            // 分割処理の開始を通知
            println!("ファイルを分割しています: {}", config.file_path.display());
            println!("分割サイズ: {} バイト", size);

            // ファイル分割を実行
            let splitter = splitter::Splitter::new(size);
            splitter.split_file(&config.file_path)?;

            println!("分割が完了しました。");
        }
        Mode::Merge => {
            // 結合モードの処理
            
            // 結合処理の開始を通知
            println!("ファイルを結合しています: {}", config.file_path.display());

            // ファイル結合を実行
            let merger = merger::Merger::new();
            merger.merge_files(&config.file_path)?;

            println!("結合が完了しました。");
        }
    }

    Ok(())
}
