use std::fs::read_to_string;

fn main() {
    println!("===AoC day four===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let (raw_rules, raw_updates) = split_lines(raw_lines);
    let rule_map = create_rule_map(raw_rules);
    let middle_sum = find_valid_updates(rule_map, raw_updates);
    println!("Part one Middle sum: {}", middle_sum);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let (raw_rules, raw_updates) = split_lines(raw_lines);
    let rule_map = create_rule_map(raw_rules);
    let invalid_sum = sum_invalid_updates(rule_map, raw_updates);
    println!("Part two Invalid sum: {}", invalid_sum);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn split_lines(raw_lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in raw_lines {
        if line.contains('|') {
            rules.push(line);
        } else if line.contains(',') {
            updates.push(line);
        }
    }
    (rules, updates)
}

fn create_rule_map(raw_rules: Vec<String>) -> Vec<(String, Vec<String>)> {
    // rule map is a new hashmap, the value is a vector of strings
    let mut rule_map: Vec<(String, Vec<String>)> = Vec::new();

    for rule in raw_rules {
        let mut parts = rule.split('|');
        let first = parts.next().unwrap().trim().to_string();
        let second = parts.next().unwrap().trim().to_string();

        if let Some((_, values)) = rule_map.iter_mut().find(|(key, _)| key == &second) {
            values.push(first);
        } else {
            rule_map.push((second, vec![first]));
     }
    }
    // println!("{:?}", rule_map);
    rule_map
}

// part 1

fn find_valid_updates(
    rule_map: Vec<(String, Vec<String>)>,
    raw_updates: Vec<String>,
) -> u32 {
    let mut middle_sum = 0;

    for update in raw_updates {
        let mut invalid_values = Vec::new();
        let parts = update.split(',');
        let mut is_valid = true;
        for part in parts {
            if invalid_values.contains(&part.to_string()) {
                // this update is invalid
                is_valid = false;
                break;
            } else {
                if let Some((_, values)) = rule_map.iter().find(|(key, _)| key == &part) {
                    invalid_values.extend(values.clone());
                }
            }
        }
        if is_valid {
            // get middle part of the update, which is variable length
            let length = update.split(',').collect::<Vec<&str>>().len() as u32;
            let middle_value = update.split(',').collect::<Vec<&str>>()[(length / 2) as usize];
            middle_sum += middle_value.parse::<u32>().unwrap();
        }
    }
    middle_sum
}

// part 2

fn update_sort(a: &str, b: &str, rule_map: Vec<(String, Vec<String>)>) -> std::cmp::Ordering {

    if rule_map.iter().any(|(key, values)| key == a && values.contains(&b.to_string())) {
        return std::cmp::Ordering::Greater;
    } else if rule_map.iter().any(|(key, values)| key == b && values.contains(&a.to_string())) {
        return std::cmp::Ordering::Less;
    } else {
        println!("no rule for {:?} and {:?}", a, b);
        return std::cmp::Ordering::Equal;
    }
}

fn sum_invalid_updates(
    rule_map: Vec<(String, Vec<String>)>,
    raw_updates: Vec<String>,
) -> u32 {
    let mut invalid_sum = 0;

    for update in raw_updates {
        let mut invalid_values = Vec::new();
        let parts = update.split(',');
        let mut is_valid = true;
        for part in parts {
            if invalid_values.contains(&part.to_string()) {
                // this update is invalid
                is_valid = false;
                break;
            } else {
                if let Some((_, values)) = rule_map.iter().find(|(key, _)| key == &part) {
                    invalid_values.extend(values.clone());
                }
            }
        }
        if !is_valid {
            let mut parts: Vec<&str> = update.split(',').collect();
            parts.sort_by(|a, b| update_sort(a, b, rule_map.clone()));

            // get middle part of the update, which is variable length
            let length = parts.len() as u32;
            let middle_value = parts[(length / 2) as usize];
            invalid_sum += middle_value.parse::<u32>().unwrap();
        }
    }
    invalid_sum
}