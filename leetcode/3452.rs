// https://leetcode.com/problems/sum-of-good-numbers/description/

impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;

        for i in 0..nums.len() {
            if i >= k && nums[i] <= nums[i - k] {
                continue;
            }
            if i + k < nums.len() && nums[i] <= nums[i + k] {
                continue;
            }

            result += nums[i];
        }

        result
    }
}
