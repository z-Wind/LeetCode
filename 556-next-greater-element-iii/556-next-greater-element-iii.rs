use std::collections::BinaryHeap;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<char> = n.to_string().chars().collect();
        let mut heap = BinaryHeap::new();
        let n = digits.len();

        for i in (0..n).rev() {
            heap.push(digits[i]);
            
            if *heap.peek().unwrap() > digits[i] {
                let mut end = n - 1;
                loop {
                    let tmp = heap.pop().unwrap();
                    if *heap.peek().unwrap() > digits[i] {
                        digits[end] = tmp;
                        end -= 1;
                    } else {
                        digits[i] = tmp;
                        break;
                    }
                }
                
                for j in (i + 1..=end).rev() {
                    digits[j] = heap.pop().unwrap();
                }

                return digits
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or(-1);
            }
        }

        return -1;
    }
}