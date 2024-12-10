use std::fs::read_to_string;

struct Frequency {
    character: char,
    coordinates: Vec<(usize, usize)>,
}

fn main() {
    println!("===AoC day eight===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let frequencies = gather_frequencies(&raw_lines);

    let solution_map = find_antinodes(&raw_lines, &frequencies);
    let antinode_count = solution_map
        .iter()
        .fold(0, |acc, line| acc + line.matches("#").count());
    println!("Antinode count: {}", antinode_count);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let frequencies = gather_frequencies(&raw_lines);

    let solution_map = find_antinodes_2(&raw_lines, &frequencies);
    println!("Solution map:");
    for line in &solution_map {
        println!("{}", line);
    }
    let antinode_count = solution_map
        .iter()
        .fold(0, |acc, line| acc + line.matches("#").count());
    println!("Antinode count: {}", antinode_count);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn gather_frequencies(lines: &Vec<String>) -> Vec<Frequency> {
    let mut frequencies: Vec<Frequency> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if !character.is_alphanumeric() {
                continue;
            }
            if let Some(freq) = frequencies
                .iter_mut()
                .find(|freq| freq.character == character)
            {
                freq.coordinates.push((x, y));
            } else {
                frequencies.push(Frequency {
                    character,
                    coordinates: vec![(x, y)],
                });
            }
        }
    }
    frequencies
}

fn find_antinodes(lines: &Vec<String>, frequencies: &Vec<Frequency>) -> Vec<String> {
    let mut solution_map = lines.clone();
    for freq in frequencies {
        for (i, coord) in freq.coordinates.iter().enumerate() {
            let (x, y) = coord;
            for (j, other_coord) in freq.coordinates.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (other_x, other_y) = other_coord;
                let x_offset = *other_x as i32 - *x as i32;
                let y_offset = *other_y as i32 - *y as i32;
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let x_new = *x as i32 + 2 * x_offset;
                let y_new = *y as i32 + 2 * y_offset;
                if x_new >= 0
                    && y_new >= 0
                    && x_new < solution_map[0].len() as i32
                    && y_new < solution_map.len() as i32
                {
                    let mut new_line = solution_map[y_new as usize].clone();
                    new_line.replace_range(x_new as usize..x_new as usize + 1, "#");
                    solution_map[y_new as usize] = new_line;
                }
            }
        }
    }

    solution_map
}

fn find_antinodes_2(lines: &Vec<String>, frequencies: &Vec<Frequency>) -> Vec<String> {
    // the same as one, but a valid antinode can be found in any place where a repeated offset is in bounds
    let mut solution_map = lines.clone();
    for freq in frequencies {
        for (i, coord) in freq.coordinates.iter().enumerate() {
            let (x, y) = coord;
            for (j, other_coord) in freq.coordinates.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (other_x, other_y) = other_coord;
                let x_offset = *other_x as i32 - *x as i32;
                let y_offset = *other_y as i32 - *y as i32;
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let mut x_new = *x as i32 + x_offset;
                let mut y_new = *y as i32 + y_offset;
                while x_new >= 0
                    && y_new >= 0
                    && x_new < solution_map[0].len() as i32
                    && y_new < solution_map.len() as i32
                {
                    let mut new_line = solution_map[y_new as usize].clone();
                    new_line.replace_range(x_new as usize..x_new as usize + 1, "#");
                    solution_map[y_new as usize] = new_line;
                    x_new += x_offset;
                    y_new += y_offset;
                }
            }
        }
    }
    solution_map
}