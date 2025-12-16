use std::fs;

pub fn solve_a() {
    let input = fs::read_to_string("../inputs/03.txt").unwrap();
    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();
        println!("line: {line}");
        total_joltage += calculate_highest_joltage(line).unwrap();
    }
    println!("part 1: {}", total_joltage);
}

pub fn solve_b() {
    let input = fs::read_to_string("../inputs/03.txt").unwrap();
    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();
        println!("line: {line}");
        total_joltage += calculate_highest_joltage2(line).unwrap();
    }
    println!("part 2: {}", total_joltage);
}

fn calculate_highest_joltage(line: &str) -> Option<i64> {
    let mut nums_with_index: Vec<(usize, u32)> = line
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .collect();

    // this needs to be a stable sort!
    nums_with_index.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (left_index, left_val)) in nums_with_index.iter().enumerate() {
        for (j, (right_index, right_val)) in nums_with_index.iter().enumerate() {
            if i == j {
                continue;
            }
            if left_index < right_index {
                let num = format!("{left_val}{right_val}");
                return num.parse::<i64>().ok();
            }
        }
    }

    None
}

fn calculate_highest_joltage2(line: &str) -> Option<i64> {
    let mut nums_with_index: Vec<(usize, u32)> = line
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .collect();

    //let pick_count = 12;
    //let ignore_count = line.chars().count() - pick_count;
    //let mut ignored_indeces = vec![];

    // this needs to be a stable sort!
    //nums_with_index.sort_by(|a, b| b.1.cmp(&a.1));
    nums_with_index.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("nums: {:?}", nums_with_index);
    println!();

    let mut picks = vec![];

    for (i, (left_index, left_val)) in nums_with_index.iter().enumerate() {
        for (j, (right_index, right_val)) in nums_with_index.iter().enumerate() {
            if i == j {
                continue;
            }
            if left_index < right_index {
                let num = format!("{left_val}{right_val}");
                let parsed = num.parse::<i64>().ok();
                picks.push(parsed.unwrap());
                return parsed;
            }
        }
    }

    None
}

#[test]
fn test1() {
    let input = "987654321111111
    811111111111119
    234234234234278
    818181911112111";
    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();
        total_joltage += calculate_highest_joltage(line).unwrap();
    }
    assert_eq!(357, total_joltage);
}

/*
#[test]
fn test2() {
    let input = "987654321111111
    811111111111119
    234234234234278
    818181911112111";
    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();
        total_joltage += calculate_highest_joltage2(line).unwrap();
    }
    assert_eq!(3121910778619, total_joltage);
}
*/
