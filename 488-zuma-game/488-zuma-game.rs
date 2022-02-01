// https://leetcode.com/problems/zuma-game/discuss/1226417/Two-well-explained-BFS-and-DFS-C%2B%2B-solutions-that-pass-all-35-test-cases-(17th-Nov-21)

use std::collections::HashMap;

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board = board.as_bytes();
        let mut freq = [0; 26];
        for c in hand.bytes() {
            let idx = (c - b'A') as usize;
            freq[idx] += 1;
        }

        let mut cache = HashMap::new();
        let ans = find_min_step(board, &mut freq, &mut cache);
        if ans == i32::MAX {
            return -1;
        }
        ans
    }
}

fn find_min_step(
    mut board: &[u8],
    freq: &mut [i32; 26],
    cache: &mut HashMap<(Vec<u8>, [i32; 26]), i32>,
) -> i32 {
    // println!("{}", String::from_utf8(board.to_vec()).unwrap());
    let board = clear(board);
    if board.is_empty() {
        return 0;
    }
    // println!("=> {}:{:?}", String::from_utf8(board.clone()).unwrap(), freq);

    let key = (board.clone(), freq.clone());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let mut ans = i32::MAX;
    for i in 0..board.len() {
        for c in [b'R', b'Y', b'B', b'G', b'W'] {
            let idx = (c - b'A') as usize;
            if freq[idx] == 0 {
                continue;
            }

            // "RRWWRRBBRR"
            // "WB"
            if !((board[i] == c) || (board[i] != c && i > 0 && board[i] == board[i - 1])) {
                continue;
            }

            freq[idx] -= 1;
            ans = ans.min(find_min_step(
                &[&board[..i], &[c], &board[i..]].concat(),
                freq,
                cache,
            ));
            freq[idx] += 1;
        }
    }
    if ans != i32::MAX {
        ans = ans + 1;
    }
    cache.insert(key, ans);
    ans
}

fn clear(board: &[u8]) -> Vec<u8> {
    let mut s = board.to_vec();

    let mut prev = 0;
    for i in 1..=s.len() {
        if i < s.len() && s[i] == s[prev] {
            continue;
        }

        if i - prev >= 3 {
            let s2 = [&board[..prev], &board[i..]].concat();
            return clear(&s2);
        };
        prev = i;
    }

    board.to_vec()
}
