use std::iter::zip;

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = (Vec<i32>, Vec<i32>);

fn str_to_u32(string: &str) -> i32 {
    string.to_string().parse().unwrap()
}

#[aoc_generator(day1)]
pub fn parse(input: &str) -> InputType {
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];
    for line in input.lines() {
        let mut it = line.split("   ");
        let first = str_to_u32(it.next().unwrap());
        let second: i32 = str_to_u32(it.next().unwrap());
        first_list.push(first);
        second_list.push(second);
    }
    first_list.sort();
    second_list.sort();
    (first_list, second_list)
}

#[aoc(day1, part1)]
fn solve_part_one(input: &InputType) -> u32 {
    zip(&input.0, &input.1).map(|(a, b)| { (a-b).abs() as u32}).sum()
}

#[aoc(day1, part2)]
fn solve_part_two(input: &InputType) -> u32 {
    let mut sum = 0;
    for val in &input.0 {
        let nb_in_right_list = input.1.iter().filter(|x| **x == *val).count() as u32;
        let result = (*val as u32) * nb_in_right_list;
        sum += result;
    }
    sum
}

pub fn part1(input: &str) -> u32 {
    solve_part_one(&parse(input))
}

pub fn part2(input: &str) -> u32 {
    solve_part_two(&parse(input))
}
