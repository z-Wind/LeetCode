// https://leetcode.com/problems/palindrome-pairs/discuss/79195/O(n-*-k2)-java-solution-with-Trie-structure

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut trie = Trie::new();
        for i in 0..words.len() {
            trie.insert_rev(&words[i], i as i32);
        }
        // println!("{:?}", trie);

        for i in 0..words.len() {
            trie.palindrome_pairs(&words[i], i as i32, &mut ans);
            // println!("{}:{} => {:?}", i, words[i], ans);
        }

        ans
    }
}

#[derive(Debug, Default)]
struct Trie {
    index: i32,
    list: Vec<i32>,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            index: -1,
            ..Default::default()
        }
    }

    /** Inserts a reverse word into the trie. */
    fn insert_rev(&mut self, word: &str, index: i32) {
        let mut curr = self;
        for (i, c) in word.bytes().map(|c| (c - b'a') as usize).enumerate().rev() {
            if is_palindrome(&word[0..=i].as_bytes()) {
                curr.list.push(index);
            }
            
            curr = curr.children[c].get_or_insert(Box::new(Trie::new()));
        }
        
        curr.list.push(index);
        curr.index = index;
    }

    fn palindrome_pairs(&self, prefix: &str, index: i32, ans: &mut Vec<Vec<i32>>) {
        let mut curr = self;
        for (i, c) in prefix.bytes().map(|c| (c - b'a') as usize).enumerate() {
            // febe, f or "", aa
            if curr.index != -1
                && curr.index != index
                && is_palindrome(&prefix[i..].as_bytes())
            {
                ans.push(vec![index, curr.index]);
            }
            match &curr.children[c] {
                None => return,
                Some(node) => {
                    curr = node;
                }
            }
        }

        for &i in curr.list.iter() {
            if i != index {
                ans.push(vec![index, i]);
            }
        }
    }
}

fn is_palindrome(s: &[u8]) -> bool {
    if s.len() == 0 {
        return true;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}
