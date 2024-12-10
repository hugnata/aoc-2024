use std::{collections::HashSet, fmt::Display, ops::Index};

type Grid = Vec<Vec<i32>>;

struct GridOpti {
    pub width: usize,
    pub height: usize,
    data: Vec<i32>
}

impl Index<usize> for GridOpti {
    type Output = [i32];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index *self.width..]
    }
}

const NOT_VISITED: bool = false;
const VISITED: bool = true;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect()
}

fn explore_part_two(grid: &Grid, x: usize, y: usize, last_height: i32) -> i32 {
    let height = grid[x][y];
    if height != last_height + 1 {
        return 0;
    }
    if height == 9 {
        return 1;
    }
    let up = if x > 0 { explore_part_two(grid, x-1, y, height)} else {0};
    let down = if x < grid.len() - 1 { explore_part_two(grid, x+1, y, height)} else {0};
    let left = if y > 0 { explore_part_two(grid, x, y-1, height)} else {0};
    let right = if y < grid[0].len() - 1  { explore_part_two(grid, x, y+1, height)} else {0};
    return up + down + left + right;
}

#[aoc(day10, part1, custom_hash)]
fn solve_part_one_custom_hash(grid: &Grid) -> i32 {
    let mut starting_positions: Vec<(usize, usize, i32)> = vec![];
    let width = grid[0].len();
    for x in 0..grid.len() {
        for y in 0..width {
            if grid[x][y] == 0 {
                starting_positions.push((x, y, 0))
            }
        }
    }

    let grid_v_limit = grid.len() - 1;
    let grid_h_limit = grid[0].len() - 1;

    let mut sum_reachable = 0;

    for starting_pos in starting_positions {
        let mut visited = vec![false; width* grid.len()];
        let mut reachable = vec![starting_pos];
        while let Some((x, y, height)) = reachable.pop() {
            if height == 9 {
                sum_reachable += 1;
                continue;
            }
            let next_height = height + 1;
            // UP
            if x > 0 && grid[x - 1][y] == next_height && !visited[(x-1)*width +y] {
                reachable.push((x - 1, y, next_height));
                visited[(x-1)*width +y] = VISITED;
            }
            // DOWN
            if x < grid_v_limit && grid[x + 1][y] == next_height && !visited[(x+1)*width +y] {
                reachable.push((x + 1, y, next_height));
                visited[(x+1)*width +y] = VISITED;

            }
            // LEFT
            if y > 0 && grid[x][y - 1] == next_height &&  !visited[x*width +y -1] {
                reachable.push((x, y - 1, next_height));
                visited[x*width +y -1] = VISITED

            }
            // RIGHT
            if y < grid_h_limit && grid[x][y + 1] == next_height && !visited[x*width +y + 1] {
                reachable.push((x, y + 1, next_height));
                visited[x*width +y+1] = VISITED
            }
        }
    }

    return sum_reachable;
}


#[aoc(day10, part2, naive)]
fn solve_part_two(grid: &Grid) -> i32 {
    let mut sum = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 0 {
                sum += explore_part_two(grid, x, y, -1);
            }
        }
    }
    sum
}

pub fn part1(input: &str) -> impl Display {
    solve_part_one_custom_hash(&parse(input))
}

pub fn part2(input: &str) -> impl Display {
    solve_part_two(&parse(input))
}

/// -----------------------------------  Solutions not chosen because of performance --------------------
fn explore_part_one(grid: &Grid, x: usize, y: usize, start_height: i32) -> u32 {
    let mut visited = HashSet::new();
    let mut reachable = vec![(x, y, start_height)];

    let mut sum_reachable = 0;
    while let Some((x, y, height)) = reachable.pop() {
        visited.insert((x, y));
        if height == 9 {
            sum_reachable += 1;
            continue;
        }
        // UP
        if x > 0  && grid[x - 1][y] == height + 1 && !visited.contains(&(x - 1, y)) {
            reachable.push((x - 1, y, height + 1));
        }
        // DOWN
        if x < grid.len() - 1 && grid[x + 1][y] == height + 1 && !visited.contains(&(x + 1, y)) {
            reachable.push((x + 1, y, height + 1));
        }
        // LEFT
        if y > 0  && grid[x][y - 1] == height + 1 && !visited.contains(&(x, y - 1)){
            reachable.push((x, y - 1, height + 1));
        }
        // RIGHT
        if y < grid[0].len() - 1  && grid[x][y + 1] == height + 1 && !visited.contains(&(x, y + 1)) {
            reachable.push((x, y + 1, height + 1));
        }
    }
    return sum_reachable;
}

#[aoc(day10, part1, naive)]
fn solve_part_one_naive(input: &Grid) -> i32 {
    let mut sum = 0;
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if input[x][y] == 0 {
                let val = explore_part_one(input, x, y, 0);
                sum += val;
            }
        }
    }
    return sum as i32;
}

#[aoc(day10, part1, loop_iter)]
fn solve_part_one_loop_iter(grid: &Grid) -> i32 {
    let mut starting_positions: Vec<(usize, usize, i32)> = vec![];
    grid.iter()
        .enumerate()
        .map(|(x, v)| {
            v.iter().enumerate().for_each(|(y, val)| {
                if *val == 0 {
                    starting_positions.push((x, y, 0));
                }
            })
        })
        .for_each(drop);

    let grid_v_limit = grid.len() - 1;
    let grid_h_limit = grid[0].len() - 1;
    let mut sum_reachable = 0;
    for starting_pos in starting_positions {
        let mut visited = HashSet::new();
        let mut reachable = vec![starting_pos];
        while let Some((x, y, height)) = reachable.pop() {
            if height == 9 {
                sum_reachable += 1;
                continue;
            }
            let next_height = height + 1;
            // UP
            if x > 0 && grid[x - 1][y] == next_height && !visited.contains(&(x - 1, y)) {
                reachable.push((x - 1, y, next_height));
                visited.insert((x-1, y));
            }
            // DOWN
            if x < grid_v_limit && grid[x + 1][y] == next_height && !visited.contains(&(x + 1, y)) {
                reachable.push((x + 1, y, next_height));
                visited.insert((x+1, y));

            }
            // LEFT
            if y > 0 && grid[x][y - 1] == next_height && !visited.contains(&(x, y - 1)) {
                reachable.push((x, y - 1, next_height));
                visited.insert((x, y-1));

            }
            // RIGHT
            if y < grid_h_limit && grid[x][y + 1] == next_height && !visited.contains(&(x, y + 1)) {
                reachable.push((x, y + 1, next_height));
                visited.insert((x, y+1));
            }
        }
    }
    return sum_reachable;
}


#[aoc(day10, part1, loop_version_no_set)]
fn solve_part_one_bool_instead_of_set(grid: &Grid) -> i32 {
    let mut starting_positions: Vec<(usize, usize, i32)> = vec![];
    let width = grid[0].len();

    for x in 0..grid.len() {
        for y in 0..width {
            if grid[x][y] == 0 {
                starting_positions.push((x, y, 0))
            }
        }
    }

    let grid_v_limit = grid.len() - 1;
    let grid_h_limit = grid[0].len() - 1;

    let mut sum_reachable = 0;

    for starting_pos in starting_positions {
        let mut visited = vec![vec![false;width]; grid.len()];
        let mut reachable = vec![starting_pos];
        while let Some((x, y, height)) = reachable.pop() {
            if height == 9 {
                sum_reachable += 1;
                continue;
            }
            let next_height = height + 1;
            // UP
            if x > 0 && grid[x - 1][y] == next_height && visited[x-1][y] == NOT_VISITED {
                reachable.push((x - 1, y, next_height));
                visited[x-1][y] = VISITED;
            }
            // DOWN
            if x < grid_v_limit && grid[x + 1][y] == next_height && visited[x + 1][y] == NOT_VISITED {
                reachable.push((x + 1, y, next_height));
                visited[x+1][y] = VISITED;

            }
            // LEFT
            if y > 0 && grid[x][y - 1] == next_height && visited[x][y-1] == NOT_VISITED {
                reachable.push((x, y - 1, next_height));
                visited[x][y-1] = VISITED;

            }
            // RIGHT
            if y < grid_h_limit && grid[x][y + 1] == next_height &&  visited[x][y+1] == NOT_VISITED {
                reachable.push((x, y + 1, next_height));
                visited[x][y+1] = VISITED;
            }
        }
    }

    return sum_reachable;
}

#[aoc_generator(day10, part1, opti_grid)]
fn parse_opti(input: &str) -> GridOpti {
    let width = input.lines().peekable().peek().unwrap().len();
    println!("width is {width}");
    let data: Vec<i32> = input
        .chars().filter(|c| *c != '\n')
        .map(|c| c as i32 - '0' as i32).collect();
    let height = data.len() /width;
    GridOpti {
        data: data,
        width: width,
        height: height
    }
}

#[aoc(day10, part1, opti_grid)]
fn solve_part_one_custom_hash_opti_grid(grid: &GridOpti) -> i32 {
    let mut starting_positions: Vec<(usize, usize, i32)> = vec![];
    let width = grid.width;
    for x in 0..grid.height {
        for y in 0..grid.width {
            if grid[x][y] == 0 {
                starting_positions.push((x, y, 0))
            }
        }
    }

    let grid_v_limit = grid.height - 1;
    let grid_h_limit = grid.width - 1;

    let mut sum_reachable = 0;

    for starting_pos in starting_positions {
        let mut visited = vec![false; grid.width * grid.height];
        let mut reachable = vec![starting_pos];
        while let Some((x, y, height)) = reachable.pop() {
            if height == 9 {
                sum_reachable += 1;
                continue;
            }
            let next_height = height + 1;
            // UP
            if x > 0 && grid[x - 1][y] == next_height && !visited[(x-1)*width +y] {
                reachable.push((x - 1, y, next_height));
                visited[(x-1)*width +y] = VISITED;
            }
            // DOWN
            if x < grid_v_limit && grid[x + 1][y] == next_height && !visited[(x+1)*width +y] {
                reachable.push((x + 1, y, next_height));
                visited[(x+1)*width +y] = VISITED;

            }
            // LEFT
            if y > 0 && grid[x][y - 1] == next_height &&  !visited[x*width +y -1] {
                reachable.push((x, y - 1, next_height));
                visited[x*width +y -1] = VISITED

            }
            // RIGHT
            if y < grid_h_limit && grid[x][y + 1] == next_height && !visited[x*width +y + 1] {
                reachable.push((x, y + 1, next_height));
                visited[x*width +y+1] = VISITED
            }
        }
    }

    return sum_reachable;
}

#[aoc(day10, part1, loop_version)]
fn solve_part_one(grid: &Grid) -> i32 {
    let mut starting_positions: Vec<(usize, usize, i32)> = vec![];
    let width = grid[0].len();
    for x in 0..grid.len() {
        for y in 0..width {
            if grid[x][y] == 0 {
                starting_positions.push((x, y, 0))
            }
        }
    }

    let grid_v_limit = grid.len() - 1;
    let grid_h_limit = grid[0].len() - 1;

    let mut sum_reachable = 0;

    for starting_pos in starting_positions {
        let mut visited = HashSet::new();
        let mut reachable = vec![starting_pos];
        while let Some((x, y, height)) = reachable.pop() {
            if height == 9 {
                sum_reachable += 1;
                continue;
            }
            let next_height = height + 1;
            // UP
            if x > 0 && grid[x - 1][y] == next_height && !visited.contains(&(x - 1, y)) {
                reachable.push((x - 1, y, next_height));
                visited.insert((x-1, y));
            }
            // DOWN
            if x < grid_v_limit && grid[x + 1][y] == next_height && !visited.contains(&(x + 1, y)) {
                reachable.push((x + 1, y, next_height));
                visited.insert((x+1, y));

            }
            // LEFT
            if y > 0 && grid[x][y - 1] == next_height && !visited.contains(&(x, y - 1)) {
                reachable.push((x, y - 1, next_height));
                visited.insert((x, y-1));

            }
            // RIGHT
            if y < grid_h_limit && grid[x][y + 1] == next_height && !visited.contains(&(x, y + 1)) {
                reachable.push((x, y + 1, next_height));
                visited.insert((x, y+1));
            }
        }
    }

    return sum_reachable;
}



#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    const EXAMPLE_0: &str = "9990999
9991999
9992999
6543456
7999997
8999998
9999999
";

const EXAMPLE_1: &str = "9999909
9943219
9959929
9965439
9979949
9987659
9999999
";

    #[test]
    fn part1_naive() {
        assert_eq!(solve_part_one_naive(&parse(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one_naive(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part1_iter() {
        assert_eq!(solve_part_one_loop_iter(&parse(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one_loop_iter(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part1_loop() {
        assert_eq!(solve_part_one(&parse(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part1_no_set() {
        assert_eq!(solve_part_one_bool_instead_of_set(&parse(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one_bool_instead_of_set(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part1_custom_hash() {
        assert_eq!(solve_part_one_custom_hash(&parse(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one_custom_hash(&parse(EXAMPLE)), 36);
    }


    #[test]
    fn part1_opti_grid() {
        assert_eq!(solve_part_one_custom_hash_opti_grid(&parse_opti(EXAMPLE_0)), 4);
        assert_eq!(solve_part_one_custom_hash_opti_grid(&parse_opti(EXAMPLE)), 36);
        assert_eq!(solve_part_one_custom_hash_opti_grid(&parse_opti(include_str!("../input/2024/day10.txt"))), 512);
    }


    #[test]
    fn part1_example() {
        assert_eq!(solve_part_one(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part1_input() {
        assert_eq!(solve_part_one(&parse(include_str!("../input/2024/day10.txt"))), 512);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part_two(&parse(EXAMPLE_1)), 6);
    }

    #[test]
    fn part2_input() {
        assert_eq!(solve_part_two(&parse(include_str!("../input/2024/day10.txt"))), 1045);
    }
}
