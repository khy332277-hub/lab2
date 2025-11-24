use zbus::Connection;
use std::error::Error;

#[tokio::main] // main 함수를 async로 만들어줌
async fn main() -> Result<(), Box<dyn Error>> {
    // 세션 버스 연결 (async)
    let connection = Connection::session().await?;
    
    // 서비스 이름 등록 (async)
    connection.request_name("org.lab2.Notifier").await?;
    
    println!("DBus 서버 시작! 알림 메시지를 기다립니다...");

    loop {
        println!("서버가 실행 중입니다...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

