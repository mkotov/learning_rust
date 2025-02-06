https://leetcode.com/problems/tuple-with-same-product/

use std::collections::HashMap;
.entry(u).or_default().
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut d : HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                *d.entry(nums[i] * nums[j]).or_default() += 1;
            }
        }
                
        
        let mut result = 0;

        for (k, v) in d {
            result += v * (v - 1);
        }
                
        4 * result
    }
}
