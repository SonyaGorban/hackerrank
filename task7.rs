use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn miniMaxSum(arr: &[i32]) {
    let total_sum: i32 = arr.iter().sum(); // Обчисл загальну суму всіх елементів масиву
    let min_value = *arr.iter().min().unwrap(); // Знах мінімальне значення в масиві
    let max_value = *arr.iter().max().unwrap();

    let min_sum = total_sum - max_value; // Sum excluding the maximum element
    let max_sum = total_sum - min_value; // Sum excluding the minimum element

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
