use std::collections::BinaryHeap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counts = vec![0; 26 * 2 + 10];
        for c in s.bytes() {
            let idx = if c.is_ascii_lowercase() {
                (c - b'a') as usize
            } else if c.is_ascii_uppercase() {
                (c - b'A') as usize + 26
            } else {
                (c - b'0') as usize + 26 * 2
            };
            counts[idx] += 1;
        }

        let mut heap = BinaryHeap::new();
        for i in 0..26 * 2 + 10 {
            if counts[i] == 0 {
                continue;
            }

            let c = if i < 26 {
                (i as u8 + b'a') as char
            } else if i < 26 * 2 {
                (i as u8 + b'A' - 26) as char
            } else {
                (i as u8 + b'0' - 26 * 2) as char
            };
            heap.push((counts[i], c.to_string()));
        }
        // println!("{:?}", heap);
        
        let mut ans = String::with_capacity(s.len());
        while let Some((count, c)) = heap.pop(){
            ans.push_str(&c.repeat(count)); 
        }  
        ans
    }
}
