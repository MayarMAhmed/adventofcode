use ::rayon::prelude::*; //trying the parallel processing
use ::std::env;
use ::std::fs;
use ::std::time::Instant;
/* this has lots of duplicate code so I will refactor it and make it efficient
#[inline(always)]
//use rayon to read the file in parallel and turn each line into a grid of string digits ,empty spaces are included AND SYMBOLS are IGNORED
fn parse_file(contents: &str) -> Vec<Vec<String>> {
    let grid: Vec<Vec<String>> = contents
        .par_lines() //parallel iterator over lines
        .map(|line| {
            line.chars()
                .map(|char| char.to_string())
                .collect::<Vec<String>>() //collect each char as string into a vec
        })
        .collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    //transpose the grid
    let transposed_grid: Vec<Vec<String>> = (0..num_cols)
        .into_par_iter()
        .map(|col_index| {
            (0..num_rows)
                .map(|row_index| grid[row_index][col_index].clone())
                .collect()
        })
        .collect();
    transposed_grid
}

#[inline(always)]
fn extract_problems(grid: &Vec<Vec<String>>) -> Vec<Vec<Vec<&String>>> {
    //split the grid into problems separated by empty rows
    let split_vec: Vec<&[Vec<String>]> = grid
        .split(|row| row.iter().all(|cell| cell == " "))
        .collect();
    //println!("{:?}", split_vec);
    let problems: Vec<Vec<Vec<&String>>> = split_vec
        .par_iter()
        .map(|prob| {
            prob.iter()
                .map(|row| row.iter().collect::<Vec<&String>>())
                .collect::<Vec<Vec<&String>>>()
        })
        .collect();
    problems
}

#[inline(always)]
fn do_hw(problems: &[Vec<Vec<&String>>]) -> i64 {
    problems
        .par_iter()
        .map(|problem| {
            let num_rows = problem.len();
            let num_cols = problem[0].len();
            let rows: Vec<&Vec<&String>> =
                problem.par_iter().filter(|r| r.len() == num_cols).collect();

            let action_col = num_cols - 1;

            let action: char = rows
                .iter()
                .flat_map(|row| row[action_col].chars())
                .find(|&c| c == '+' || c == '*')
                .unwrap_or('?');

            // Build values by moving top to bottom i.e each column
            let values: Vec<i64> = (0..action_col)
                .into_par_iter()
                .filter_map(|c| {
                    let digits: String = rows
                        .iter()
                        .flat_map(|row| row[c].chars())
                        .filter(|ch| ch.is_ascii_digit())
                        .collect();

                    digits.parse::<i64>().ok()
                })
                .collect();

            match action {
                '+' => values.into_iter().sum::<i64>(),
                '*' => values.into_iter().product::<i64>(),
                _ => 0i64,
            }
        })
        .sum()
}

#[inline(always)]
fn do_weird_hw(problems: &[Vec<Vec<&String>>]) -> i64 {
    problems
    .par_iter()
    .map(|problem| {
        let num_rows = problem.len();
        let num_cols = problem[0].len();
        let rows: Vec<&Vec<&String>> =
            problem.par_iter().filter(|r| r.len() == num_cols).collect();
        let action_col = num_cols - 1;

        let action: char = rows
            .iter()
            .flat_map(|row| row[action_col].chars())
            .find(|&c| c == '+' || c == '*')
            .unwrap_or('?');

        // Build values by moving left to right i.e each row
        let values: Vec<i64> = (0..action_col)
            .into_par_iter()
            .filter_map(|r| {
                let digits: String = rows[r]
                    .iter()
                    .flat_map(|cell| cell.chars())
                    .filter(|ch| ch.is_ascii_digit())
                    .collect();

                digits.parse::<i64>().ok()
            })
            .collect();
        println!("Weird values: {:?}", values);
        println!("Weird action: {}", action);


        match action {
            '+' => values.into_iter().sum::<i64>(),
            '*' => values.into_iter().product::<i64>(),
            _ => 0i64,
        }
    })
    .sum()
}

fn main() {
    let start = Instant::now();
    let file = env::args().nth(1).expect("cargo run <file_path>");
    let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");
    let grid = parse_file(&contents);
    let problems = extract_problems(&grid);

    let output_pt1 = do_hw(&problems);
    let output_pt2 = do_weird_hw(&problems);

    let duration = start.elapsed();
    println!("Output for normal math pt1: {}", output_pt1);
    println!("Output for weird math pt2: {}", output_pt2);
    println!("Time elapsed in total is: {:?}", duration);
}
*/


#[derive(Copy, Clone)]
enum Mode {
    Normal, // build numbers top->bottom in each column
    Weird,  // build numbers left->right in each row 
}

#[inline(always)]
fn parse_file(contents: &str) -> Vec<Vec<char>> {
    // read into grid of chars because this is faster than strings as it, fixed-size data 
    let grid: Vec<Vec<char>> = contents
        .par_lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    (0..num_cols)
        .into_par_iter()
        .map(|col_index| {
            (0..num_rows)
                .map(|row_index| grid[row_index][col_index])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

#[inline(always)]
fn extract_problems<'a>(grid_t: &'a [Vec<char>]) -> Vec<&'a [Vec<char>]> {
    grid_t
        .split(|row| row.iter().all(|&cell| cell == ' '))
        .collect()
}
//Normal math build numbers top->bottom in each column
#[inline(always)]
fn build_values_by_column(rows: &[&[char]], action_col: usize) -> Vec<i64> {
    (0..action_col)
        .filter_map(|c| {
            let mut digits = String::new();
            for row in rows {
                if let Some(&ch) = row.get(c) {
                    if ch.is_ascii_digit() {
                        digits.push(ch);
                    }
                }
            }
            digits.parse::<i64>().ok()
        })
        .collect()
}

//Weird math build numbers left->right in each row
#[inline(always)]
fn build_values_by_row(rows: &[&[char]], action_col: usize) -> Vec<i64> {
    (0..action_col)
        .filter_map(|r| {
            let row = rows.get(r)?;
            let mut digits = String::new();
            for &ch in row.iter() {
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            digits.parse::<i64>().ok()
        })
        .collect()
}

#[inline(always)]
fn solve_total(problems: &[&[Vec<char>]], mode: Mode) -> i64 {
    problems.par_iter().map(|p| solve(p, mode)).sum()
}

#[inline(always)]
fn solve(problem: &[Vec<char>], mode: Mode) -> i64 {
    let num_cols = problem[0].len();
    if num_cols == 0 {
        return 0;
    }

    let rows: Vec<&[char]> = problem
        .iter()
        .map(|r| r.as_slice())
        .filter(|r| r.len() == num_cols)
        .collect();

    let action_col = num_cols - 1;

    let action = rows
        .iter()
        .filter_map(|row| row.get(action_col).copied())
        .find(|&c| c == '+' || c == '*')
        .unwrap_or('?');

    let values: Vec<i64> = match mode {
        Mode::Normal => build_values_by_column(&rows, action_col),
        Mode::Weird => build_values_by_row(&rows, action_col),
    };

    match action {
        '+' => values.into_iter().sum(),
        '*' => values.into_iter().product(),
        _ => 0,
    }
}


fn main() {
    let start = Instant::now();

    let file = env::args().nth(1).expect("cargo run <file_path>");
    let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");

    let grid_t = parse_file(&contents);
    let problems = extract_problems(&grid_t);

    let output_pt1 = solve_total(&problems, Mode::Normal);
    let output_pt2 = solve_total(&problems, Mode::Weird);

    println!("Output for normal math pt1: {}", output_pt1);
    println!("Output for weird math pt2: {}", output_pt2);
    println!("Time elapsed in total is: {:?}", start.elapsed());
}
