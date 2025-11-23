use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

fn main() {
    println!("메시지 큐 채팅 예제 시작!");

    // 채널 생성 (송신자, 수신자)
    let (tx, rx) = mpsc::channel();

    // 상대방 스레드 (수신자)
    let handle = thread::spawn(move || {
        for received in rx {
            println!("상대: {}", received);
        }
    });

    // 메인 스레드 (송신자)
    loop {
        print!("나: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");
        let input = input.trim().to_string();

        if input.eq_ignore_ascii_case("exit") {
            println!("채팅 종료!");
            break;
        }

        tx.send(input).expect("메시지 전송 실패");
    }

    // 채널 닫기
    drop(tx);

    // 상대방 스레드 종료 대기
    handle.join().unwrap();
}

