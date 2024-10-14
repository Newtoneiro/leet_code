impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut grid: Vec<Vec<char>> = Vec::new();
        for i in 0..num_rows {
            grid.push(Vec::new());
        }

        let mut cur_row: i32 = 0;
        let mut cur_increment: i32 = 1;
        for ch in s.chars() {
            grid[cur_row as usize].push(ch);
            cur_row += cur_increment;

            if cur_row == num_rows {
                cur_row = num_rows - 2;
                cur_increment = -1;
            } else if cur_row == -1 {
                cur_row = 1;
                cur_increment = 1;
            }
        }
        
        let mut output = "".to_string();
        for v in grid {
            output.extend(v.into_iter());
        }

        output
    }
}