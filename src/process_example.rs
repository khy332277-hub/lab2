use std::process::Command;

fn main() {
    println!("ls 명령 실행 예제:");

    // "ls -l" 명령 실행
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("명령 실행 실패");

    // 결과 출력
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

