use std::io;

fn main() {
    let (rows, cols) = {
        let mut input = String::new();
        println!("행과 열 개수를 입력하세요 (예: 2 3):");
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.trim()
                                   .split_whitespace()
                                   .map(|x| x.parse().unwrap())
                                   .collect();
        (nums[0], nums[1])
    };

    let matrix1 = read_matrix(rows, cols, "첫 번째 행렬");
    let matrix2 = read_matrix(rows, cols, "두 번째 행렬");

    let mut result = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            result[i][j] = matrix1[i][j] + matrix2[i][j];
        }
    }

    println!("두 행렬의 합:");
    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn read_matrix(rows: usize, cols: usize, name: &str) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; cols]; rows];
    println!("{} 입력 (행별로 공백 구분):", name);
    for i in 0..rows {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.trim().split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect();
        if nums.len() != cols { panic!("열 개수 불일치!"); }
        matrix[i] = nums;
    }
    matrix
}

