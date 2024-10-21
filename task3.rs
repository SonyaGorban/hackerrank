use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn aVeryBigSum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Reading the first line (the number of elements in the array) but not using it directly
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Reading the array elements, parsing them as i64, and collecting them into a vector
    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    // Calculating the sum using aVeryBigSum function
    let result = aVeryBigSum(&ar);

    // Writing the result to the file specified by OUTPUT_PATH
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}
