use crate::config::{Config, Mode};
use crate::error::Result;
use clap::Parser;
use std::path::PathBuf;

/// コマンドライン引数の定義
#[derive(Parser, Debug)]
#[command(name = "filesplit-rs")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = format!("ファイル分割・結合ツール v{}", env!("CARGO_PKG_VERSION")),
    long_about = None
)]
pub struct Cli {
    /// 分割サイズを指定（-mオプションと同時に使用不可）
    #[arg(
        short = 's',
        long = "size",
        help = "分割サイズ（バイト）",
        conflicts_with = "merge"
    )]
    size: Option<String>,

    /// ファイル結合モードを指定
    #[arg(short = 'm', long = "merge", help = "ファイルを結合する")]
    merge: bool,

    /// 対象ファイルのパス
    #[arg(help = "対象ファイルパス")]
    file_path: PathBuf,
}

impl Cli {
    /// コマンドライン引数を解析してConfigを生成する
    ///
    /// # 戻り値
    /// 解析結果のConfig、またはエラー
    pub fn parse_args() -> Result<Config> {
        // clapを使用して引数を解析
        let cli = Cli::parse();

        // 動作モードを決定
        let mode = if cli.merge {
            // 結合モード
            Mode::Merge
        } else if let Some(size_str) = cli.size {
            // 分割モード（サイズをパース）
            let size = Config::parse_size(&size_str)?;
            Mode::Split { size }
        } else {
            // -sまたは-mのどちらかが必須
            return Err(crate::error::FileSplitError::InvalidSize(
                "分割モードでは -s オプションでサイズを指定してください".to_string(),
            ));
        };

        Ok(Config {
            file_path: cli.file_path,
            mode,
        })
    }
}
