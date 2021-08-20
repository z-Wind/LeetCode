use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map_t: HashMap<char, i32> = HashMap::new();
        t.chars()
            .enumerate()
            .for_each(|(i, c)| *map_t.entry(c).or_insert(0) += 1);

        let filtered_s: Vec<(usize, char)> = s
            .match_indices(|c| map_t.contains_key(&c))
            .map(|(i, s)| (i, s.chars().next().unwrap()))
            .collect();
        //println!("{:?}, {:?}", map_t, filtered_s);

        let (mut l, mut r) = (0, 0);
        let mut ans = (usize::MAX, None, None);
        let mut collected = 0;
        let done = map_t.len();
        let mut window_count: HashMap<char, i32> = HashMap::new();
        while r < filtered_s.len() {
            let c = filtered_s[r].1;
            *window_count.entry(c).or_insert(0) += 1;
            match (map_t.get(&c), window_count.get(&c)) {
                (Some(num_t), Some(num_w)) if num_t == num_w => collected += 1,
                _ => (),
            }
            while l <= r && collected == done {
                let c = filtered_s[l].1;

                let end = filtered_s[r].0;
                let start = filtered_s[l].0;
                let range = end - start + 1;
                if range < ans.0 {
                    ans = (range, Some(start), Some(end));
                }
                *window_count.entry(c).or_insert(0) -= 1;
                match (map_t.get(&c), window_count.get(&c)) {
                    (Some(num_t), Some(num_w)) if num_t > num_w => collected -= 1,
                    _ => (),
                }
                //println!("{},{} => {:?}",l,r,ans);
                l += 1;
            }
            r += 1;
        }

        match ans{
            (_,None,None) => String::from(""),
            (_,Some(start),Some(end)) => s[start..=end].to_string(),
            _ => panic!(),
        }
    }
}
