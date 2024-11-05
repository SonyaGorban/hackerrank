use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp::{max, min};

fn gcd(x: i32, y: i32) -> i32 {  
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i32, y: i32) -> i32 {     // Функція для обчислення НСК
    (x * y) / gcd(x, y)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {  // Основна функція, яка рахує кількість чисел між двома масивами
    // Calculate the LCM of array a
    let lcm_a = a.iter().cloned().reduce(|x, y| lcm(x, y)).unwrap();

    // Calculate the GCD of array b
    let gcd_b = b.iter().cloned().reduce(|x, y| gcd(x, y)).unwrap(); // Знаходимо НСД для масиву b

    // Count the multiples of lcm_a that divide gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
