// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/discuss/87739/Java-Strict-O(N)-Two-Pointer-Solution/92538

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let maxUnique = get_max_unique_letters(s);
        let mut ans = 0;

        for numUniqueTarget in 1..=maxUnique {
            ans = ans.max(longest_substring_with_N_unique_chars(s, k, numUniqueTarget));
        }

        ans
    }
}

fn longest_substring_with_N_unique_chars(s: &[u8], k: i32, numUniqueTarget: i32) -> i32 {
    let n = s.len();
    let mut counts = vec![0; 26];
    let mut numUnique = 0; // counter 1
    let mut numNoLessThanK = 0; // counter 2
    let mut begin = 0;
    let mut end = 0;
    let mut len = 0;

    while end < n {
        let idx = (s[end] - b'a') as usize;
        if counts[idx] == 0 {
            numUnique += 1; // increment counts[c] after this statement
        }
        counts[idx] += 1;
        if counts[idx] == k {
            numNoLessThanK += 1; // inc end after this statement
        }
        end += 1;

        while numUnique > numUniqueTarget {
            let idx = (s[begin] - b'a') as usize;
            if counts[idx] == k {
                numNoLessThanK -= 1; // decrement counts[c] after this statement
            }
            counts[idx] -= 1;
            if counts[idx] == 0 {
                numUnique -= 1; // inc begin after this statement
            }
            begin += 1;
        }

        // if we found a string where the number of unique chars equals our target
        // and all those chars are repeated at least K times then update max
        if numUnique == numUniqueTarget && numUnique == numNoLessThanK {
            len = len.max(end - begin);
        }
    }

    len as i32
}

// get the maximum number of unique letters in the string s
fn get_max_unique_letters(s: &[u8]) -> i32 {
    let mut exist = vec![false; 26];
    let mut maxUnique = 0;
    for c in s {
        let idx = (c - b'a') as usize;
        if !exist[idx] {
            maxUnique += 1;
            exist[idx] = true;
        }
    }
    maxUnique
}
