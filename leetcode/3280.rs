https://leetcode.com/problems/convert-date-to-binary/

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let year = &date[0..4].parse::<i32>().unwrap();
        let month = &date[5..7].parse::<i32>().unwrap();
        let day = &date[8..10].parse::<i32>().unwrap();
        
        format!("{year:b}-{month:b}-{day:b}")       
    }
}
