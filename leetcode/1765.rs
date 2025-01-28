// https://leetcode.com/problems/map-of-highest-peak/description/

use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut result : Vec<Vec<i32>> = vec![vec![-1; n]; m];

        let mut to_visit : VecDeque<(usize, usize, i32)> = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    to_visit.push_back((i, j, 0));
                }
            }
        }

        while let Some((i, j, v)) = to_visit.pop_front() {
            if i >= 0 && i < m && j >= 0 && j < n && result[i][j] == -1 {
                result[i][j] = v;

                to_visit.push_back((i - 1, j, v + 1));
                to_visit.push_back((i + 1, j, v + 1));
                to_visit.push_back((i, j - 1, v + 1));
                to_visit.push_back((i, j + 1, v + 1));
            }
        }

        result
    }
}
