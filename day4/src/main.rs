use anyhow::Result;

fn safe_get_pos(grid: &Vec<Vec<String>>, pos: (isize, isize)) -> Option<String> {
    let grid_length_assuming_square: isize = grid.len().try_into().unwrap();
    if pos.0 < 0 || pos.0 > grid_length_assuming_square - 1 {
        return None;
    }
    if pos.1 < 0 || pos.1 > grid_length_assuming_square - 1 {
        return None;
    }
    let x: usize = pos.0.try_into().unwrap();
    let y: usize = pos.1.try_into().unwrap();
    Some(grid[y][x].clone())
}

fn num_rolls_adjacent(grid: &Vec<Vec<String>>, pos: (usize, usize)) -> usize {
    let mut rolls = 0;
    let x: isize = pos.0 as isize;
    let y: isize = pos.1 as isize;
    for i in -1isize..=1 {
        for j in -1isize..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let maybe_c = safe_get_pos(&grid, (x + i, y + j));
            if let Some(c) = maybe_c {
                if c == "@".to_string() {
                    rolls += 1
                }
            }
        }
    }

    rolls
}

fn can_be_removed(grid: &Vec<Vec<String>>, pos: (usize, usize)) -> bool {
    let adjacent_rolls = num_rolls_adjacent(grid, pos);
    if adjacent_rolls < 4 {
        return true;
    }
    false
}

#[allow(dead_code)]
fn pt1(grid: &Vec<Vec<String>>) -> Result<()> {
    let grid_length = grid.iter().len();

    let mut num = 0;
    for y in 0..grid_length {
        for x in 0..grid_length {
            if grid[y][x] == "@".to_string() {
                let n = num_rolls_adjacent(grid, (x, y));
                if n < 4 {
                    num += 1
                }
            }
        }
    }

    println!("{num}");

    Ok(())
}

fn pt2(grid: &Vec<Vec<String>>) -> Result<()> {
    let mut grid = grid.clone();
    let grid_length = &grid.iter().len();
    let mut removed = 0;

    loop {
        let last_removed = removed;
        let mut removable_coords: Vec<(usize, usize)> = vec![];

        for y in 0..*grid_length {
            for x in 0..*grid_length {
                let c = &grid[y][x];
                if !(*c == "@".to_string()) {
                    continue;
                }
                let coord = (x, y);
                if can_be_removed(&grid, coord) {
                    removable_coords.push(coord);
                }
            }
        }

        for coord in removable_coords {
            grid[coord.1][coord.0] = ".".to_string();
            removed += 1;
        }

        if last_removed == removed {
            break;
        }
    }

    println!("{removed}");

    Ok(())
}

fn main() -> Result<()> {
    let grid = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    // pt1(&grid)?;
    pt2(&grid)?;
    Ok(())
}
