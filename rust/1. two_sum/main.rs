use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut dict: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            match dict.get(&num) {
                Some(i2) => return Vec::from([*i2, i as i32]),
                None => {
                    dict.insert(target - num, i as i32);
                },
            };
        }

        return Vec::new();
    }
}