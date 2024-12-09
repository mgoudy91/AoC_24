use std::fs::read_to_string;

fn main() {
    println!("AoC day two");
    part_one();
    part_two();
}

fn part_one() {
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    println!("Part one");
    let lines: Vec<Vec<i32>> = read_lines("input.txt");
    let safe = count_safe(lines);
    println!("The number of safe line is: {}", safe);
}

fn part_two() {
    // The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.
    // The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!
    // Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
    println!("Part two");
    let lines: Vec<Vec<i32>> = read_lines("input.txt");
    let safe = count_safe_2(lines);
    println!("The number of safe line is: {}", safe);
}

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect() // gather them together into a vector
}

fn is_line_safe (line: Vec<i32>) -> bool {
    let mut is_safe = true;
    let going_up = line[1] > line[0];
    for i in 1..line.len() {
        if ((line[i] - line[i - 1]).abs() > 3)
            || (line[i] == line[i - 1])
            || (going_up != (line[i] > line[i - 1]))
        {
            is_safe = false;
            break;
        }
    }
    is_safe
}

fn count_safe(lines: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for line in lines {
        if is_line_safe(line) {
            result += 1;
        }
    }
    result
}

fn count_safe_2(lines: Vec<Vec<i32>>) -> i32 {
    let mut safe_lines: Vec<Vec<i32>> = Vec::new();
    let mut unsafe_lines: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if is_line_safe(line.clone()) {
            safe_lines.push(line);
        } else {
            unsafe_lines.push(line);
        }
    }

    for unsafe_line in unsafe_lines {
        let mut is_safe = false;
        for i in 0..unsafe_line.len() {
            let mut line = unsafe_line.clone();
            line.remove(i);
            if is_line_safe(line) {
                is_safe = true;
                break;
            }
        }
        if is_safe {
            safe_lines.push(unsafe_line);
        }
    }
    
    safe_lines.len() as i32
}