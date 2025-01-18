// https://leetcode.com/problems/neighboring-bitwise-xor/description/
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut s = 0;
        for n in derived {
            s ^= n
        }

        s == 0
    }
}
