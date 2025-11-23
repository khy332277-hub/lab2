use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let src_path = "src_file.txt";
    let dst_path = "dst_file.txt";

    // 원본 파일 열기
    let mut src_file = File::open(src_path)?;
    
    // 대상 파일 생성
    let mut dst_file = File::create(dst_path)?;

    // 버퍼로 읽고 쓰기
    let mut buffer = Vec::new();
    src_file.read_to_end(&mut buffer)?; // 파일 전체 읽기
    dst_file.write_all(&buffer)?;       // 대상 파일에 쓰기

    println!("{} → {} 복사 완료!", src_path, dst_path);
    Ok(())
}

