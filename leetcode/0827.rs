https://leetcode.com/problems/making-a-large-island/description/

use std::collections::HashSet;
use std::cmp::max;

fn dfs(grid: &mut Vec<Vec<i32>>, sizes: &mut Vec<i32>, i: usize, j: usize) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    if grid[i][j] <= 0 {
        return 0
    }

    grid[i][j] = -(sizes.len() as i32) - 1;

    let mut result = 1;
            
    if i > 0 {
        result += dfs(grid, sizes, i - 1, j);
    }
    if j > 0 {
        result += dfs(grid, sizes, i, j - 1);
    }
    if i < m - 1 {
        result += dfs(grid, sizes, i + 1, j);
    }
    if j < n - 1 {
        result += dfs(grid, sizes, i, j + 1);
    }      
  
    result
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut sizes: Vec<i32> = Vec::new();

        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let r = dfs(&mut grid, &mut sizes, i, j);
                    if r == (m * n) as i32 {
                        return r
                    }
                    sizes.push(r);
                }
            }
        }

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut s: HashSet<i32> = HashSet::new();

                    if i > 0 && grid[i - 1][j] < 0 {
                        s.insert(grid[i - 1][j]);
                    }
                    if j > 0 && grid[i][j - 1] < 0 {
                        s.insert(grid[i][j - 1]);
                    }
                    if i < m - 1 && grid[i + 1][j] < 0 {
                        s.insert(grid[i + 1][j]);
                    }
                    if j < n - 1 && grid[i][j + 1] < 0 {
                        s.insert(grid[i][j + 1]);
                    }

                    let mut v = 1;
                    for k in s {
                        v += sizes[(-k - 1) as usize];
                    }

                    result = max(result, v);
                }
            }
        }

        result
    }
}
