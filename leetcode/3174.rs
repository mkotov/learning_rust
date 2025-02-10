https://leetcode.com/problems/clear-digits/

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = String::new();
    
        for c in s.chars() {
            if c.is_alphabetic() {
                result.push(c);
            } else {
                result.pop();
            }
        }
    
        result
    }
}
