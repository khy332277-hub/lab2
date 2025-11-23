use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    println!("파이프 채팅 예제 시작!");

    // 자식 프로세스: cat 명령으로 부모 메시지 읽고 그대로 출력
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("자식 프로세스 생성 실패");

    let mut child_stdin = child.stdin.take().expect("stdin 열기 실패");
    let mut child_stdout = child.stdout.take().expect("stdout 열기 실패");

    loop {
        // 부모 입력 받기
        print!("나: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");
        let input = input.trim();

        // 종료 조건
        if input.eq_ignore_ascii_case("exit") {
            println!("채팅 종료!");
            break;
        }

        // 자식 프로세스로 메시지 보내기
        writeln!(child_stdin, "{}", input).expect("쓰기 실패");
        child_stdin.flush().unwrap();

        // 자식 출력 읽기
        let mut buffer = String::new();
        use std::io::Read;
        child_stdout.read_to_string(&mut buffer).unwrap();
        println!("상대: {}", buffer.trim());
    }

    // 자식 stdin 닫기
    drop(child_stdin);

    // 자식 종료 대기
    child.wait().expect("자식 종료 실패");
}
