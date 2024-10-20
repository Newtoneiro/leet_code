impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![1; nums.len()];

        let mut left: i32 = 1;
        for i in 0..nums.len() {
            output[i] *= left;
            left *= nums[i];
        }

        let mut right: i32 = 1;
        for i in (0..nums.len()).rev() {
            output[i] *= right;
            right *= nums[i];
        }

        output
    }
}