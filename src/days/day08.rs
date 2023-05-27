#![allow(dead_code)]

fn build_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::<Vec<u8>>::new();

    let lines = input.lines();
    for line in lines {
        let mut row = Vec::<u8>::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<u8>().unwrap());
        }
        grid.push(row);
    }

    grid
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn check_top(grid: &[Vec<u8>], pos: &Pos) -> bool {
    let tree = grid[pos.y][pos.x];
    grid.iter()
        .enumerate()
        .take(pos.y)
        .all(|(_y, row)| row[pos.x] < tree)
}

fn check_bottom(grid: &[Vec<u8>], pos: &Pos) -> bool {
    let tree = grid[pos.y][pos.x];
    grid.iter()
        .enumerate()
        .skip(pos.y + 1)
        .all(|(_y, row)| row[pos.x] < tree)
}

fn check_left(grid: &[Vec<u8>], pos: &Pos) -> bool {
    let tree = grid[pos.y][pos.x];
    for i in 0..pos.x {
        if grid[pos.y][i] >= tree {
            return false;
        }
    }
    true
}

fn check_right(grid: &[Vec<u8>], pos: &Pos) -> bool {
    let tree = grid[pos.y][pos.x];
    for i in (pos.x + 1)..grid[pos.y].len() {
        if grid[pos.y][i] >= tree {
            return false;
        }
    }
    true
}

fn is_visible(grid: &[Vec<u8>], pos: &Pos) -> bool {
    check_top(grid, pos)
        || check_bottom(grid, pos)
        || check_left(grid, pos)
        || check_right(grid, pos)
}

fn trees_visible_up(grid: &[Vec<u8>], pos: &Pos) -> u32 {
    let tree = grid[pos.y][pos.x];
    grid.iter()
        .take(pos.y)
        .take_while(|&row| row[pos.x] < tree)
        .count() as u32
}

fn trees_visible_down(grid: &[Vec<u8>], pos: &Pos) -> u32 {
    let tree = grid[pos.y][pos.x];
    grid.iter()
        .skip(pos.y + 1)
        .take_while(|&row| row[pos.x] < tree)
        .count() as u32
}

fn trees_visible_left(grid: &[Vec<u8>], pos: &Pos) -> u32 {
    let tree = grid[pos.y][pos.x];
    let mut count: u32 = 0;

    for i in (0..pos.x).rev() {
        count += 1;
        if grid[pos.y][i] >= tree {
            return count;
        }
    }
    count
}

fn trees_visible_right(grid: &[Vec<u8>], pos: &Pos) -> u32 {
    let tree = grid[pos.y][pos.x];
    grid[pos.y][pos.x + 1..]
        .iter()
        .take_while(|&cell| *cell < tree)
        .count() as u32
}

pub fn part1(input: &str) -> String {
    let grid = build_grid(input);
    let mut visible_count = &grid.len() * 2 + &grid[0].len() * 2 - 4;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let pos = Pos { x, y };
            if is_visible(&grid, &pos) {
                visible_count += 1
            }
        }
    }
    visible_count.to_string()
}

pub fn part2(input: &str) -> String {
    let grid = build_grid(input);
    let mut max_score: u32 = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let pos = Pos { x, y };

            let up = trees_visible_up(&grid, &pos);
            let down = trees_visible_down(&grid, &pos);
            let left = trees_visible_left(&grid, &pos);
            let right = trees_visible_right(&grid, &pos);

            let score = up * down * left * right;

            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "8");
    }
}
