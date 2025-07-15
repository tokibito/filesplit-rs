use crate::error::Result;
use crate::io::{BufferedReader, SplitFileWriter};
use std::path::Path;

/// ファイル分割処理を行う構造体
pub struct Splitter {
    /// 分割サイズ（バイト単位）
    chunk_size: usize,
}

impl Splitter {
    /// 新しいSplitterを作成する
    /// 
    /// # 引数
    /// * `chunk_size` - 分割サイズ（バイト単位）
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// 指定されたファイルを分割する
    /// 
    /// # 引数
    /// * `file_path` - 分割するファイルのパス
    /// 
    /// # 動作
    /// ファイルをchunk_sizeバイトごとに分割し、
    /// 元のファイル名.001, .002, ...の形式で保存する
    pub fn split_file(&self, file_path: &Path) -> Result<()> {
        // ファイルリーダーとライターを初期化
        let mut reader = BufferedReader::new(file_path)?;
        let mut writer = SplitFileWriter::new(file_path);
        
        // 読み込み用バッファを確保
        let mut buffer = vec![0u8; self.chunk_size];

        // ファイルの終端まで読み込みと書き込みを繰り返す
        loop {
            // バッファサイズ分のデータを読み込む
            let bytes_read = reader.read_chunk(&mut buffer)?;
            
            // ファイルの終端に達したら終了
            if bytes_read == 0 {
                break;
            }

            // 読み込んだデータを分割ファイルに書き込む
            writer.write_next_file(&buffer[..bytes_read])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_splitter_creation() {
        // Splitterの作成をテスト
        let splitter = Splitter::new(1024);
        assert_eq!(splitter.chunk_size, 1024);
    }

    #[test]
    fn test_split_file_exact_chunks() {
        // ちょうど分割サイズで割り切れるファイルのテスト
        let temp_dir = TempDir::new().unwrap();
        let mut temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        
        // 30バイトのデータを書き込む（10バイトずつ3つに分割）
        temp_file.write_all(b"0123456789abcdefghij1234567890").unwrap();
        temp_file.flush().unwrap();
        
        let splitter = Splitter::new(10);
        splitter.split_file(temp_file.path()).unwrap();
        
        // 分割ファイルが作成されたことを確認
        let base_name = temp_file.path().file_name().unwrap();
        let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
        let file2 = temp_dir.path().join(format!("{}.002", base_name.to_string_lossy()));
        let file3 = temp_dir.path().join(format!("{}.003", base_name.to_string_lossy()));
        
        assert!(file1.exists());
        assert!(file2.exists());
        assert!(file3.exists());
        
        // 各ファイルの内容を確認
        assert_eq!(fs::read(&file1).unwrap(), b"0123456789");
        assert_eq!(fs::read(&file2).unwrap(), b"abcdefghij");
        assert_eq!(fs::read(&file3).unwrap(), b"1234567890");
    }

    #[test]
    fn test_split_file_with_remainder() {
        // 分割サイズで割り切れないファイルのテスト
        let temp_dir = TempDir::new().unwrap();
        let mut temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        
        // 25バイトのデータを書き込む（10バイトずつ2つ + 5バイトの余り）
        temp_file.write_all(b"0123456789abcdefghij12345").unwrap();
        temp_file.flush().unwrap();
        
        let splitter = Splitter::new(10);
        splitter.split_file(temp_file.path()).unwrap();
        
        // 分割ファイルが作成されたことを確認
        let base_name = temp_file.path().file_name().unwrap();
        let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
        let file2 = temp_dir.path().join(format!("{}.002", base_name.to_string_lossy()));
        let file3 = temp_dir.path().join(format!("{}.003", base_name.to_string_lossy()));
        
        assert!(file1.exists());
        assert!(file2.exists());
        assert!(file3.exists());
        
        // 各ファイルの内容を確認
        assert_eq!(fs::read(&file1).unwrap(), b"0123456789");
        assert_eq!(fs::read(&file2).unwrap(), b"abcdefghij");
        assert_eq!(fs::read(&file3).unwrap(), b"12345");
    }

    #[test]
    fn test_split_empty_file() {
        // 空のファイルを分割するテスト
        let temp_dir = TempDir::new().unwrap();
        let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        
        let splitter = Splitter::new(10);
        splitter.split_file(temp_file.path()).unwrap();
        
        // 分割ファイルが作成されていないことを確認
        let base_name = temp_file.path().file_name().unwrap();
        let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
        assert!(!file1.exists());
    }

    #[test]
    fn test_split_small_file() {
        // 分割サイズより小さいファイルのテスト
        let temp_dir = TempDir::new().unwrap();
        let mut temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        
        temp_file.write_all(b"small").unwrap();
        temp_file.flush().unwrap();
        
        let splitter = Splitter::new(100);
        splitter.split_file(temp_file.path()).unwrap();
        
        // 1つの分割ファイルだけが作成されることを確認
        let base_name = temp_file.path().file_name().unwrap();
        let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
        let file2 = temp_dir.path().join(format!("{}.002", base_name.to_string_lossy()));
        
        assert!(file1.exists());
        assert!(!file2.exists());
        assert_eq!(fs::read(&file1).unwrap(), b"small");
    }

    #[test]
    fn test_split_nonexistent_file() {
        // 存在しないファイルを分割しようとした場合のエラーテスト
        let splitter = Splitter::new(1024);
        let result = splitter.split_file(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }
}
