// https://leetcode.com/problems/trapping-rain-water-ii/description/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut pq = BinaryHeap::new();

        for i in 0..m {
            pq.push((Reverse(height_map[i][0]), i, 0));
            visited[i][0] = true;
            pq.push((Reverse(height_map[i][n - 1]), i, n - 1));
            visited[i][n - 1] = true;
        }
        for j in 0..n {
            pq.push((Reverse(height_map[0][j]), 0, j));
            visited[0][j] = true;
            pq.push((Reverse(height_map[m - 1][j]), m - 1, j));
            visited[m - 1][j] = true;
        }

        let mut result = 0;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((Reverse(h), i, j)) = pq.pop() {
            for &(dr, dc) in &directions {
                let nr = i as isize + dr;
                let nc = j as isize + dc;
                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let nc = nc as usize;
                    let nr = nr as usize;
                    if !visited[nr][nc] {
                        result += (h - height_map[nr][nc]).max(0);
                        pq.push((Reverse(h.max(height_map[nr][nc])), nr, nc));
                        visited[nr][nc] = true;
                    }
                }
            }
        }

        result
    }
}
