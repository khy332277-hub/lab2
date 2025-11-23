use std::collections::HashMap;
use std::io;

fn main() {
    let mut phonebook: HashMap<String, String> = HashMap::new();

    loop {
        println!("전화번호부 메뉴:");
        println!("1. 등록");
        println!("2. 검색");
        println!("3. 목록 출력");
        println!("4. 종료");
        println!("선택하세요 (1-4):");

        let choice = read_line().trim().to_string();

        match choice.as_str() {
            "1" => {
                println!("이름 입력:");
                let name = read_line().trim().to_string();
                println!("전화번호 입력:");
                let number = read_line().trim().to_string();

                phonebook.insert(name.clone(), number.clone());
                println!("{}: {} 등록 완료", name, number);
            }
            "2" => {
                println!("검색할 이름 입력:");
                let name = read_line().trim().to_string();
                match phonebook.get(&name) {
                    Some(number) => println!("{}의 전화번호: {}", name, number),
                    None => println!("{}을(를) 찾을 수 없습니다.", name),
                }
            }
            "3" => {
                println!("전화번호부 전체 목록:");
                for (name, number) in &phonebook {
                    println!("{}: {}", name, number);
                }
            }
            "4" => {
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

