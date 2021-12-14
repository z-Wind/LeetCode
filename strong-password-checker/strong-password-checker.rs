// https://leetcode.com/problems/strong-password-checker/discuss/91007/C%2B%2B-0ms-O(n)-35-lines-solution-with-detailed-explanation
// the problem into three cases:
// (1) s.length() < 6
// (2) 6 <= s.length() <= 20
// (3) s.length() > 20
// https://leetcode.com/problems/strong-password-checker/discuss/91007/C++-0ms-O(n)-35-lines-solution-with-detailed-explanation/329852

use std::collections::HashMap;
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let s = password.as_bytes();
        let deleteTarget: i32 = 0.max(s.len() as i32 - 20);
        let addTarget: i32 = 0.max(6 - s.len() as i32);
        let mut toDelete: i32 = 0;
        let mut toAdd: i32 = 0;
        let mut toReplace: i32 = 0;
        let mut needUpper: i32 = 1;
        let mut needLower: i32 = 1;
        let mut needDigit: i32 = 1;

        ///////////////////////////////////
        // For cases of s.length() <= 20 //
        ///////////////////////////////////
        let mut l = 0;
        for r in 0..s.len() {
            if s[r].is_ascii_uppercase() {
                needUpper = 0;
            } else if s[r].is_ascii_lowercase() {
                needLower = 0;
            } else if s[r].is_ascii_digit() {
                needDigit = 0;
            }

            if r - l == 2 {
                // if it's a three-letter window
                if s[l] == s[l + 1] && s[l + 1] == s[r] {
                    // found a three-repeating substr
                    // insert letter to break repetition if possible
                    if toAdd < addTarget {
                        toAdd += 1;
                        l = r;
                    }
                    // replace current word to avoid three repeating chars
                    else {
                        toReplace += 1;
                        l = r + 1;
                    }
                // keep the window with no more than 3 letters
                } else {
                    l += 1;
                }
            }
        }
        if s.len() <= 20 {
            return (addTarget + toReplace).max(needUpper + needLower + needDigit);
        }

        //////////////////////////////////
        // For cases of s.length() > 20 //
        //////////////////////////////////
        toReplace = 0; // reset toReplace
        let mut lenCnts: Vec<HashMap<i32, i32>> = vec![HashMap::new(); 3]; // to record repetitions with (length % 3) == 0, 1 or 2
        l = 0;
        for r in 0..=s.len() {
            // record all repetion frequencies
            if r == s.len() || s[l] != s[r] {
                let len = (r - l) as i32;
                // we only care about repetions with length >= 3
                if len > 2 {
                    *lenCnts[len as usize % 3].entry(len).or_insert(0) += 1;
                }
                l = r;
            }
        }

        /*
            Use deletions to minimize replacements, following below orders:
            (1) Try to delete one letter from repetitions with (length % 3) == 0. Each deletion decreases replacement by 1
            (2) Try to delete two letters from repetitions with (length % 3) == 1. Each deletion decreases repalcement by 1
            (3) Try to delete multiple of three letters from repetions with (length % 3) == 2. Each deletion (of three
            letters) decreases repalcements by 1
        */
        for i in 0..3 {
            let mut temp = Vec::with_capacity(lenCnts[i].len());
            for (len, count) in lenCnts[i].iter_mut() {
                if i < 2 {
                    let numLetters: i32 = i as i32 + 1;
                    let dec = (*count).min((deleteTarget - toDelete) / numLetters);
                    // only care should be deleted
                    if dec > 0 {
                        toDelete += dec * numLetters; // dec is the number of repetitions we'll delete from
                        *count -= dec; // update number of repetitions left

                        // after letters deleted, it fits in the group where (length % 3) == 2
                        if *len - numLetters > 2 {
                            temp.push((*len - numLetters, dec));
                        }
                    }
                }

                // record number of replacements needed
                // note if len is the length of repetition, we need (len / 3) number of replacements
                toReplace += (*count * (*len / 3));
            }
            // after letters deleted, it fits in the group where (length % 3) == 2
            for (len, dec) in temp {
                *lenCnts[2].entry(len).or_insert(0) += dec;
            }
        }
        // toReplace change to toDelete with deleting three same letters
        let dec = (deleteTarget - toDelete) / 3; // try to delete multiple of three letters as many as possible
        toReplace -= dec; // maybe negative because no repetions
        toDelete += dec * 3;
        // println!("toReplace:{},toDelete:{},deleteTarget:{}",toReplace, toDelete, deleteTarget);
        return deleteTarget + toReplace.max(needUpper + needLower + needDigit);
    }
}
