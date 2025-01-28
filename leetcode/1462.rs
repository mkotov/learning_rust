// https://leetcode.com/problems/course-schedule-iv/description/

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;

        let mut P = vec![vec![false; n]; n];

        for p in prerequisites {
            P[p[0] as usize][p[1] as usize] = true
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    P[i][j] |= P[i][k] && P[k][j];
                }
            }
        }

        let mut result = Vec::new();

        for q in queries {
            result.push(P[q[0] as usize][q[1] as usize])
        }

        result
   }
}
