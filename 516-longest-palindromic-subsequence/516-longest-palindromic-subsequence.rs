impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut map = vec![vec![0; n]; n];
        compare_subseq(&mut map, &s, 0, n - 1)
    }
}

fn compare_subseq(map: &mut Vec<Vec<i32>>, s: &Vec<char>, i: usize, j: usize) -> i32 {
    if i == j {
        return 1;
    } else if i > j {
        return 0;
    }

    if map[i][j] != 0 {
        return map[i][j];
    }

    map[i][j] = if s[i] == s[j] {
        2 + compare_subseq(map, s, i + 1, j - 1)
    } else {
        compare_subseq(map, s, i, j - 1).max(compare_subseq(map, s, i + 1, j))
    };
    
    map[i][j]
}

