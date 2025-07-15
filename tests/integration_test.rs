use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

/// 統合テスト用のヘルパー関数：コマンドを実行
fn run_command(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(&["run", "--"])
        .args(args)
        .output()
        .expect("コマンドの実行に失敗しました")
}

#[test]
fn test_split_and_merge_integration() {
    // 一時ディレクトリとファイルを作成
    let temp_dir = TempDir::new().unwrap();
    let mut temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
    
    // テストデータを作成（100バイト）
    let test_data = "0123456789".repeat(10);
    temp_file.write_all(test_data.as_bytes()).unwrap();
    temp_file.flush().unwrap();
    
    let file_path = temp_file.path().to_str().unwrap();
    
    // ファイルを30バイトずつに分割
    let output = run_command(&["-s", "30", file_path]);
    assert!(output.status.success(), "分割コマンドが失敗しました: {:?}", 
            String::from_utf8_lossy(&output.stderr));
    
    // 分割ファイルが作成されたことを確認
    let base_name = temp_file.path().file_name().unwrap();
    let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
    let file2 = temp_dir.path().join(format!("{}.002", base_name.to_string_lossy()));
    let file3 = temp_dir.path().join(format!("{}.003", base_name.to_string_lossy()));
    let file4 = temp_dir.path().join(format!("{}.004", base_name.to_string_lossy()));
    
    assert!(file1.exists(), "file1が存在しません");
    assert!(file2.exists(), "file2が存在しません");
    assert!(file3.exists(), "file3が存在しません");
    assert!(file4.exists(), "file4が存在しません");
    
    // 各ファイルのサイズを確認
    assert_eq!(fs::metadata(&file1).unwrap().len(), 30);
    assert_eq!(fs::metadata(&file2).unwrap().len(), 30);
    assert_eq!(fs::metadata(&file3).unwrap().len(), 30);
    assert_eq!(fs::metadata(&file4).unwrap().len(), 10);
    
    // 元のファイルを削除
    fs::remove_file(temp_file.path()).unwrap();
    
    // ファイルを結合
    let output = run_command(&["-m", file_path]);
    assert!(output.status.success(), "結合コマンドが失敗しました: {:?}", 
            String::from_utf8_lossy(&output.stderr));
    
    // 結合されたファイルの内容を確認
    let merged_content = fs::read_to_string(temp_file.path()).unwrap();
    assert_eq!(merged_content, test_data);
}

#[test]
fn test_split_empty_file() {
    // 空のファイルを分割するテスト
    let temp_dir = TempDir::new().unwrap();
    let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = run_command(&["-s", "100", file_path]);
    assert!(output.status.success());
    
    // 分割ファイルが作成されていないことを確認
    let base_name = temp_file.path().file_name().unwrap();
    let file1 = temp_dir.path().join(format!("{}.001", base_name.to_string_lossy()));
    assert!(!file1.exists());
}

#[test]
fn test_merge_nonexistent_files() {
    // 存在しない分割ファイルを結合しようとするテスト
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.txt");
    
    let output = run_command(&["-m", nonexistent_path.to_str().unwrap()]);
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("分割ファイルが見つかりません"));
}

#[test]
fn test_invalid_size_argument() {
    // 無効なサイズ引数のテスト
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = run_command(&["-s", "abc", file_path]);
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("有効な数値ではありません"));
}

#[test]
fn test_missing_arguments() {
    // 引数が足りない場合のテスト
    let output = run_command(&[]);
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("FILE_PATH") || stderr.contains("required"));
}

#[test]
fn test_help_option() {
    // ヘルプオプションのテスト
    let output = run_command(&["--help"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ファイル分割・結合ツール"));
    assert!(stdout.contains("-s"));
    assert!(stdout.contains("-m"));
}

#[test]
fn test_split_binary_file() {
    // バイナリファイルの分割・結合テスト
    let temp_dir = TempDir::new().unwrap();
    let mut temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
    
    // バイナリデータを作成
    let binary_data: Vec<u8> = (0..=255).collect();
    temp_file.write_all(&binary_data).unwrap();
    temp_file.flush().unwrap();
    
    let file_path = temp_file.path().to_str().unwrap();
    
    // ファイルを分割
    let output = run_command(&["-s", "100", file_path]);
    assert!(output.status.success());
    
    // 元のファイルを削除
    fs::remove_file(temp_file.path()).unwrap();
    
    // ファイルを結合
    let output = run_command(&["-m", file_path]);
    assert!(output.status.success());
    
    // 結合されたファイルの内容を確認
    let merged_content = fs::read(temp_file.path()).unwrap();
    assert_eq!(merged_content, binary_data);
}