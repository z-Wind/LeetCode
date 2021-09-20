use std::collections::HashSet;
#[derive(Debug, Default)]
struct Node {
    children: [Option<Box<Node>>;26],
}
impl Node{
    fn new() -> Self{        
        Default::default()
    }
}

#[derive(Debug, Default)]
struct Trie {
    prefix_tree: Box<Node>,
    set: HashSet<String>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie{
            prefix_tree:Box::new(Node::new()),
            set:HashSet::new(),
        }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut children = &mut self.prefix_tree.children;
        for c in word.chars(){
            let i = (c as u8 - b'a') as usize;
            match children[i]{
                None => children[i] = Some(Box::new(Node::new())),
                Some(_) => (),
            }
            children = &mut children[i].as_mut().unwrap().children;
        }
        // println!("{:?}", self.prefix_tree);
        self.set.insert(word);
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.set.contains(&word)
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut children = &self.prefix_tree.children;
        for c in prefix.chars(){
            let i = (c as u8 - b'a') as usize;
            match &children[i]{
                None => return false,
                Some(node) => children = &node.children,
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