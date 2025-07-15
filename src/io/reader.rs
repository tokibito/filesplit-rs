use crate::error::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// バッファ付きファイルリーダー
/// 大きなファイルを効率的に読み込むための構造体
pub struct BufferedReader {
    reader: BufReader<File>,
}

impl BufferedReader {
    /// 新しいBufferedReaderを作成する
    ///
    /// # 引数
    /// * `path` - 読み込むファイルのパス
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    /// ファイルから指定されたバッファサイズ分のデータを読み込む
    ///
    /// # 引数
    /// * `buffer` - 読み込んだデータを格納するバッファ
    ///
    /// # 戻り値
    /// 実際に読み込んだバイト数
    pub fn read_chunk(&mut self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_buffered_reader_read() {
        // 一時ファイルを作成してテスト
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, World!";
        temp_file.write_all(test_data).unwrap();
        temp_file.flush().unwrap();

        // BufferedReaderでファイルを読み込む
        let mut reader = BufferedReader::new(temp_file.path()).unwrap();
        let mut buffer = vec![0u8; 20];

        let bytes_read = reader.read_chunk(&mut buffer).unwrap();
        assert_eq!(bytes_read, test_data.len());
        assert_eq!(&buffer[..bytes_read], test_data);
    }

    #[test]
    fn test_buffered_reader_read_partial() {
        // 小さなバッファで部分的に読み込むテスト
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = b"1234567890";
        temp_file.write_all(test_data).unwrap();
        temp_file.flush().unwrap();

        let mut reader = BufferedReader::new(temp_file.path()).unwrap();
        let mut buffer = vec![0u8; 5];

        // 最初の5バイトを読み込む
        let bytes_read = reader.read_chunk(&mut buffer).unwrap();
        assert_eq!(bytes_read, 5);
        assert_eq!(&buffer[..bytes_read], b"12345");

        // 次の5バイトを読み込む
        let bytes_read = reader.read_chunk(&mut buffer).unwrap();
        assert_eq!(bytes_read, 5);
        assert_eq!(&buffer[..bytes_read], b"67890");

        // EOFに達したことを確認
        let bytes_read = reader.read_chunk(&mut buffer).unwrap();
        assert_eq!(bytes_read, 0);
    }

    #[test]
    fn test_buffered_reader_file_not_found() {
        // 存在しないファイルを開こうとした場合のエラーテスト
        let result = BufferedReader::new(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_buffered_reader_empty_file() {
        // 空のファイルを読み込むテスト
        let temp_file = NamedTempFile::new().unwrap();

        let mut reader = BufferedReader::new(temp_file.path()).unwrap();
        let mut buffer = vec![0u8; 10];

        let bytes_read = reader.read_chunk(&mut buffer).unwrap();
        assert_eq!(bytes_read, 0);
    }
}
