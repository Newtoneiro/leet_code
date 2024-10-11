impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut buffer: String = "".to_string();
        let mut best_len: i32 = 0;
        let mut cur_len: i32 = 0;

        for ch in s.chars() {
            if buffer.contains(ch) {
                let idx = buffer.find(ch).unwrap();
                buffer = buffer.chars().skip(idx + 1).collect();
                cur_len -= (idx as i32)
            } else {
                cur_len += 1;
                if cur_len > best_len {
                    best_len = cur_len;
                }
            }
            buffer.push(ch);
        }

        best_len
    }
}