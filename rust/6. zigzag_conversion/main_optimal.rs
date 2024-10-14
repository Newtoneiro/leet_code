impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut output = Vec::new();
        
        let seq_len: i32 = num_rows + (num_rows - 2);  // horizontal line + zigzag
        for (i, ch) in s.chars().enumerate() {
            let new_i = num_rows - (num_rows - 1 - (i as i32 % seq_len)).abs() - 1;
            output.push((new_i, ch));
        }

        output.sort_by(|a, b| a.0.cmp(&b.0));
        output.iter()
            .map(|i| i.1.to_string())
            .reduce(|cur, next| cur + &next)
            .unwrap()
    }
}