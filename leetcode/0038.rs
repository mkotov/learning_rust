// https://leetcode.com/problems/count-and-say/

impl Solution {
    pub fn rle(n: &str) -> String {
        let mut result = String::new();

        let mut c = n.chars().nth(0).unwrap();
        let mut m = 1; 
        for d in n[1..].chars() { 
            if d == c {
                m += 1;
            } else {
                result.push_str(&m.to_string());
                result.push(c);
                c = d;
                m = 1;
            }
        }

        result.push_str(&m.to_string());
        result.push(c);

        result 
    }
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string()
        }

        Self::rle(&Self::count_and_say(n - 1))
    }
}
