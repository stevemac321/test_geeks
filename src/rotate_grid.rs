pub fn rotate_grid(grid: &mut [[i32; 4]; 4]) {
    let n = grid.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = grid[i][j];
            grid[i][j] = grid[n - j - 1][i];
            grid[n - j - 1][i] = grid[n - i - 1][n - j - 1];
            grid[n - i - 1][n - j - 1] = grid[j][n - i - 1];
            grid[j][n - i - 1] = temp;
        }
    }
}
