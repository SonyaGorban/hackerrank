use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut left_to_right_sum = 0;
    let mut right_to_left_sum = 0;

    for i in 0..n {
        left_to_right_sum += arr[i][i];         // Sum for the left-to-right diagonal
        right_to_left_sum += arr[i][n - i - 1]; // Sum for the right-to-left diagonal
    }

    (left_to_right_sum - right_to_left_sum).abs() // Return the absolute difference
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the size of the matrix
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    // Initialize the 2D vector (matrix)
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    // Read the matrix values
    for i in 0..n {
        arr.push(stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect());
    }

    // Calculate the diagonal difference
    let result = diagonalDifference(&arr);

    // Write the result to the file specified by OUTPUT_PATH
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}
