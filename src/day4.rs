use aoc_runner_derive::{aoc, aoc_generator};

type Grid = Vec<Vec<char>>;
type InputType = Grid;

fn search_direction(grid: &Grid, tokens: &[char], i: usize, j: usize, delta_i: i32, delta_j: i32) -> bool {
    let i = (i as i32 + delta_i) as usize;
    let j = (j as i32 + delta_j) as usize;
    // If outside bounds, return
    if i >= grid.len() || j >= grid[i].len()  {
        return false
    }
    // If the symbol is not the one we expected, return
    if grid[i][j] != *tokens.first().unwrap() {
        return false
    }
    // If not more token to look for, we found our string
    let remaining_tokens = &tokens[1..tokens.len()];
    if remaining_tokens.len() == 0 {
        return true;
    }

    return search_direction(grid, remaining_tokens, i, j, delta_i, delta_j)
}

fn search_from(grid: &Grid, tokens: &[char], i: usize,j: usize) -> u32 {
    // Right
    let ru = search_direction(grid, tokens, i, j, -1, 1);
    let r = search_direction(grid, tokens, i, j, 0, 1);
    let rd = search_direction(grid, tokens, i, j, 1, 1);
    // Left
    let lu = search_direction(grid, tokens, i, j, -1, -1);
    let l = search_direction(grid, tokens, i, j, 0, -1);
    let ld = search_direction(grid, tokens, i, j, 1, -1);
    // Mid
    let mu = search_direction(grid, tokens, i, j, 1, 0);
    let md = search_direction(grid, tokens, i, j, -1, 0);

    let results = [ru, r, rd, lu, l, ld, mu, md];
    return results.map(|b| b as u32 ).iter().sum();
}

fn search_x_from(grid: &Grid, i: usize,j: usize) -> u32 {
    //     M
    //   A
    // S
    let rum = search_direction(grid, &['M'], i, j, -1, 1);
    let lds = search_direction(grid, &['S'], i, j, 1, -1);
    let ldru_diagonal = rum && lds;
    //     S
    //   A
    // M
    let rus = search_direction(grid, &['S'], i, j, -1, 1);
    let ldm = search_direction(grid, &['M'], i, j, 1, -1);
    let ldru_diagonal = ldru_diagonal | (rus && ldm);

    // M
    //   A
    //      S
    let lum = search_direction(grid, &['M'], i, j, -1, -1);
    let rds = search_direction(grid, &['S'], i, j, 1, 1);
    let lurd_diagonal = lum && rds;
    // S
    //   A
    //      M
    let lus = search_direction(grid, &['S'], i, j, -1, -1);
    let rdm = search_direction(grid, &['M'], i, j, 1, 1);
    let lurd_diagonal = lurd_diagonal | (lus && rdm);

    return (lurd_diagonal && ldru_diagonal) as u32;
}


#[aoc_generator(day4)]
pub fn parse(input: &str) -> InputType {
    let mut rows = vec![];
    for line in input.lines() {
        let cols = line.chars().collect();
        rows.push(cols);
    }
    return rows;
}

#[aoc(day4, part1)]
fn solve_part_one(grid: &InputType) -> u32 {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                let result = search_from(&grid, &['M', 'A', 'S'], i, j);
                sum += result
            }
        }
    }

    return sum as u32;
}

#[aoc(day4, part2)]
fn solve_part_two(grid: &InputType) -> u32 {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                let result = search_x_from(&grid, i, j);
                sum += result
            }
        }
    }

    sum
}

pub fn part1(input: &str) -> u32 {
    solve_part_one(&parse(input))
}

pub fn part2(input: &str) -> u32 {
    solve_part_two(&parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_input() {
        let data = include_str!("../examples/day4.txt");
        assert_eq!(solve_part_one(&parse(data)), 18);
    }

    #[test]
    fn part_two_input() {
        let data = include_str!("../examples/day4.txt");
        assert_eq!(solve_part_two(&parse(data)), 9);
    }
}
