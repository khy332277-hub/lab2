use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("탐색할 디렉토리 경로를 입력하세요:");
    let path = read_line().trim().to_string();

    let path = Path::new(&path);
    if path.exists() && path.is_dir() {
        println!("{} 디렉토리 내용:", path.display());
        list_dir_recursive(path, 0);
    } else {
        println!("유효한 디렉토리가 아닙니다.");
    }
}

fn list_dir_recursive(path: &Path, level: usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                let indent = "  ".repeat(level);
                if file_path.is_dir() {
                    println!("{}[DIR] {}", indent, file_path.display());
                    // 재귀 호출
                    list_dir_recursive(&file_path, level + 1);
                } else {
                    println!("{}{}", indent, file_path.display());
                }
            }
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 오류");
    input
}

