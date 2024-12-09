use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("===AoC day three===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let cleaned_lines = clean_input(raw_lines);
    println!("Cleaned lines count {:?}", cleaned_lines.len());
    let sum = sum_up(cleaned_lines);
    println!("Sum: {}", sum);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let cleaned_lines = clean_input_2(raw_lines);
    println!("Cleaned lines count {:?}", cleaned_lines.len());
    let sum = sum_up_2(cleaned_lines);
    println!("Sum: {}", sum);
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

fn clean_input(lines: Vec<String>) -> Vec<String> {
    // Define the regex pattern
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    lines
        .into_iter()
        .flat_map(|s| {
            re.find_iter(&s)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<String>>() // Collect matches as a vector of strings
        })
        .collect::<Vec<String>>() // Collect results into a Vec<String>
}

fn sum_up(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    
    for line in lines {
        let nums = line
            .split(|c| c == '(' || c == ')' || c == ',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        sum += nums[0] * nums[1];
    }
    sum
}

// Part 2

fn clean_input_2(lines: Vec<String>) -> Vec<String> {
    // Define the regex pattern
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    lines
        .into_iter()
        .flat_map(|s| {
            re.find_iter(&s)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<String>>() // Collect matches as a vector of strings
        })
        .collect::<Vec<String>>() // Collect results into a Vec<String>
}

fn sum_up_2(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    let mut is_counting = true;
    
    for line in lines {
        match line.as_str() {
            "do()" => is_counting = true,
            "don't()" => is_counting = false,
            _ => {
                if is_counting {
                    let nums = line
                        .split(|c| c == '(' || c == ')' || c == ',')
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
                    sum += nums[0] * nums[1];
                }
            }
        }
    }
    sum
}