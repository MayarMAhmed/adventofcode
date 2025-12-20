use std::env;
use std::fs;
use std::time::Instant;
use std::ops::AddAssign;
use enum_display::EnumDisplay;
use std::collections::BTreeSet; //sets in rust that are ordered so that i can get min and max values easily using fist()/min and last()/max
//easier way to implement  std::fmt::Display on enum 
#[derive(EnumDisplay)]
enum Output {
    Int(i32),
    Long(i64),
}

// as this is my sort of class method to handle multiple types of output. I 
// need to implement (overload) the add assign operator for this enum so that I can
// add two Output values together. 
impl AddAssign for Output {
    fn add_assign(&mut self, other: Self) {
        match (self, other) {
            (Output::Int(a), Output::Int(b)) => *a += b,
            (Output::Long(a), Output::Long(b)) => *a += b,
            _ => panic!("Cannot add different Output types"),
        }
    }
}

/* impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Output::Int(n) => write!(f, "{}", n),
            Output::Long(n) => write!(f, "{}", n),
        }
    }
} */

fn find_max_subseq(input_line: &str, kmer: i8) -> Output {
    //check if kmer is < 4 then the output is int
    let length: i32 =input_line.chars().count().try_into().unwrap();
    let mut removed_chars: i32 = length-kmer as i32; //bc i save kmer as i8
    if kmer <= 4 {
        let mut combinations: BTreeSet<i32> = BTreeSet::new();
        let chars: Vec<char> = input_line.chars().collect();
        for i in 0..chars.len() {
            for j in (i+1)..chars.len() {
                let value = format!("{}{}", chars[i], chars[j]) ;
                //println!("Generated combination: {}", value);
                if let Ok(num) = value.parse::<i32>() {
                    combinations.insert(num);
                }
            }
        }
        //println!("the max subseq is: {:?}", combinations.last());
        Output::Int(combinations.last().copied().unwrap_or(0)) //bc .last returns option type i.e reference to the last element so use copied to get the value
    } else {
        let mut stack: Vec<i64> = Vec::new();
        let chars: Vec<char> = input_line.chars().collect();
        for i in chars.iter() {
            let digit = i.to_digit(10).unwrap() as i64;
            while !stack.is_empty() && removed_chars > 0 && *stack.last().unwrap() < digit as i64 {
                stack.pop();
                removed_chars -= 1;
            }
            stack.push(digit);
        }
        //if after the loop we still have to remove some digits, remove them from the end
        if removed_chars > 0 {
            // removed_chars is "how many to remove", so keep len - removed_chars
            let new_len = stack.len().saturating_sub(removed_chars as usize);
            stack.truncate(new_len);
        }
        //now combine the digits in the stack to form the number 
        let max_sum: i64 = stack.iter().fold(0i64, |acc, &x| acc * 10 + x as i64);
        Output::Long(max_sum)
    }
}

fn main() {
    let start = Instant::now();
    let mut p1_sum:i32 = 0;
    let mut p2_sum: i64 = 0;
    let file_path = env::args().nth(1).expect("cargo run <file_path>");
    let content = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    for line in content.lines() {
        if let Output::Int(val) = find_max_subseq(line, 2) {
            p1_sum += val;
        }
        if let Output::Long(val2) = find_max_subseq(line, 12) {
            p2_sum += val2;
        }
    }
    let duration = start.elapsed();
    println!("Part 1: {}, Part 2: {}", p1_sum, p2_sum);
    println!("Time taken is: {:?}", duration);
}
