// https://leetcode.com/problems/word-ladder/discuss/40707/C%2B%2B-BFS
use std::collections::VecDeque;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        let begin_word = begin_word.as_bytes().to_vec();
        let end_word = end_word.as_bytes().to_vec();
        let mut dict:HashSet<Vec<u8>> = HashSet::from_iter(word_list.iter().map(|x| x.as_bytes().to_vec()));
        let mut todo:VecDeque<Vec<u8>> = VecDeque::new();
        todo.push_back(begin_word);
        let mut ladder = 1;
        while !todo.is_empty() {
            let n = todo.len();
            for i in (0..n){
                let mut word = todo.pop_front().unwrap();
                if (word == end_word) {
                    return ladder;
                }
                dict.remove(&word);
                for j in (0..word.len()) {
                    let c = word[j];
                    for k in (0..26 as u8){
                        word[j] = b'a' + k;
                        if dict.contains(&word) {
                            todo.push_back(word.clone());
                        }
                     }
                    word[j] = c;
                }
            }
            ladder+=1;
        }
        return 0;
    }
}