use rand::{self, Rng};
use std::{thread, time::Duration};

/// Returns a 2d vector of size w x h filled with 0s
fn empty_grid(w: usize, h: usize) -> Vec<Vec<i8>> {
    return vec![vec![0i8; w]; h];
}

/// Randomizes the grid with either 0s or 1s
fn randomize_grid(grid: &mut Vec<Vec<i8>>) {
    for row in grid.iter_mut() {
        for elt in row.iter_mut() {
            *elt = rand::thread_rng().gen_range(0..2) as i8;
        }
    }
}

/// Replaces the cell's value with 1
fn birth(grid: &mut Vec<Vec<i8>>, x: usize, y: usize) {
    grid[x][y] = 1;
}

/// Replaces the cell's value with 0
fn kill(grid: &mut Vec<Vec<i8>>, x: usize, y: usize) {
    grid[x][y] = 0;
}

/// Returns the number of alive neighbors
fn neighbors(grid: &Vec<Vec<i8>>, x: usize, y: usize) -> i8 {
    let mut count = 0;
    let dx: [i32; 3] = [-1, 0, 1];
    let dy: [i32; 3] = [-1, 0, 1];

    for &i in dx.iter() {
        for &j in dy.iter() {
            if i != 0 || j != 0 {
                let nx = ((x as i32 + i + grid.len() as i32) % grid.len() as i32) as usize;
                let ny = ((y as i32 + j + grid[0].len() as i32) % grid[0].len() as i32) as usize;
                count += grid[nx][ny];
            }
        }
    }
    count
}

/// Updates the grid to its next state
fn update_grid(grid: &mut Vec<Vec<i8>>) {
    let mut new_grid = grid.clone();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let alive_neighbors = neighbors(&grid, x, y);
            if alive_neighbors == 2 {
                continue;
            } else if alive_neighbors == 3 {
                birth(&mut new_grid, x, y);
            } else {
                kill(&mut new_grid, x, y);
            }
        }
    }
    *grid = new_grid;
}

/// Displays the grid with square blocks
fn display_grid(grid: &mut Vec<Vec<i8>>) {
    for row in grid.iter() {
        println!();
        for elt in row.iter() {
            if *elt == 0 {
                print!("□ ");
            } else {
                print!("■ ")
            }
        }
    }
    println!();
}

/// Main function: plays a random 20x10 grid 100 times
fn main() {
    let mut grid = empty_grid(20, 10);
    randomize_grid(&mut grid);
    for _ in 0..100 {
        display_grid(&mut grid);
        update_grid(&mut grid);
        thread::sleep(Duration::from_millis(1000));
    }
}
