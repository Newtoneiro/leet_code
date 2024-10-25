use std::collections::VecDeque;


impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();

        for elem in tokens {
            match elem.parse::<i32>() {
                Ok(elem_int) => stack.push_back(elem_int),
                Err(e) => {
                    let elem1 = stack.pop_back().unwrap();
                    let elem2 = stack.pop_back().unwrap();

                    let out = match elem.as_str() {
                        "+" => {
                            elem2 + elem1
                        },
                        "-" => {
                            elem2 - elem1
                        },
                        "*" => {
                            elem2 * elem1
                        },
                        "/" => {
                            elem2 / elem1
                        },
                        _ => 0
                    };

                    stack.push_back(out);
                },
            };
        }

        stack.pop_back().unwrap()
    }
}
