use std::fs;

pub fn solve_a() {
    let input = fs::read_to_string("../inputs/04.txt").unwrap();
    let x_count = find_forklift_rolls(&input);
    println!("part 1: {}", x_count);
}

fn find_forklift_rolls(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut reachable = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &char) in line.iter().enumerate() {
            if char == '@' && count_neighbour_paper(&grid, row, col) < 4 {
                reachable += 1;
            }
        }
    }

    reachable
}

fn count_neighbour_paper(input: &[Vec<char>], row: usize, col: usize) -> usize {
    if input.is_empty() {
        return 0;
    }
    let mut paper = 0;
    let height = input.len() as isize;
    let width = input[0].len() as isize;

    const DELTAS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (delta_row, delta_col) in DELTAS {
        let new_col = (col as isize) + delta_col;
        let new_row = (row as isize) + delta_row;

        if new_row > -1
            && new_row < width
            && new_col > -1
            && new_col < height
            && input[new_row as usize][new_col as usize] == '@'
        {
            paper += 1;
        }
    }

    paper
}

#[test]
fn test() {
    let input = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
    let x_count = find_forklift_rolls(input);
    assert_eq!(13, x_count);
}
