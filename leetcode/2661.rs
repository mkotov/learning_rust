// https://leetcode.com/problems/first-completely-painted-row-or-column/

use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut d: HashMap<i32, (usize, usize)> = HashMap::new();
        let m = mat.len();
        let n = mat[0].len();

        for i in 0..m {
            for j in 0..n {
                d.insert(mat[i][j], (i, j));
            }
        }

        let mut rs: Vec<usize> = vec![0; m];
        let mut cs: Vec<usize> = vec![0; n];
        
        for i in 0..m * n {
            let (x, y) = d[&arr[i]];
        
            rs[x] += 1;
            if rs[x] == n {
                return i as i32
            }

            cs[y] += 1;
            if cs[y] == m {
                return i as i32
            }
        }
        unreachable!();
    }
}
