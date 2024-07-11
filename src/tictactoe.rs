pub fn check_winner(grid: [[i32; 3]; 3]) -> i32 {
    for i in 0..3 {
        if grid[i][0] != 0 && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
            return grid[i][0];
        }
        if grid[0][i] != 0 && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
            return grid[0][i];
        }
    }
    if grid[0][0] != 0 && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        return grid[0][0];
    }
    if grid[0][2] != 0 && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
        return grid[0][2];
    }
    for i in 0..3 {
        for j in 0..3 {
            if grid[i][j] == 0 {
                return 0;
            }
        }
    }
    2
}
