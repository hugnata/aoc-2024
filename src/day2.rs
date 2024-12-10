use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Vec<Vec<i32>>;

#[inline]
fn str_to_u32(string: &str) -> i32 {
    string.to_string().parse().unwrap()
}


#[aoc_generator(day2)]
pub fn parse(input: &str) -> InputType {
    let mut results: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let result_line: Vec<i32> = line.split(" ").map(|x| str_to_u32(x)).collect();
        results.push(result_line);
    }
    results
}

fn is_safe(result_line: &Vec<i32>) -> bool {
    let rep_diff: Vec<i32> = result_line.windows(2).map(|slice| slice[1] - slice[0]).collect();
    let increasing = rep_diff.iter().filter(|x| **x > 0).count() == rep_diff.len();
    let decreasing = rep_diff.iter().filter(|x| **x < 0).count() == rep_diff.len();
    if ! increasing && ! decreasing {
        return false
    }

    let evolution_ok = rep_diff.iter().map(|x| x.abs()).filter(|x| *x > 3).count() == 0;
    if ! evolution_ok {
        return false
    }

    return true
}

fn is_safe_part_two(result_line: &Vec<i32>) -> bool {
    let rep_diff: Vec<i32> = result_line.windows(2).map(|slice| slice[1] - slice[0]).collect();
    let increasing_nb = rep_diff.iter().filter(|x| **x > 0).count();
    let decreasing_nb = rep_diff.iter().filter(|x| **x < 0).count();
    // Unsafe is not increasing or decreasing no matter if we remove some values
    if increasing_nb < rep_diff.len() - 1 && decreasing_nb < rep_diff.len() - 1 {
        // println!("{result_line:?} is Unsafe for sure : not increasing or decreasing");
        return false
    }

    if increasing_nb == rep_diff.len() - 1{
        let remove_index = rep_diff.iter().enumerate().find(|(_,x)| **x <= 0).map(|(i,_)|i).unwrap();
        let mut result_modified1 = result_line.clone();
        let mut result_modified2 = result_line.clone();
        result_modified1.remove(remove_index);
        if remove_index < result_line.len() - 1 {
            result_modified2.remove(remove_index +1);
        }
        return is_safe(&result_modified1) || is_safe(&result_modified2);
    }

    if decreasing_nb == rep_diff.len() - 1{
        let remove_index = rep_diff.iter().enumerate().find(|(_,x)| **x >= 0).map(|(i,_)|i).unwrap();
        // let mut result_modified = result_line.clone();
        // result_modified.remove(remove_index);
        // println!("The index to remove in {result_line:?} is {remove_index:?}, which is {:?}", result_line[remove_index]);
        let mut result_modified1 = result_line.clone();
        let mut result_modified2 = result_line.clone();
        result_modified1.remove(remove_index);
        if remove_index < result_line.len() - 1 {
            result_modified2.remove(remove_index +1);
        }
        return is_safe(&result_modified1) || is_safe(&result_modified2);
    }

    let evolution_nok_nb = rep_diff.iter().map(|x| x.abs()).filter(|x| *x > 3).count();
    if evolution_nok_nb == 0 {
        return true;
    } else if evolution_nok_nb == 1 {
        let remove_index = rep_diff.iter().enumerate().find(|(i,x)| x.abs() > 3).map(|(i,x)|i).unwrap();
        let mut result_modified1 = result_line.clone();
        let mut result_modified2 = result_line.clone();
        result_modified1.remove(remove_index);
        if remove_index < result_line.len() - 1 {
            result_modified2.remove(remove_index +1);
        }
        return is_safe(&result_modified1) || is_safe(&result_modified2);
    } else {
        return false;
    }
}




#[aoc(day2, part1)]
fn solve_part_one(input: &InputType) -> u32 {
    input.iter().map(|x| is_safe(x)).filter(|x| *x == true).count() as u32
}

#[aoc(day2, part2)]
fn solve_part_two(input: &InputType) -> u32 {
    input.iter().map(|x| is_safe_part_two(x)).filter(|x| *x == true).count() as u32
}

pub fn part1(input: &str) -> impl Display {
    solve_part_one(&parse(input))
}

pub fn part2(input: &str) -> impl Display {
    solve_part_two(&parse(input))
}
