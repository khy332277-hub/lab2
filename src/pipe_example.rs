use std::process::{Command, Stdio};
use std::io::{Write, Read};

fn main() {
    // 자식 프로세스 실행 (예: "cat" 명령)
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("자식 프로세스 생성 실패");

    // 자식 stdin에 데이터 쓰기
    let child_stdin = child.stdin.as_mut().expect("stdin 열기 실패");
    child_stdin.write_all(b"Hello from parent process!\n").expect("쓰기 실패");

    // 자식 stdout 읽기
    let mut output = String::new();
    child.stdout.as_mut().unwrap().read_to_string(&mut output).expect("읽기 실패");

    // 결과 출력
    println!("자식 프로세스에서 받은 데이터:\n{}", output);

    // 자식 프로세스 종료 대기
    let status = child.wait().expect("자식 종료 대기 실패");
    println!("자식 프로세스 종료 상태: {}", status);
}

