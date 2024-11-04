use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
for i in 0..n {
        // Calculate the number of spaces
        let spaces = n - i - 1;
        // Calculate the number of hashes
        let hashes = i + 1;

        // Print spaces
        for _ in 0..spaces {
            print!(" ");
        }
        // Print hashes
        for _ in 0..hashes {
            print!("#");
        }

        // Move to the next line
        println!();
    }

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}