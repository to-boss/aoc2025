use std::fs;

struct Dial {
    pub value: usize,
    pub result: usize,
    count_every_zero: bool,
}

impl Dial {
    fn new(count_every_zero: bool) -> Dial {
        Dial {
            value: 50,
            result: 0,
            count_every_zero,
        }
    }

    fn rotate_right(&mut self, n: usize) {
        let mut n = n;
        while n > 0 {
            if self.value == 99 {
                self.value = 0;
            } else {
                self.value += 1;
            }

            if self.count_every_zero && self.value == 0 {
                self.result += 1;
            }
            n -= 1;
        }

        if !self.count_every_zero && self.value == 0 {
            self.result += 1;
        }
    }

    fn rotate_left(&mut self, n: usize) {
        let mut n = n;
        while n > 0 {
            if self.value == 0 {
                self.value = 99;
            } else {
                self.value -= 1;
            }
            n -= 1;
            if self.count_every_zero && self.value == 0 {
                self.result += 1;
            }
        }

        if !self.count_every_zero && self.value == 0 {
            self.result += 1;
        }
    }
}

pub fn solve_a() {
    let input = fs::read_to_string("../inputs/01.txt").unwrap();
    let mut dial = Dial::new(false);
    for line in input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }
    println!("part 1: {}", dial.result);
}

pub fn solve_b() {
    let input = fs::read_to_string("../inputs/01.txt").unwrap();
    let mut dial = Dial::new(true);
    for line in input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }
    println!("part 2: {}", dial.result);
}

fn parse_and_apply_line(line: &str, dial: &mut Dial) {
    let (dir, num) = line.split_at(1);
    let num: usize = num.parse().unwrap();
    match dir {
        "R" => dial.rotate_right(num),
        "L" => dial.rotate_left(num),
        _ => unreachable!(),
    }
}

#[test]
fn test1() {
    let test_input = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";
    let mut dial = Dial::new(false);
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }

    assert_eq!(3, dial.result);
}

#[test]
fn test2() {
    println!("T2 Start ------");
    let test_input = "L50
        L50
        L1";
    let mut dial = Dial::new(false);
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }

    assert_eq!(1, dial.result);
}

#[test]
fn test4() {
    let test_input = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    L122";
    let mut dial = Dial::new(true);
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }

    assert_eq!(6, dial.result);
}
