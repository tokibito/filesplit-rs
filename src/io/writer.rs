use crate::error::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// 分割ファイルの書き込みを管理する構造体
pub struct SplitFileWriter {
    /// 元ファイルのパス
    base_path: PathBuf,
    /// 現在の分割ファイルインデックス
    current_index: usize,
}

impl SplitFileWriter {
    /// 新しいSplitFileWriterを作成する
    /// 
    /// # 引数
    /// * `base_path` - 元ファイルのパス
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
            current_index: 1,
        }
    }

    /// 次の分割ファイルにデータを書き込む
    /// 
    /// # 引数
    /// * `data` - 書き込むデータ
    /// 
    /// # 動作
    /// ファイル名に連番（.001, .002, ...）を付けて保存し、
    /// インデックスをインクリメントする
    pub fn write_next_file(&mut self, data: &[u8]) -> Result<()> {
        let file_path = self.get_split_file_path(self.current_index);
        let file = File::create(&file_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()?;
        self.current_index += 1;
        Ok(())
    }

    /// 指定されたインデックスの分割ファイルパスを生成する
    /// 
    /// # 引数
    /// * `index` - ファイルのインデックス（1から始まる）
    /// 
    /// # 戻り値
    /// 例: "file.txt" -> "file.txt.001"
    pub fn get_split_file_path(&self, index: usize) -> PathBuf {
        let mut path = self.base_path.clone();
        let file_name = format!(
            "{}.{:03}",
            path.file_name().unwrap_or_default().to_string_lossy(),
            index
        );
        path.set_file_name(file_name);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_split_file_writer_creation() {
        // SplitFileWriterの作成をテスト
        let writer = SplitFileWriter::new(Path::new("/tmp/test.txt"));
        assert_eq!(writer.current_index, 1);
        assert_eq!(writer.base_path, PathBuf::from("/tmp/test.txt"));
    }

    #[test]
    fn test_get_split_file_path() {
        // 分割ファイルパスの生成をテスト
        let writer = SplitFileWriter::new(Path::new("/tmp/test.txt"));
        
        assert_eq!(
            writer.get_split_file_path(1),
            PathBuf::from("/tmp/test.txt.001")
        );
        assert_eq!(
            writer.get_split_file_path(10),
            PathBuf::from("/tmp/test.txt.010")
        );
        assert_eq!(
            writer.get_split_file_path(999),
            PathBuf::from("/tmp/test.txt.999")
        );
    }

    #[test]
    fn test_write_next_file() {
        // ファイルへの書き込みをテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        let mut writer = SplitFileWriter::new(&base_path);
        
        // 最初のファイルに書き込み
        writer.write_next_file(b"First chunk").unwrap();
        let first_file = temp_dir.path().join("test.txt.001");
        assert!(first_file.exists());
        assert_eq!(fs::read(&first_file).unwrap(), b"First chunk");
        
        // 次のファイルに書き込み
        writer.write_next_file(b"Second chunk").unwrap();
        let second_file = temp_dir.path().join("test.txt.002");
        assert!(second_file.exists());
        assert_eq!(fs::read(&second_file).unwrap(), b"Second chunk");
        
        // インデックスが正しく増加していることを確認
        assert_eq!(writer.current_index, 3);
    }

    #[test]
    fn test_write_empty_data() {
        // 空のデータの書き込みをテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        let mut writer = SplitFileWriter::new(&base_path);
        writer.write_next_file(b"").unwrap();
        
        let file = temp_dir.path().join("test.txt.001");
        assert!(file.exists());
        assert_eq!(fs::read(&file).unwrap(), b"");
    }

    #[test]
    fn test_write_large_data() {
        // 大きなデータの書き込みをテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        let mut writer = SplitFileWriter::new(&base_path);
        let large_data = vec![b'X'; 10000];
        
        writer.write_next_file(&large_data).unwrap();
        
        let file = temp_dir.path().join("test.txt.001");
        assert!(file.exists());
        assert_eq!(fs::read(&file).unwrap().len(), 10000);
    }
}
