// https://leetcode.com/problems/check-balanced-string/description/

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut i = 0;
        let mut v = vec![0, 0];

        for c in num.chars() {
            v[i] += (c as i32) - ('0' as i32);
            i = 1 - i;
        }

        v[0] == v[1]
    }
}
