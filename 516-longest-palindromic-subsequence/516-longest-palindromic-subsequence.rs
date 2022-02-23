use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        compare_subseq(&mut map, &s, 0, s.len() - 1)
    }
}

fn compare_subseq(
    map: &mut HashMap<(usize, usize), i32>,
    s: &Vec<char>,
    i: usize,
    j: usize,
) -> i32 {
    if i == j {
        return 1;
    } else if i > j {
        return 0;
    }

    let key = (i, j);
    if map.contains_key(&key) {
        return *map.get(&key).unwrap();
    }

    let ans = if s[i] == s[j] {
        2 + compare_subseq(map, s, i + 1, j - 1)
    } else {
        compare_subseq(map, s, i, j - 1).max(compare_subseq(map, s, i + 1, j))
    };
    map.insert(key, ans);
    ans
}
