use zbus::Connection;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session()?;
    connection.send_signal(
        Some("org.lab2.Notifier")?,
        "/org/lab2/Notifier",
        Some("org.lab2.Notifier")?,
        "Notify",
        &("안녕하세요! Lab2 알림 메시지입니다."),
    )?;

    println!("알림 메시지를 서버에 전송했습니다.");
    Ok(())
}

