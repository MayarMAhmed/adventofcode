use std::env; //to read command line arguments i.e argpase 
use std::fs; //to read file
use std::time::Instant; //to get time duration of execution

fn move_head(direction: &str, step: i32, intial_pos: i32) -> (i32, i32) {
    let mut counter: i32 = 0;
    let mut value: i32 = 0;
    if matches!(direction, "R") {
        value = intial_pos + step;
    } else if matches!(direction, "L") {
        value = intial_pos - step;
    } else {
        println!("Invalid direction");
    }
    while !(0..=99).contains(&value) { // the idiomatic way to check range in rust so it is saying while value not in 0 to 99 loop thx clippy
        counter += 1;
        if value == 100 {
            value = 0;
        }
        if value < 0 {
            value += 100;
        } else if value > 100 {
            value -= 100;
        }
    }
    (value, counter)
}
//let creates new variable, mut makes it mutable so u can't use let when changing value
fn main() {
    let start = Instant::now(); //start time measurement
    let mut inital_postion: i32 = 50;
    let mut c: i32 = 0;
    let mut zer: i32 = 0;
    let file_path = env::args().nth(1).expect("cargo run <file_path>"); //pip the 1st argument as file path
    println!("You entered: {}", file_path.trim()); //remove empty spaces
    //read file and process it
    let contents = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    let lines = contents.lines(); //read lines from file
    for line in lines {
        let (direct, step_str) = line.split_at(1); //split by 1st character
        let step: i32 = step_str.trim().parse().expect("Not a number"); //convert string to integer
        //println!("Direction: {}, Steps: {}", direct, step);
        let (new_pos, counter) = move_head(direct, step, inital_postion);
        inital_postion = new_pos;
        if inital_postion == 0 {
            c += 1;
        }
        zer += counter;
        //println!("New Position: {}, Counter: {}", inital_postion, c);
    }
    let duration = start.elapsed(); //end time measurement

    println!(
        "Total times crossed zero: {}, Total adjustments made: {}",
        c, zer
    );
    println!("Time taken is: {:?}", duration);
}

