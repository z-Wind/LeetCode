use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_loc: Vec<usize> = vec![usize::MAX; t.len()];
        let mut t_loc_map: HashMap<char,Vec<usize>> = HashMap::new();
        t.chars().enumerate()
            .for_each(|(i,c)| t_loc_map.entry(c).or_insert(vec![]).push(i));
        
        let mut min_left: usize = 0;
        let mut min_right: usize = usize::MAX;
        for (i, c) in s.chars().enumerate() {
            if let Some(v) =  t_loc_map.get(&c) {
                let mut min_i = v[0];
                for x in v.iter() {
                    if t_loc[min_i] > t_loc[*x] {
                        min_i = *x;
                    }
                }
                if min_right == usize::MAX{
                    for x in v.iter() {
                        if t_loc[*x] == usize::MAX {
                            min_i = *x;
                            break;
                        }
                    }
                }
                t_loc[min_i] = i;
                let left = *t_loc.iter().min().unwrap();
                let right = *t_loc.iter().max().unwrap();
                //println!("({},{}) ({},{}) {:?}", left, right, min_left, min_right, t_loc);
                if right != usize::MAX
                    && right - left < min_right - min_left
                    && right - left + 1 >= t.len()
                {
                    min_left = left;
                    min_right = right;
                }
            }
            //println!("{:?} => {},{}", t_loc, min_left, min_right);
        }
        if min_right == usize::MAX {
            return String::from("");
        }
        s[min_left..=min_right].to_string()
    }
}
