use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();
        if n > m {
            return false;
        }

        let mut chars: HashMap<u8, i32> = HashMap::new();
        for c in s1.bytes() {
            chars.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        let s2 = s2.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while j < m {
            match chars.get(&s2[j]) {
                None => {
                    for k in i..=j {
                        chars.entry(s2[k]).and_modify(|x| *x += 1);
                    }
                    i = j + 1;
                    j = i;
                    // println!("");
                }
                Some(0) => {
                    chars.entry(s2[i]).and_modify(|x| *x += 1);
                    i += 1;
                    if i > j {
                        j = i;
                    }
                }
                _ => {
                    // print!("{}", s2[j] as char);
                    chars.entry(s2[j]).and_modify(|x| *x -= 1);
                    j += 1;
                }
            }
            if j - i == n {
                return true;
            }
        }

        false
    }
}
