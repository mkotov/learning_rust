// https://leetcode.com/problems/count-servers-that-communicate/

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut A = vec![0; m];
        let mut B = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                A[i] += grid[i][j];
                B[j] += grid[i][j];
            }
        }

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && (A[i] > 1 || B[j] > 1) {
                    result += 1
                }
            }
        }

        result   
    }
}
