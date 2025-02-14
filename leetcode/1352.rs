// https://leetcode.com/problems/product-of-the-last-k-numbers/description/

struct ProductOfNumbers {
    v: Vec<i32>
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { v: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.v = vec![1];
        } else {
            self.v.push(self.v[self.v.len() - 1] * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k  > self.v.len() - 1 {
            0
        } else {
            self.v[self.v.len() - 1] / self.v[self.v.len() - 1 - k]
        }
    }
}
