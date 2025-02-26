https://leetcode.com/problems/find-special-substring-of-length-k/description/

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut n: i32 = 0;
        let mut c: Option<u8> = None;

        for a in s.bytes() {
            if c.is_some() && c.unwrap() == a {
                n += 1;
            } else {
                if n == k {
                    return true;
                }
                n = 1;
                c = Some(a);
            }
        }

        n == k   
    }
}
