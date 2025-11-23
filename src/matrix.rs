use std::io;

fn main() {
    // 행렬 크기 입력
    println!("행의 수를 입력하세요:");
    let rows = read_usize();

    println!("열의 수를 입력하세요:");
    let cols = read_usize();

    // 첫 번째 행렬 입력
    println!("첫 번째 행렬을 입력하세요 (공백으로 구분):");
    let matrix1 = read_matrix(rows, cols);

    // 두 번째 행렬 입력
    println!("두 번째 행렬을 입력하세요 (공백으로 구분):");
    let matrix2 = read_matrix(rows, cols);

    // 행렬 덧셈
    let result = add_matrices(&matrix1, &matrix2, rows, cols);

    // 결과 출력
    println!("두 행렬의 합:");
    print_matrix(&result, rows, cols);
}

// usize 입력 읽기
fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 오류");
    input.trim().parse::<usize>().expect("숫자를 입력하세요")
}

// 행렬 입력
fn read_matrix(rows: usize, cols: usize) -> Vec<i32> {
    let mut matrix = Vec::with_capacity(rows * cols);

    for _ in 0..rows {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 오류");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("숫자를 입력하세요"))
            .collect();

        if row.len() != cols {
            panic!("열 수가 맞지 않습니다!");
        }

        matrix.extend(row);
    }

    matrix
}

// 행렬 덧셈
fn add_matrices(a: &Vec<i32>, b: &Vec<i32>, rows: usize, cols: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(rows * cols);

    for i in 0..(rows * cols) {
        result.push(a[i] + b[i]);
    }

    result
}

// 행렬 출력
fn print_matrix(matrix: &Vec<i32>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            print!("{} ", matrix[i * cols + j]);
        }
        println!();
    }
}

