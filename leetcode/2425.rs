// https://leetcode.com/problems/bitwise-xor-of-all-pairings/
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut a = 0;
        for n in &nums1 {
            a ^= n
        }

        let mut b = 0;
        for n in &nums2 {
            b ^= n
        }

        let mut r = 0;
        if nums1.len() % 2 == 1 {
            r ^= b
        }
        if nums2.len() % 2 == 1 {
            r ^= a
        }
       
        r
    }
}
