use std::fs;

pub fn solve_a() {
    let input = fs::read_to_string("../inputs/02.txt").unwrap();
    let mut sum = 0;
    for line in input.split(",") {
        let (first, last) = line.split_once("-").expect("given string has -");
        let range = Range::new(first, last);
        let ids = range.generate_ids_i64();
        let ids_str = range.generate_ids_str();

        for (i, id_str) in ids_str.iter().enumerate() {
            if is_invalid(id_str) {
                sum += ids[i];
            }
        }
    }
    println!("part 1: {sum}");
}

pub fn solve_b() {
    let input = fs::read_to_string("../inputs/02.txt").unwrap();
    let mut sum = 0;
    for line in input.split(",") {
        let (first, last) = line.split_once("-").expect("given string has -");
        let range = Range::new(first, last);
        let ids = range.generate_ids_i64();
        let ids_str = range.generate_ids_str();

        for (i, id_str) in ids_str.iter().enumerate() {
            if is_invalid2(id_str) {
                sum += ids[i];
            }
        }
    }
    println!("part 2: {sum}");
}

fn is_invalid(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }

    let (left, right) = id.split_at(id.len() / 2);
    left == right
}

fn is_invalid2(id: &str) -> bool {
    let max_pattern = id.len() / 2;

    for pattern_size in 1..max_pattern + 1 {
        let pattern = &id.as_bytes()[0..pattern_size];
        let mut chunks = id.as_bytes().chunks_exact(pattern_size);

        let all_same = chunks.all(|chunk| chunk == pattern);
        if all_same && chunks.remainder().is_empty() {
            return true;
        }
    }

    false
}

struct Range {
    first: i64,
    last: i64,
}

impl Range {
    fn new(first: &str, last: &str) -> Self {
        let first = first.trim().parse::<i64>().unwrap();
        let last = last.trim().parse::<i64>().unwrap();

        Range { first, last }
    }

    fn generate_ids_i64(&self) -> Vec<i64> {
        let mut ids = vec![];
        for id in self.first..self.last + 1 {
            ids.push(id);
        }

        ids
    }

    fn generate_ids_str(&self) -> Vec<String> {
        let mut ids = vec![];
        for id in self.first..self.last + 1 {
            ids.push(id.to_string());
        }

        ids
    }
}

#[test]
fn test1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let mut sum = 0;
    for line in input.split(",") {
        let (first, last) = line.split_once("-").expect("given string has -");
        let range = Range::new(first, last);
        let ids = range.generate_ids_i64();
        let ids_str = range.generate_ids_str();

        for (i, id_str) in ids_str.iter().enumerate() {
            if is_invalid(id_str) {
                sum += ids[i];
            }
        }
    }
    assert_eq!(1227775554, sum);
}

#[test]
fn test2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let mut sum = 0;
    for line in input.split(",") {
        let (first, last) = line.split_once("-").expect("given string has -");
        let range = Range::new(first, last);
        let ids = range.generate_ids_i64();
        let ids_str = range.generate_ids_str();

        for (i, id_str) in ids_str.iter().enumerate() {
            if is_invalid2(id_str) {
                sum += ids[i];
            }
        }
    }
    assert_eq!(4174379265, sum);
}
