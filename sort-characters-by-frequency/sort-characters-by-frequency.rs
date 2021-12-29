use std::collections::BTreeMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counts = vec![0; 26*2+10];
        for c in s.bytes() {
            let idx = if c.is_ascii_lowercase() {
                (c - b'a') as usize
            } else if c.is_ascii_uppercase() {
                (c - b'A') as usize + 26
            } else {
                (c - b'0') as usize + 26*2
            };
            counts[idx] += 1;
        }

        let mut map = BTreeMap::new();
        for i in 0..26*2+10 {
            if counts[i] == 0 {
                continue;
            }

            let c = if i < 26 {
                (i as u8 + b'a') as char
            } else if i < 26*2 {
                (i as u8 + b'A' - 26) as char
            } else {
                (i as u8 + b'0' - 26*2) as char
            };
            map.entry(counts[i]).or_insert(Vec::new()).push(c)
        }

        map.into_iter()
            .rev()
            .map(|(count, v)| {
                v.into_iter()
                    .map(|c| vec![c; count].into_iter().collect::<String>())
                    .collect::<String>()
            })
            .collect()
    }
}
