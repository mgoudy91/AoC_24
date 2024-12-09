use std::fs::read_to_string;

struct LinesAndDict {
    first_values: Vec<String>,
    dict: std::collections::HashMap<String, i32>,
}

fn main() {
    println!("AoC day one");
    part_one();
    part_two();
}

fn part_one() {
    println!("Part one");
    let raw_lines = read_lines("input.txt");
    let lines = lines_to_arrays(raw_lines);
    let sorted_lines = sort_arrays(lines);
    let distance = calculate_distance(sorted_lines);
    println!("The p1 distance between the two arrays is: {}", distance);
}

fn part_two () {
    println!("Part two");
    let raw_lines = read_lines("input.txt");
    let lines_and_dict: LinesAndDict = lines_to_arrays_and_dict(raw_lines);
    let distance = calculate_distance_2(lines_and_dict);
    println!("The p2 distance between the two arrays is: {}", distance);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn lines_to_arrays(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut first_values = Vec::new();
    let mut second_values = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        first_values.push(values[0].to_string());
        second_values.push(values[1].to_string());
    }

    vec![first_values, second_values]
}



fn lines_to_arrays_and_dict(lines: Vec<String>) -> LinesAndDict {
    let mut dict = std::collections::HashMap::new();
    let mut first_values = Vec::new();
    let mut second_values = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        first_values.push(values[0].to_string());
        second_values.push(values[1].to_string());
        let count = dict.entry(values[1].to_string()).or_insert(0);
        *count += 1;
    }
    LinesAndDict {
        first_values,
        dict,
    }
}

fn sort_arrays(lines: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut sorted_lines = lines.clone();
    sorted_lines[0].sort();
    sorted_lines[1].sort();
    sorted_lines
}

fn calculate_distance(sorted_lines: Vec<Vec<String>>) -> i32 {
    let mut distance = 0;
    for i in 0..sorted_lines[0].len(){
        let first_value = &sorted_lines[0][i];
        let second_value = &sorted_lines[1][i];
        let diff = first_value.parse::<i32>().unwrap() - second_value.parse::<i32>().unwrap();
        distance += diff.abs();
    }
    distance
}

fn calculate_distance_2(lines_and_dict: LinesAndDict) -> i32 {
    let mut distance = 0;
    for i in 0..lines_and_dict.first_values.len(){
        let first_value = &lines_and_dict.first_values[i];
        // look up the number of times the first_value has been seen in dict, or 0 if it's not in the dict
        let count = *lines_and_dict.dict.get(first_value).unwrap_or(&0);
        distance += (first_value.parse::<i32>().unwrap() * count);
    }
    distance
}