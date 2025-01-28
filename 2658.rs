// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/description/

use core::cmp::max;

fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut result = 0;

    if grid[i][j] > 0 {
        result += grid[i][j];         
        grid[i][j] = 0;
        if i > 0 {
            result += dfs(grid, i - 1, j)
        }
        if i < grid.len() - 1 {
            result += dfs(grid,i + 1, j)
        }
        if j > 0 {
            result +=  dfs(grid, i, j - 1)
        }
        if j < grid[0].len() - 1 {
            result += dfs(grid, i, j + 1)
        }
    }

    result
}

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
 
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                result = max(result, dfs(&mut grid, i, j))
            }
        }

        result    
    }
}
