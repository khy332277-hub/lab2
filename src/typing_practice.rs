use std::io::{self, Write};
use std::time::{Instant};

fn main() {
    let sentences = vec![
        "Rust is a systems programming language.",
        "Hello world! Let's practice typing.",
        "The quick brown fox jumps over the lazy dog."
    ];

    let mut total_chars = 0;
    let mut total_errors = 0;
    let start_time = Instant::now();

    println!("타자 연습 시작!\n");

    for sentence in &sentences {
        println!("문장: {}", sentence);
        print!("입력: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 오류");
        let input = input.trim();

        let mut errors = 0;
        for (c1, c2) in sentence.chars().zip(input.chars()) {
            if c1 != c2 {
                errors += 1;
            }
        }
        // 입력 길이가 문장보다 짧거나 길 경우 추가 오타 반영
        errors += sentence.len().saturating_sub(input.len());
        errors += input.len().saturating_sub(sentence.len());

        println!("오타: {}", errors);
        total_errors += errors;
        total_chars += input.len();
        println!();
    }

    let elapsed_secs = start_time.elapsed().as_secs_f64();
    let wpm = (total_chars as f64 / 5.0) / (elapsed_secs / 60.0); // 5글자 = 1 단어 기준

    println!("총 입력 글자 수: {}", total_chars);
    println!("총 오타 수: {}", total_errors);
    println!("소요 시간: {:.2}초", elapsed_secs);
    println!("평균 타수(WPM): {:.2}", wpm);
}

