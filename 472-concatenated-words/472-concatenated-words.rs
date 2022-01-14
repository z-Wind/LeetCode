impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|s| s.len());
        
        let mut ans = Vec::new();
        let mut trie = Trie::new();
        for word in words{
            if trie.starts_in(&word).unwrap_or(0) > 1{
                ans.push(word);
            } else {
                trie.insert(&word);    
            }
        }
        
        ans
    }
}

#[derive(Debug, Default)]
struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>;26],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: &str) {
        let mut curr = self;
        for i in word.bytes().map(|c| (c - b'a') as usize){
            curr = curr.children[i].get_or_insert(Box::new(Trie::new()));
        }
        curr.is_end = true;
    }
    
    fn starts_in(&self, word: &str) -> Option<i32> {
        if word.is_empty(){
            return Some(0);
        }
        // println!("{}", word);

        let mut ans = None;
        let mut curr = self;
        for (i, c) in word.bytes().map(|c| (c - b'a') as usize).enumerate(){
            match &curr.children[c]{
                None => break,
                Some(node) => curr = node,
            }
            if curr.is_end{
                match self.starts_in(&word[i+1..]){
                    Some(count) => return Some(1+count),
                    None => (),
                }
            }
        }

        // println!("{} => {:?}", word, ans);
        ans
    }
}