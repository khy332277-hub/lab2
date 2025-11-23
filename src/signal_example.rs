use nix::sys::signal;
use nix::unistd::Pid;
use std::thread;
use std::time::Duration;

fn main() {
    // 현재 프로세스 PID
    let pid = Pid::this();
    println!("현재 PID: {}", pid);

    // 2초 후에 SIGTERM 보내기
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        println!("SIGTERM 신호 전송");
        signal::kill(pid, signal::Signal::SIGTERM).unwrap();
    });

    println!("프로그램 실행 중... Ctrl+C 또는 SIGTERM 대기");

    // 무한 루프, 시그널을 기다림
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

