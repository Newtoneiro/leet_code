use std::collections::HashMap;


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            *hm.entry(num).or_insert(0) += 1;
        }
        let mut hm_list: Vec<(i32, i32)> = hm.into_iter()
            .collect();
        hm_list.sort_by(|a, b| b.1.cmp(&a.1));
        
        let actual_output: Vec<i32> = hm_list.into_iter().take(k as usize).map(|(val, count)| val).collect();
        actual_output
    }
}
