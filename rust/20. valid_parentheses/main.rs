use std::collections::VecDeque;


impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<char> = VecDeque::new();

        for ch in s.chars() {
            if ch == '{' || ch == '[' || ch == '(' {
                stack.push_back(ch);
            } else {
                if stack.len() == 0 {
                    return false;
                }
                let last: char = stack.pop_back().unwrap();
                let closure: bool = match ch {
                    '}' => last == '{',
                    ']' => last == '[',
                    ')' => last == '(',
                    _ => false,
                };
                if !closure {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}
