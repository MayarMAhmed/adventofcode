use std::env;
use std::fs;
use std::time::Instant;

fn find_synonym(min: i64, max: i64, flag: bool) -> i64 {
    let mut count: i64 = 0;
    for num in min..=max {
        let num_str = num.to_string();
        let length = num_str.len(); //becase my input is digits only so this works and faster the .chars().count()
        if flag {
            /*for i in 1..=(length / 2 ) {
                if length % i != 0 {
                    continue;
                }
                let subseq = &num_str[..i];
                let repeats = length / i;
                if subseq.repeat(repeats) == num_str {
                    count += num;
                    break;
                }
            }*/
            // this is the lazy iterator way of rust which sort of like when we do map lambda in python.
            // A pain to build but it is also CRAZY FAST slashed my runtime BY HALF!!!!!
            //TODO::maybe change the other for loop too later
            let repeat = (1..=length / 2)
                .filter(|&repeat| length % repeat == 0)
                .any(|repeat| {
                    let subseq = &num_str[..repeat];
                    num_str
                        .as_bytes()
                        .chunks(repeat)
                        .all(|chunk| chunk == subseq.as_bytes())
                });
            if repeat {
                count += num;
            }
        } else {
            if length % 2 != 0 {
                continue;
            }
            let mid = length / 2;
            if num_str[0..mid] == num_str[mid..length] {
                count += num;
            }
        }
    }
    count
}

fn main() {
    let start: Instant = Instant::now();
    let mut sum_pt1: i64 = 0;
    let mut sum_pt2: i64 = 0;
    let mut min_val: i64;
    let mut max_val: i64;
    let file: String = env::args().nth(1).expect("Please provide a file path");
    let contents: String = fs::read_to_string(file).expect("Could not read the file");
    let parts: Vec<&str> = contents.trim().split(',').collect();
    for part in &parts {
        //println!("Processing part: {}", part);
        let range_parts: Vec<&str> = part.split('-').collect();
        min_val = range_parts[0].trim().parse().expect("Not a number");
        max_val = range_parts[1].trim().parse().expect("Not a number");
        sum_pt1 += find_synonym(min_val, max_val, false); //flag false for part 1
        sum_pt2 += find_synonym(min_val, max_val, true);
    }
    println!("Part 1 sum is: {}", sum_pt1);
    println!("Part 2 sum is: {}", sum_pt2);
    let duration: std::time::Duration = start.elapsed();
    println!("Time taken is: {:?}", duration);
}
