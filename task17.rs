use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequency = HashMap::new();

    // Count the frequency of each bird type
    for &bird_type in arr {
        *frequency.entry(bird_type).or_insert(0) += 1;
    }

    // Find the bird type with the highest frequency, using the smallest ID in case of a tie
    let mut max_count = 0;
    let mut most_frequent_type = i32::MAX;

    for (&bird_type, &count) in &frequency {
        if count > max_count || (count == max_count && bird_type < most_frequent_type) {
            max_count = count;
            most_frequent_type = bird_type;
        }
    }

    most_frequent_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
