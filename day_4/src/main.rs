use std::fs::read_to_string;

static TARGET_WORD: &str = "XMAS";

// 'cross' notation is first char is middle, then the rest is in a cross pattern
static TARGET_CROSS: &str = "AMS";

fn main() {
    println!("===AoC day four===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let count = find_word(raw_lines, TARGET_WORD);
    println!("The word {} appears {} times", TARGET_WORD, count);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");

    // log out a 10x10 grid of the first 10x10 characters
    // for line in raw_lines.iter().take(10) {
    //     println!("{}", line.chars().take(10).collect::<String>());
    // }

    // let grid: Vec<String> = raw_lines.iter().take(10).map(|line| line.chars().take(10).collect()).collect();
    // let grid: Vec<String> = raw_lines.iter().take(10).map(|line| line.chars().take(10).collect()).collect();
    let count = find_word_2(raw_lines, TARGET_CROSS);
    println!("The cross {} appears {} times", TARGET_CROSS, count);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

// Part 1

fn find_word(lines: Vec<String>, word: &str) -> i32 {
    let first_char = word.chars().nth(0).unwrap();
    let mut word_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == first_char {
                word_count += count_target_word_from_location(&lines, x, y, &word[1..]);
            }
        }
    }

    word_count
}

fn count_target_word_from_location(
    lines: &Vec<String>,
    x: usize,
    y: usize,
    remaining_target: &str,
) -> i32 {
    let mut word_count = 0;
    let directions = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    for (dx, dy) in directions {
        let mut current_x = x as isize;
        let mut current_y = y as isize;
        let mut current_target = remaining_target;
        while current_target.len() > 0 {
            current_x += dx;
            current_y += dy;
            if current_x < 0
                || current_x >= lines[0].len() as isize
                || current_y < 0
                || current_y >= lines.len() as isize
            {
                break;
            }
            let current_character = lines[current_y as usize]
                .chars()
                .nth(current_x as usize)
                .unwrap();
            if current_character == current_target.chars().nth(0).unwrap() {
                current_target = &current_target[1..];
            } else {
                break;
            }
        }
        if current_target.len() == 0 {
            word_count += 1;
        }
    }

    word_count
}

// Part 2

fn find_word_2(lines: Vec<String>, word: &str) -> i32 {
    // Same as before, only difference is we do something different for each target letter
    let first_char = word.chars().nth(0).unwrap();
    let mut cross_count = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == first_char {
                cross_count += count_crosses_from_location(&lines, x, y, &word[1..]);
            }
        }
    }
    cross_count
}

fn count_crosses_from_location(
    lines: &Vec<String>,
    x: usize,
    y: usize,
    remaining_target: &str,
) -> i32 {
    println!("=======");
    println!("Checking location {}, {}", x, y);
    let mut word_count = 0;
    let diagonals = vec![(1, 1), (1, -1)];
    for (dx, dy) in diagonals {
        let mut current_x = x as isize;
        let mut current_y = y as isize;
        let mut current_target = remaining_target;
        let x1 = current_x + dx;
        let y1 = current_y + dy;
        let x2 = current_x - dx;
        let y2 = current_y - dy;

        if x1 < 0 || x1 >= lines[0].len() as isize {
            println!("Fail: Out of bounds: x1 = {}", x1);
            return 0;
        }
        if y1 < 0 || y1 >= lines.len() as isize {
            println!("Fail: Out of bounds: y1 = {}", y1);
            return 0;
        }
        if x2 < 0 || x2 >= lines[0].len() as isize {
            println!("Fail: Out of bounds: x2 = {}", x2);
            return 0;
        }
        if y2 < 0 || y2 >= lines.len() as isize {
            println!("Fail: Out of bounds: y2 = {}", y2);
            return 0;
        }

        let current_character1 = lines[y1 as usize].chars().nth(x1 as usize).unwrap();
        let current_character2 = lines[y2 as usize].chars().nth(x2 as usize).unwrap();
        println!("Checking diagonals: {}, {}", current_character1, current_character2);

        // check if current_character1 is found inside current_target, do not change current_target
        if current_target.contains(current_character1) {
            println!("Found current_character1 in current_target");
        } else {
            println!("Fail: current_character1 not found in current_target");
            return 0;
        }

        // check if current_character2 is found inside current_target, do not change current_target
        if current_target.contains(current_character2) {
            println!("Found current_character2 in current_target");
        } else {
            println!("Fail: current_character2 not found in current_target");
            return 0;
        }
        
        // are current_character1 and current_character2 different
        if current_character1 == current_character2 {
            println!("Fail: current_character1 and current_character2 are the same");
            return 0;
        }
    }
    println!("Success: Found a cross");
    return 1;
}
