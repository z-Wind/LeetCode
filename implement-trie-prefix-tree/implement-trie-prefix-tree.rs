use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug)]
struct Node {
    val: char,
    children: HashMap::<char,Box<Node>>,
}
impl Node{
    fn new(val:char) -> Self{
        Node{
            val,
            children:HashMap::new(),
        }    
    }
}

#[derive(Debug)]
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
            prefix_tree:Box::new(Node::new('0')),
            set:HashSet::new(),
        }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut children = &mut self.prefix_tree.children;
        for c in word.chars(){
            let entry = children.entry(c).or_insert(Box::new(Node::new(c)));
            children = &mut entry.children;
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
            match children.get(&c){
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