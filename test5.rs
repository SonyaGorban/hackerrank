use std::io::{self, BufRead};

/*


 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
let total = arr.len() as f64; // Загал кількість елементів у масиві
    let positives = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negatives = arr.iter().filter(|&&x| x < 0).count() as f64;  // Підрахунок негативних елементів у масиві
    let zeros = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positives / total); // Виведення відношення позитивних елементів з точністю до 6 знаків після коми
    println!("{:.6}", negatives / total);
    println!("{:.6}", zeros / total);}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}