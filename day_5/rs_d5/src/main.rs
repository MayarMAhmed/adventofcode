use ::std::fs;
use ::std::time::Instant;
use std::env;
//TODO: Find a way around the unwrap calls
#[inline(always)]
fn parse_file_contents(contents: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = contents.trim().split("\n\n").collect();
    let fresh_ranges: Vec<(i64, i64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let bounds: Vec<&str> = line.trim().split('-').collect();
            if bounds.len() == 2 {
                let start = bounds[0].parse::<i64>().ok()?;
                let end = bounds[1].parse::<i64>().ok()?;
                Some((start.min(end), start.max(end)))
            } else {
                None
            }
        })
        .collect();
    let ingredients: Vec<i64> = parts[1]
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap_or(0))
        .collect();
    (fresh_ranges, ingredients)
}

#[inline(always)]
fn is_fresh(range: &(i64, i64), ingredient: i64) -> bool {
    ingredient >= range.0 && ingredient <= range.1 //we did .1 because we want the index of that tuple in the list
}
#[inline(always)]
fn count_valid_ranges(ranges: &[(i64, i64)]) -> i64 {
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort(); //sort by first element of tuple
    let correct: Vec<(i64, i64)> =
        sorted_ranges
            .into_iter()
            .fold(Vec::new(), |mut acc, (start, end)| {
                if acc.is_empty() || acc.last().unwrap().1 < start {
                    //if the end of last range/max is less than min of current range always add it
                    acc.push((start, end));
                } else {
                    let last = acc.last_mut().unwrap(); //this is REALLY COOL last_mut allows me to get a reference to the last element in my vec so I can modify it directly
                    if end > last.1 {
                        last.1 = end;
                    }
                }
                acc
            });
    correct.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let start: Instant = Instant::now();
    let file: String = env::args().nth(1).expect("Please provide a file path");
    let contents: String = fs::read_to_string(file).expect("Could not read the file");
    let (fresh_ranges, ingredients) = parse_file_contents(&contents);
    let sum_pt1: i16 = ingredients
        .iter()
        .filter(|&&ingredient| fresh_ranges.iter().any(|range| is_fresh(range, ingredient)))
        .count() as i16;
    /*     let fresh_set: BTreeSet<i64> = fresh_ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .collect();
    let sum_pt2: i64 = fresh_set.iter().count() as i64;
    println!("Part 2: Sum of all fresh ingredients is {}", sum_pt2);
     */
    //see now u figured out the equilvalent way of infinite loops in rust lol
    let sum_pt2: i64 = count_valid_ranges(&fresh_ranges);
    println!("Part 1: Number of fresh ingredients is {}", sum_pt1);
    println!("Part 2:Total number of fresh ranges is {}", sum_pt2);
    let duration = start.elapsed();
    println!("Time taken is: {:?}", duration);
}
