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
