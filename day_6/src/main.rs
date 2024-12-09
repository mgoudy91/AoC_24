use std::fs::read_to_string;

static VALID_GUARDS: [char; 4] = ['^', 'v', '<', '>'];

fn main() {
    println!("===AoC day six===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let path = walk_guard(raw_lines);
    let x_count = count_char_in_lines(&path, 'X');
    // pretty_print_lines(&path);
    println!("Number of steps: {}", x_count);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let blocker_count = walk_guard_with_direction(raw_lines);
    println!("Number of blocker squares: {}", blocker_count);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn find_guard(lines: &Vec<String>) -> (usize, usize) {
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if VALID_GUARDS.contains(&character) {
                return (x, y);
            }
        }
    }
    panic!("No guard found");
}

fn pretty_print_lines(lines: &Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

// part 1

fn walk_guard(lines: Vec<String>) -> Vec<String> {
    let mut map_with_path = lines.clone();
    let mut current_map = lines.clone();

    let (x, y) = find_guard(&lines);
    let mut guard_coords = (x, y);

    let mut guard_character;

    let mut current_direction;

    loop {
        guard_character = current_map[guard_coords.1]
            .chars()
            .nth(guard_coords.0)
            .unwrap();

        match guard_character {
            '^' => current_direction = "up",
            'v' => current_direction = "down",
            '<' => current_direction = "left",
            '>' => current_direction = "right",
            _ => panic!("Unknown guard"),
        }

        let (next_x, next_y, new_guard_char) = match current_direction {
            "up" => (guard_coords.0, guard_coords.1.wrapping_sub(1), '>'),
            "down" => (guard_coords.0, guard_coords.1 + 1, '<'),
            "left" => (guard_coords.0.wrapping_sub(1), guard_coords.1, '^'),
            "right" => (guard_coords.0 + 1, guard_coords.1, 'v'),
            _ => panic!("Unknown direction"),
        };

        if next_x >= current_map[0].len() || next_y >= current_map.len() {
            break;
        }

        if current_map[next_y].chars().nth(next_x).unwrap() == '#' {
            current_map[guard_coords.1].replace_range(
                guard_coords.0..guard_coords.0 + 1,
                &new_guard_char.to_string(),
            );
        } else {
            current_map[next_y].replace_range(next_x..next_x + 1, &guard_character.to_string());
            current_map[guard_coords.1].replace_range(guard_coords.0..guard_coords.0 + 1, ".");
            map_with_path[guard_coords.1].replace_range(guard_coords.0..guard_coords.0 + 1, "X");
            guard_coords = (next_x, next_y);
        }
    }
    map_with_path[guard_coords.1].replace_range(guard_coords.0..guard_coords.0 + 1, "X");
    map_with_path
}

fn count_char_in_lines(lines: &Vec<String>, character: char) -> usize {
    lines.iter().fold(0, |acc, line| {
        acc + line.chars().filter(|&c| c == character).count()
    })
}

// part 2
fn walk_guard_with_direction(lines: Vec<String>) -> usize {
    let mut current_map = lines.clone();
    let (x, y) = find_guard(&lines);
    let mut guard_coords = (x, y);

    let mut guard_character;
    let mut current_direction;
    let mut visited_coords = Vec::new();
    let mut potential_loops = 0;

    loop {
        guard_character = current_map[guard_coords.1]
            .chars()
            .nth(guard_coords.0)
            .unwrap();

        match guard_character {
            '^' => current_direction = "up",
            'v' => current_direction = "down",
            '<' => current_direction = "left",
            '>' => current_direction = "right",
            _ => panic!("Unknown guard"),
        }

        visited_coords.push((guard_coords.0, guard_coords.1, current_direction.to_string()));

        let (next_x, next_y, new_guard_char) = match current_direction {
            "up" => (guard_coords.0, guard_coords.1.wrapping_sub(1), '>'),
            "down" => (guard_coords.0, guard_coords.1 + 1, '<'),
            "left" => (guard_coords.0.wrapping_sub(1), guard_coords.1, '^'),
            "right" => (guard_coords.0 + 1, guard_coords.1, 'v'),
            _ => panic!("Unknown direction"),
        };

        if next_x >= current_map[0].len() || next_y >= current_map.len() {
            break;
        }

        if current_map[next_y].chars().nth(next_x).unwrap() == '#' {
            current_map[guard_coords.1].replace_range(
                guard_coords.0..guard_coords.0 + 1,
                &new_guard_char.to_string(),
            );
        } else {
            let pretend_guard_coords = match current_direction {
                "up" => (guard_coords.0 + 1, guard_coords.1),
                "down" => (guard_coords.0.wrapping_sub(1), guard_coords.1),
                "left" => (guard_coords.0, guard_coords.1 + 1),
                "right" => (guard_coords.0, guard_coords.1.wrapping_sub(1)),
                _ => panic!("Unknown direction"),
            };

            if visited_coords.contains(&(pretend_guard_coords.0, pretend_guard_coords.1, new_guard_char.to_string())) {
                potential_loops += 1;
            }

            current_map[next_y].replace_range(next_x..next_x + 1, &guard_character.to_string());
            current_map[guard_coords.1].replace_range(guard_coords.0..guard_coords.0 + 1, ".");
            guard_coords = (next_x, next_y);
        }
    }

    println!("Potential loops: {}", potential_loops);

    visited_coords.len()
}