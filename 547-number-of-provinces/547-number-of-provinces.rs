use std::collections::VecDeque;

impl Solution {
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut deq = VecDeque::new();

        let mut ans = 0;
        for i in 0..n {
            // println!("start: {}", i);
            if is_connected[i][i] == 0 {
                continue;
            }

            is_connected[i][i] = 0;
            deq.push_back(i);
            while let Some(city) = deq.pop_front() {
                // println!("connect: {}", city);
                for j in i + 1..n {
                    if is_connected[city][j] == 0 {
                        continue;
                    }

                    is_connected[city][j] = 0;
                    is_connected[j][city] = 0;
                    is_connected[j][j] = 0;
                    deq.push_back(j);
                }
            }

            ans += 1;
        }

        ans
    }
}
