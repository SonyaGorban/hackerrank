use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Calculate Anna's total excluding the k-th item
    let total: i32 = bill.iter().enumerate()
        .filter(|(i, _)| *i != k as usize) // Exclude the k-th item
        .map(|(_, &price)| price) // Get prices
        .sum(); // Sum the prices
    
    // Calculate what Anna should actually pay
    let anna_share = total / 2; // Split the total with Brian

    // Check if Brian's calculation is correct
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share); 
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}
