use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * The 'breakingRecords' function returns the number of times Maria broke
 * her record for the highest and lowest scores.
 *
 * Parameters:
 * - scores: an array with the scores of each game in the season
 */
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    
    
    let mut max_count = 0;
    let mut min_count = 0;

    
    for &score in &scores[1..] {
        if score > max_score {
            max_score = score; 
            max_count += 1; 
        }
        if score < min_score {
            min_score = score; 
            min_count += 1; 
        }
    }

    vec![max_count, min_count] /
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of games
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the scores array
    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call breakingRecords and get the result
    let result = breakingRecords(&scores);

    // Write the result to file
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
