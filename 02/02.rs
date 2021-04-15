use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut count1 = 0;
    let mut count2 = 0;
    for line in contents.lines() {
        let v: Vec<_> = line.split(':').collect();
        let rule = v[0].split(' ').collect::<Vec<_>>();
        let numbers = rule[0].split('-').collect::<Vec<_>>();
        let (start, end) = (
            numbers[0].parse::<usize>().unwrap(),
            numbers[1].parse::<usize>().unwrap(),
        );
        let c = rule[1];
        let password = &v[1][1..];
        // Part 1
        let char_count = password.matches(c).count();
        if start <= char_count && char_count <= end {
            count1 += 1;
        }
        // Part 2
        let (start, end) = (start - 1, end - 1);
        if password[start..start + 1] == *c {
            if password[end..end + 1] != *c {
                count2 += 1;
            }
        } else {
            if password[end..end + 1] == *c {
                count2 += 1;
            }
        }
    }
    println!("{} {}", count1, count2);
}
