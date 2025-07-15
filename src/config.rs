use crate::error::{FileSplitError, Result};
use std::path::PathBuf;

/// プログラムの動作モード
pub enum Mode {
    /// ファイル分割モード（分割サイズを指定）
    Split { size: usize },
    /// ファイル結合モード
    Merge,
}

/// プログラムの設定情報
pub struct Config {
    /// 対象ファイルのパス
    pub file_path: PathBuf,
    /// 動作モード
    pub mode: Mode,
}

impl Config {
    /// 文字列からサイズをパースする
    pub fn parse_size(size_str: &str) -> Result<usize> {
        size_str.parse::<usize>().map_err(|_| {
            FileSplitError::InvalidSize(format!("'{}' は有効な数値ではありません", size_str))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_size_valid() {
        // 有効なサイズのパースをテスト
        assert_eq!(Config::parse_size("1024").unwrap(), 1024);
        assert_eq!(Config::parse_size("0").unwrap(), 0);
        assert_eq!(Config::parse_size("9999999").unwrap(), 9999999);
    }

    #[test]
    fn test_parse_size_invalid() {
        // 無効なサイズのパースをテスト
        assert!(Config::parse_size("abc").is_err());
        assert!(Config::parse_size("-100").is_err());
        assert!(Config::parse_size("12.34").is_err());
        assert!(Config::parse_size("").is_err());
        
        // エラーメッセージの確認
        let err = Config::parse_size("abc").unwrap_err();
        match err {
            FileSplitError::InvalidSize(msg) => {
                assert!(msg.contains("abc"));
                assert!(msg.contains("有効な数値ではありません"));
            }
            _ => panic!("予期しないエラー型"),
        }
    }

    #[test]
    fn test_mode_enum() {
        // Mode列挙型の動作確認
        let split_mode = Mode::Split { size: 1024 };
        match split_mode {
            Mode::Split { size } => assert_eq!(size, 1024),
            _ => panic!("予期しないモード"),
        }

        let merge_mode = Mode::Merge;
        match merge_mode {
            Mode::Merge => (), // OK
            _ => panic!("予期しないモード"),
        }
    }

    #[test]
    fn test_config_struct() {
        // Config構造体の作成と使用をテスト
        let config = Config {
            file_path: PathBuf::from("/tmp/test.txt"),
            mode: Mode::Split { size: 2048 },
        };
        
        assert_eq!(config.file_path, PathBuf::from("/tmp/test.txt"));
        match config.mode {
            Mode::Split { size } => assert_eq!(size, 2048),
            _ => panic!("予期しないモード"),
        }
    }
}
