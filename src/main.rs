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
