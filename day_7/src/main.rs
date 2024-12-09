use std::fs::read_to_string;

struct Equation {
    test_value: i64,
    list_of_nums: Vec<i32>,
}

fn main() {
    println!("===AoC day seven===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let equations = lines_to_equations(raw_lines);
    let valid_sum = count_valid_equations(equations);
    println!("Sum of valid equations: {}", valid_sum);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let equations = lines_to_equations(raw_lines);
    let valid_sum = count_valid_equations_2(equations);
    println!("Sum of valid equations: {}", valid_sum);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn print_equation(equation: &Equation) {
    println!("Test value: {}", equation.test_value);
    for num in &equation.list_of_nums {
        print!("{}, ", num);
    }
    println!();
}

fn lines_to_equations(lines: Vec<String>) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in lines {
        let mut parts = line.split(": ");
        let test_value = parts.next().unwrap().parse::<i64>().unwrap();
        let list_of_nums = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        equations.push(Equation {
            test_value,
            list_of_nums,
        });
    }
    equations
}

// part 1

fn count_valid_equations(equations: Vec<Equation>) -> i64 {
    let operations = ["+", "*"];

    let mut valid_sum = 0;
    for equation in equations {
        let mut operations_list = Vec::new();
        let len = equation.list_of_nums.len();
        if len > 0 {
            let mut indices = vec![0; len - 1];
            loop {
                let ops: Vec<&str> = indices.iter().map(|&i| operations[i]).collect();
                operations_list.push(ops);

                let mut increment = false;
                for i in (0..indices.len()).rev() {
                    if indices[i] < operations.len() - 1 {
                        indices[i] += 1;
                        increment = true;
                        break;
                    } else {
                        indices[i] = 0;
                    }
                }
                if !increment {
                    break;
                }
            }
        }

        for ops in operations_list {
            let mut sum = equation.list_of_nums[0] as i64;
            for i in 0..ops.len() {
                match ops[i] {
                    "+" => sum += equation.list_of_nums[i + 1] as i64,
                    "*" => sum *= equation.list_of_nums[i + 1] as i64,
                    _ => panic!("Unknown operation"),
                }
            }
            if sum == equation.test_value {
                valid_sum += equation.test_value;
                break;
            }
        }
    }
    valid_sum
}


// part 2
fn count_valid_equations_2(equations: Vec<Equation>) -> i64 {
    let operations = ["+", "*", "||"];

    let mut valid_sum = 0;
    for equation in equations {
        let len = equation.list_of_nums.len();
        if len == 0 {
            continue;
        }

        let mut indices = vec![0; len - 1];
        loop {
            let ops: Vec<&str> = indices.iter().map(|&i| operations[i]).collect();

            let mut sum = equation.list_of_nums[0] as i64;
            for (i, &op) in ops.iter().enumerate() {
                match op {
                    "+" => sum += equation.list_of_nums[i + 1] as i64,
                    "*" => sum *= equation.list_of_nums[i + 1] as i64,
                    "||" => sum = format!("{}{}", sum, equation.list_of_nums[i + 1]).parse().unwrap(),
                    _ => panic!("Unknown operation"),
                }
            }
            if sum == equation.test_value {
                valid_sum += equation.test_value;
                break;
            }

            let mut increment = false;
            for i in (0..indices.len()).rev() {
                if indices[i] < operations.len() - 1 {
                    indices[i] += 1;
                    increment = true;
                    break;
                } else {
                    indices[i] = 0;
                }
            }
            if !increment {
                break;
            }
        }
    }
    valid_sum
}
