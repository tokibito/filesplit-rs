use std::fmt;

/// ファイル分割・結合処理で発生するエラーを定義
#[derive(Debug)]
pub enum FileSplitError {
    /// 無効なサイズ指定
    InvalidSize(String),
    /// ファイルが見つからない
    FileNotFound(String),
    /// I/Oエラー
    IoError(std::io::Error),
    /// 分割ファイルが見つからない
    NoSplitFiles(String),
}

/// エラーメッセージの表示形式を定義
impl fmt::Display for FileSplitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSplitError::InvalidSize(msg) => write!(f, "無効なサイズ指定: {msg}"),
            FileSplitError::FileNotFound(path) => write!(f, "ファイルが見つかりません: {path}"),
            FileSplitError::IoError(err) => write!(f, "I/Oエラー: {err}"),
            FileSplitError::NoSplitFiles(base) => write!(
                f,
                "分割ファイルが見つかりません: {base}.001, {base}.002, ..."
            ),
        }
    }
}

impl std::error::Error for FileSplitError {}

/// std::io::ErrorからFileSplitErrorへの自動変換を実装
impl From<std::io::Error> for FileSplitError {
    fn from(err: std::io::Error) -> Self {
        FileSplitError::IoError(err)
    }
}

/// このクレートで使用するResult型のエイリアス
pub type Result<T> = std::result::Result<T, FileSplitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // 各エラー型の表示形式をテスト
        let err = FileSplitError::InvalidSize("abc".to_string());
        assert_eq!(err.to_string(), "無効なサイズ指定: abc");

        let err = FileSplitError::FileNotFound("/path/to/file".to_string());
        assert_eq!(err.to_string(), "ファイルが見つかりません: /path/to/file");

        let err = FileSplitError::NoSplitFiles("test.txt".to_string());
        assert_eq!(
            err.to_string(),
            "分割ファイルが見つかりません: test.txt.001, test.txt.002, ..."
        );
    }

    #[test]
    fn test_from_io_error() {
        // std::io::Errorからの変換をテスト
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test error");
        let file_split_error: FileSplitError = io_error.into();

        match file_split_error {
            FileSplitError::IoError(_) => (), // 正しく変換された
            _ => panic!("予期しないエラー型"),
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        // エラー型がSendとSyncを実装していることを確認
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<FileSplitError>();
        assert_sync::<FileSplitError>();
    }
}
