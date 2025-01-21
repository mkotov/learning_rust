// https://leetcode.com/problems/grid-game/

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();

        let mut A: i64 = 0;
        let mut B: i64 = 0;

        for i in 0..n {
            B += grid[0][i] as i64;
        }

        let mut m: i64 = i64::MAX;

        for i in 0..n {
            B -= grid[0][i] as i64;
            m = min(m, max(A, B));
            A += grid[1][i] as i64;
        }

        m
    }
}
