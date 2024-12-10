use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct DiskMapEntry {
    is_empty: bool,
    file_id: usize,
}

fn main() {
    println!("===AoC day nine===");
    part_one();
    part_two();
}

fn part_one() {
    println!("~Part one");
    let raw_lines = read_lines("input.txt");
    let uncompressed = uncompress_disk_map(raw_lines[0].clone());
    println!("Number of entries: {}", uncompressed.len());
    let defragged = defrag_disk_map(uncompressed);
    println!("Number of entries after defrag: {}", defragged.len());
    let checksum = calc_checksum(defragged);
    println!("Checksum: {}", checksum);
}

fn part_two() {
    println!("~Part two");
    let raw_lines = read_lines("input.txt");
    let uncompressed = uncompress_disk_map(raw_lines[0].clone());
    println!("Number of entries: {}", uncompressed.len());
    let defragged = defrag_disk_map_with_blocks(uncompressed);
    println!("Number of entries after defrag: {}", defragged.len());
    // log out X/. notation for debugging
    for entry in defragged.clone() {
        print!("{}", if entry.is_empty { "." } else { "X" });
    }

    let checksum = calc_checksum(defragged);
    println!("Checksum: {}", checksum);
}

// Common

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn uncompress_disk_map(line: String) -> Vec<DiskMapEntry> {
    let mut uncompressed = Vec::new();

    let mut file_id = 0;
    let mut is_empty = false;
    for char in line.chars() {
        let count = char.to_digit(10).unwrap() as usize;
        for _ in 0..count {
            uncompressed.push(DiskMapEntry {
                is_empty,
                file_id: if is_empty { 0 } else { file_id },
            });
        }
        if !is_empty {
            file_id += 1;
        }
        is_empty = !is_empty;
    }

    uncompressed
}

fn calc_checksum(input: Vec<DiskMapEntry>) -> usize {
    let mut checksum = 0;
    for (i, entry) in input.iter().enumerate() {
        checksum += i * entry.file_id;
    }
    checksum
}

// part 1

fn defrag_disk_map(uncompressed: Vec<DiskMapEntry>) -> Vec<DiskMapEntry> {
    let mut defragged = uncompressed.clone();
    let mut left = 0;
    let mut right = defragged.len() - 1;

    while left < right {
        while left < defragged.len() && !defragged[left].is_empty {
            left += 1;
        }
        while right > 0 && defragged[right].is_empty {
            right -= 1;
        }
        if left < right {
            defragged.swap(left, right);
        }
    }

    defragged
}

// part 2
// this doesn't work...
fn defrag_disk_map_with_blocks(uncompressed: Vec<DiskMapEntry>) -> Vec<DiskMapEntry> {
    let mut defragged = uncompressed.clone();

    for file_id in (1..=defragged.iter().map(|entry| entry.file_id).max().unwrap()).rev() {
        let mut left = 0;

        while left < defragged.len() {
            // Find the next block of entries with the current file_id
            let mut block_start = None;
            let mut block_end = None;
            for i in left..defragged.len() {
                if defragged[i].file_id == file_id && !defragged[i].is_empty {
                    if block_start.is_none() {
                        block_start = Some(i);
                    }
                    block_end = Some(i);
                } else if block_start.is_some() {
                    break;
                }
            }

            if let (Some(start), Some(end)) = (block_start, block_end) {
                let block_size = end - start + 1;

                // Find the next empty space that can fit the entire block
                let mut empty_start = None;
                let mut empty_count = 0;
                for i in 0..defragged.len() {
                    if defragged[i].is_empty {
                        if empty_start.is_none() {
                            empty_start = Some(i);
                        }
                        empty_count += 1;
                        if empty_count == block_size {
                            break;
                        }
                    } else {
                        empty_start = None;
                        empty_count = 0;
                    }
                }

                if let Some(empty_start) = empty_start {
                    // Move the block to the empty space
                    for i in 0..block_size {
                        defragged.swap(empty_start + i, start + i);
                    }
                    left = empty_start + block_size;
                } else {
                    left = end + 1;
                }
            } else {
                break;
            }
        }
    }

    defragged
}