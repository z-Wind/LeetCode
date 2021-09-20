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
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|c| ((c - b'a') as usize) as usize){
            curr = curr.children[i].get_or_insert(Box::new(Trie::new()));
        }
        curr.is_end = true;
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for i in word.bytes().map(|c| ((c - b'a') as usize) as usize){
            match &curr.children[i]{
                None => return false,
                Some(node) => curr = node,
            }
        }
        curr.is_end
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for i in prefix.bytes().map(|c| ((c - b'a') as usize) as usize){
            match &curr.children[i]{
                None => return false,
                Some(node) => curr = node,
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */