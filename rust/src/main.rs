use std::fs;

struct Dial {
    pub value: usize,
    pub result: usize,
}

impl Dial {
    fn new() -> Dial {
        Dial {
            value: 50,
            result: 0,
        }
    }

    fn rotate_right(&mut self, n: usize) {
        let new_val = self.value + n;
        if new_val > 99 {
            self.value = new_val - 100;
        } else {
            self.value = new_val;
        }
        if self.value == 0 {
            self.result += 1;
        }
    }

    // TODO: we need to properly evaluate every rotation
    // else we lose some 0s
    fn rotate_left(&mut self, n: usize) {
        let mut n = n;
        if n > 99 {
            n %= 100;
        }

        if let Some(new) = self.value.checked_sub(n) {
            self.value = new;
        } else {
            self.value = 100 - self.value.abs_diff(n);
        }

        if self.value == 0 {
            self.result += 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/01.txt").unwrap();
    let mut dial = Dial::new();
    for line in input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
    }
    println!("{}", dial.result);
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
    L122";
    let mut dial = Dial::new();
    //println!("The dial starts by pointing at 50.");
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
        //println!("The dial is rotated {line} to point at {}.", dial.value);
    }

    assert_eq!(3, dial.result);
}

#[test]
fn test2() {
    let test_input = "L50
        L50
        L1";
    let mut dial = Dial::new();
    //println!("The dial starts by pointing at 50.");
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
        //println!("The dial is rotated {line} to point at {}.", dial.value);
    }

    assert_eq!(1, dial.result);
}

#[test]
fn test3() {
    let test_input = "L101";
    let mut dial = Dial::new();
    println!("The dial starts by pointing at 50.");
    for line in test_input.lines() {
        let line = line.trim();
        parse_and_apply_line(line, &mut dial);
        println!("The dial is rotated {line} to point at {}.", dial.value);
    }

    assert_eq!(1, dial.result);
}
