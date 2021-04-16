use std::fs;

fn count(field: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < field.len() {
        if field[y][x] == '#' {
            count += 1;
        }
        x = (x + dx) % field[0].len();
        y += dy;
    }
    return count;
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut field: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        field.push(line.chars().collect());
    }
    println!(
        "{} {}",
        count(&field, 3, 1),
        count(&field, 1, 1)
            * count(&field, 3, 1)
            * count(&field, 5, 1)
            * count(&field, 7, 1)
            * count(&field, 1, 2)
    )
}
