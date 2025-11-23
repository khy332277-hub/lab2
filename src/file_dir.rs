use std::fs;
use std::io;
use std::path::Path;

fn main() {
    loop {
        println!("\n파일/디렉토리 실습 메뉴:");
        println!("1. 파일 생성 및 쓰기");
        println!("2. 파일 읽기");
        println!("3. 디렉토리 생성");
        println!("4. 디렉토리 내 파일 목록");
        println!("5. 종료");
        println!("선택하세요 (1-5):");

        let choice = read_line().trim().to_string();

        match choice.as_str() {
            "1" => {
                println!("생성할 파일명 입력:");
                let filename = read_line().trim().to_string();
                println!("파일 내용 입력:");
                let content = read_line();

                match fs::write(&filename, content) {
                    Ok(_) => println!("{} 파일 생성 완료", filename),
                    Err(e) => println!("파일 생성 실패: {}", e),
                }
            }
            "2" => {
                println!("읽을 파일명 입력:");
                let filename = read_line().trim().to_string();
                match fs::read_to_string(&filename) {
                    Ok(data) => println!("파일 내용:\n{}", data),
                    Err(e) => println!("파일 읽기 실패: {}", e),
                }
            }
            "3" => {
                println!("생성할 디렉토리명 입력:");
                let dirname = read_line().trim().to_string();
                match fs::create_dir(&dirname) {
                    Ok(_) => println!("{} 디렉토리 생성 완료", dirname),
                    Err(e) => println!("디렉토리 생성 실패: {}", e),
                }
            }
            "4" => {
                println!("목록 확인할 디렉토리명 입력:");
                let dirname = read_line().trim().to_string();
                let path = Path::new(&dirname);
                if path.is_dir() {
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            println!("{} 디렉토리 목록:", dirname);
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    println!("{}", entry.file_name().to_string_lossy());
                                }
                            }
                        }
                        Err(e) => println!("읽기 실패: {}", e),
                    }
                } else {
                    println!("{} 는 디렉토리가 아닙니다.", dirname);
                }
            }
            "5" => {
                println!("프로그램 종료");
                break;
            }
            _ => println!("잘못된 선택입니다. 다시 시도하세요."),
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 오류");
    input
}

