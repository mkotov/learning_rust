// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/description/

use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls: HashMap<i32, i32> = HashMap::new();
        let mut colors: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        for query in queries {
            let ball = query[0];
            let color = query[1];
            if let Some(&prev_color) = balls.get(&ball) {
                if let Some(count) = colors.get_mut(&prev_color) {
                    *count -= 1;
                    if *count == 0 {
                        colors.remove(&prev_color);
                    }
                }
            }
            balls.insert(query[0], query[1]);
            *colors.entry(query[1]).or_default() += 1;
            result.push(colors.len() as i32);
        }

        result
    }
}
