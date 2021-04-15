use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();
    // Part 1
    for i in 0..numbers.len() - 1 {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{}", numbers[i] * numbers[j]);
            }
        }
    }
    // Part 2
    for i in 0..numbers.len() - 2 {
        for j in i..numbers.len() - 1 {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{}", numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
}
