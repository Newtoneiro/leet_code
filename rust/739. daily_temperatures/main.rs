use std::collections::VecDeque;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: VecDeque<(usize, i32)> = VecDeque::new(); // index, temperature

        for (i, temperature) in temperatures.into_iter().enumerate() {
            while let Some(&(prev_i, prev_temp)) = stack.back() {
                if temperature > prev_temp {
                    stack.pop_back();
                    out[prev_i] = (i - prev_i) as i32;
                } else {
                    break;
                }
            }
            stack.push_back((i, temperature));
        }

        out
    }
}