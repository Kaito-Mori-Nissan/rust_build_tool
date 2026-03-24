use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

/// ソースファイルがターゲットファイルより新しければtrueを返す
fn needs_recompile(source_file: &str, target_file: &str) -> bool {
    let target_metadata = match fs::metadata(target_file) {
        Ok(meta) => meta,
        Err(_) => return true, // ターゲットが存在しない場合はコンパイルが必要
    };

    let source_metadata = fs::metadata(source_file)
        .expect(&format!("エラー: ソースファイル '{}' が見つかりません", source_file));

    let target_time = target_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let source_time = source_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    source_time > target_time
}

fn main() {
    let out_dir = "out";

    // 1. 出力先ディレクトリ(out/)がなければ作成する
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("出力ディレクトリの作成に失敗しました");
        println!("出力ディレクトリ '{}' を作成しました", out_dir);
    }

    // 2. ファイルパスの定義
    let source = "c_project/main.c";

    // ※Windows環境で実行する場合は "out/main.exe" に変更してください
    #[cfg(target_os = "windows")]
    let target = "out/main.exe";
    #[cfg(not(target_os = "windows"))]
    let target = "out/main";

    // 3. タイムスタンプを比較してコンパイルを実行
    if needs_recompile(source, target) {
        println!("'{}' をコンパイルしています...", source);

        let status = Command::new("gcc")
            .arg(source)
            .arg("-o")
            .arg(target)
            .status()
            .expect("gccコマンドの実行に失敗しました。gccにパスが通っているか確認してください。");

        if status.success() {
            println!("ビルド成功: 実行ファイル '{}' が生成されました", target);
        } else {
            println!("ビルド失敗");
        }
    } else {
        println!("'{}' は最新です。コンパイルをスキップします。", source);
    }
}

