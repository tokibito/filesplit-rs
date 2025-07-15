use crate::error::{FileSplitError, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// 分割されたファイルを結合する構造体
pub struct Merger;

impl Merger {
    /// 新しいMergerを作成する
    pub fn new() -> Self {
        Self
    }

    /// 分割されたファイルを結合する
    /// 
    /// # 引数
    /// * `base_path` - 結合後のファイルパス
    /// 
    /// # 動作
    /// base_path.001, base_path.002, ...の形式のファイルを
    /// 順番に読み込んで、base_pathに結合する
    pub fn merge_files(&self, base_path: &Path) -> Result<()> {
        // 出力ファイルを作成
        let output_file = File::create(base_path)?;
        let mut writer = BufWriter::new(output_file);
        
        // 分割ファイルのインデックスとフラグを初期化
        let mut index = 1;
        let mut found_any = false;

        // すべての分割ファイルを順番に処理
        loop {
            // 現在のインデックスの分割ファイルパスを生成
            let split_path = Self::get_split_file_path(base_path, index);

            // ファイルが存在しない場合の処理
            if !split_path.exists() {
                if !found_any {
                    // 1つも分割ファイルが見つからなかった場合はエラー
                    return Err(FileSplitError::NoSplitFiles(
                        base_path.to_string_lossy().to_string(),
                    ));
                }
                // 連番が途切れたら終了
                break;
            }

            found_any = true;
            
            // 分割ファイルを読み込んで出力ファイルに書き込む
            let mut reader = BufReader::new(File::open(&split_path)?);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
            writer.write_all(&buffer)?;

            // 次のファイルへ
            index += 1;
        }

        // バッファをフラッシュして書き込みを完了
        writer.flush()?;
        Ok(())
    }

    /// 指定されたインデックスの分割ファイルパスを生成する
    /// 
    /// # 引数
    /// * `base_path` - 元ファイルのパス
    /// * `index` - ファイルのインデックス（1から始まる）
    /// 
    /// # 戻り値
    /// 例: "file.txt", 1 -> "file.txt.001"
    fn get_split_file_path(base_path: &Path, index: usize) -> PathBuf {
        let mut path = base_path.to_path_buf();
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
    use crate::error::FileSplitError;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_merger_creation() {
        // Mergerの作成をテスト
        let merger = Merger::new();
        // Mergerは状態を持たないので、作成できることだけを確認
        let _ = merger;
    }

    #[test]
    fn test_get_split_file_path() {
        // 分割ファイルパスの生成をテスト
        let base = Path::new("/tmp/test.txt");
        
        assert_eq!(
            Merger::get_split_file_path(base, 1),
            PathBuf::from("/tmp/test.txt.001")
        );
        assert_eq!(
            Merger::get_split_file_path(base, 100),
            PathBuf::from("/tmp/test.txt.100")
        );
    }

    #[test]
    fn test_merge_files_success() {
        // 正常な結合処理のテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        // 分割ファイルを作成
        let mut file1 = fs::File::create(temp_dir.path().join("test.txt.001")).unwrap();
        file1.write_all(b"First part ").unwrap();
        
        let mut file2 = fs::File::create(temp_dir.path().join("test.txt.002")).unwrap();
        file2.write_all(b"Second part ").unwrap();
        
        let mut file3 = fs::File::create(temp_dir.path().join("test.txt.003")).unwrap();
        file3.write_all(b"Third part").unwrap();
        
        // ファイルを結合
        let merger = Merger::new();
        merger.merge_files(&base_path).unwrap();
        
        // 結合されたファイルの内容を確認
        let content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(content, "First part Second part Third part");
    }

    #[test]
    fn test_merge_files_with_gap() {
        // 連番に欠けがある場合のテスト（.001, .002, .004があるが.003がない）
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        // 分割ファイルを作成（.003をスキップ）
        let mut file1 = fs::File::create(temp_dir.path().join("test.txt.001")).unwrap();
        file1.write_all(b"Part 1 ").unwrap();
        
        let mut file2 = fs::File::create(temp_dir.path().join("test.txt.002")).unwrap();
        file2.write_all(b"Part 2").unwrap();
        
        let mut file4 = fs::File::create(temp_dir.path().join("test.txt.004")).unwrap();
        file4.write_all(b" Part 4").unwrap();
        
        // ファイルを結合（.003で停止するはず）
        let merger = Merger::new();
        merger.merge_files(&base_path).unwrap();
        
        // 結合されたファイルの内容を確認（.001と.002のみ）
        let content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(content, "Part 1 Part 2");
    }

    #[test]
    fn test_merge_no_split_files() {
        // 分割ファイルが存在しない場合のエラーテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        let merger = Merger::new();
        let result = merger.merge_files(&base_path);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            FileSplitError::NoSplitFiles(_) => (), // 期待通りのエラー
            _ => panic!("予期しないエラー型"),
        }
    }

    #[test]
    fn test_merge_single_file() {
        // 分割ファイルが1つだけの場合のテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        let mut file1 = fs::File::create(temp_dir.path().join("test.txt.001")).unwrap();
        file1.write_all(b"Only one part").unwrap();
        
        let merger = Merger::new();
        merger.merge_files(&base_path).unwrap();
        
        let content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(content, "Only one part");
    }

    #[test]
    fn test_merge_empty_files() {
        // 空の分割ファイルを結合するテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        // 空のファイルを作成
        fs::File::create(temp_dir.path().join("test.txt.001")).unwrap();
        fs::File::create(temp_dir.path().join("test.txt.002")).unwrap();
        
        let merger = Merger::new();
        merger.merge_files(&base_path).unwrap();
        
        let content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_merge_large_index() {
        // 大きなインデックス番号のテスト
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test.txt");
        
        // .999のファイルを作成
        let mut file = fs::File::create(temp_dir.path().join("test.txt.999")).unwrap();
        file.write_all(b"File 999").unwrap();
        
        // .001がないのでエラーになるはず
        let merger = Merger::new();
        let result = merger.merge_files(&base_path);
        assert!(result.is_err());
    }
}
