use std::{thread::sleep, time::Duration};

use anyhow::Result;

#[allow(dead_code, unused_variables)]
fn pt1(grid: &Vec<Vec<String>>) -> Result<()> {
    let mut splits = 0;
    let mut grid = grid.clone();
    for row in 1..grid.len() {
        let prev_row = grid[row - 1].clone();
        let curr_row = grid[row].clone();
        let mut new_row = grid[row].clone();
        for (colidx, c) in curr_row.iter().enumerate() {
            if *c == "^".to_string() && prev_row[colidx] == "|".to_string() {
                new_row[colidx - 1] = "|".to_string();
                new_row[colidx + 1] = "|".to_string();
                splits += 1;
                continue;
            }
            if prev_row[colidx] == "|".to_string() || prev_row[colidx] == "S".to_string() {
                new_row[colidx] = "|".to_string()
            }
        }
        grid[row] = new_row;
    }
    println!("{}", splits);
    Ok(())
}

fn display_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", format!("{}", row.iter().collect::<String>()))
    }
}

fn traverse(grid: &Vec<Vec<char>>, finished_count: &mut u64, x: usize, y: usize) {
    let grid_height = grid.len();
    let mut y = y;
    loop {
        if y >= grid_height {
            *finished_count += 1;
            println!("{}", finished_count);
            return;
        }
        // display_grid(&grid);
        // println!("finished beams: {}", finished_count);
        // sleep(Duration::from_millis(20));
        // sleep(Duration::from_secs(1));
        if grid[y][x] == '^' {
            traverse(&grid, finished_count, x - 1, y);
            traverse(&grid, finished_count, x + 1, y);
            break;
        }
        y += 1;
    }
}

#[allow(dead_code, unused_variables)]
fn pt2(grid: &Vec<Vec<char>>) -> Result<()> {
    let start_x = grid[0].iter().position(|s| *s == 'S').unwrap();
    let start_y = 1;
    let mut finished_count = 0;
    traverse(grid, &mut finished_count, start_x, start_y);
    println!("{}", finished_count);
    Ok(())
}

#[allow(dead_code, unused_variables)]
fn pt22(grid: &Vec<Vec<char>>) -> Result<()> {
    let row_len = grid[0].len();
    let mut counts: Vec<u64> = vec![0; grid.len() - 1];
    let mut next_start_col = 0;
    for (ri, row) in grid.iter().enumerate() {
        for ci in 0..row_len {
            if ci < next_start_col {
                continue;
            }
            if row[ci] == 'S' {
                counts[ci] += 1;
                next_start_col = row.iter().position(|&c| c == 'S').unwrap() - 1;
            }
            if row[ci] == '^' {
                counts[ci - 1] += counts[ci];
                counts[ci + 1] += counts[ci];
                counts[ci] = 0;
                next_start_col = row.iter().position(|&c| c == '^').unwrap() - 1;
            }
        }
    }
    let total: u64 = counts.iter().sum();
    println!("{}", total);
    Ok(())
}

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("input.txt")?;
    // let grid: Vec<Vec<String>> = contents
    //     .lines()
    //     .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
    //     .collect();
    // pt1(&grid)?;
    let grid2: Vec<Vec<char>> = contents
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    // pt2(&grid2.clone())?;
    pt22(&grid2);
    Ok(())
}
