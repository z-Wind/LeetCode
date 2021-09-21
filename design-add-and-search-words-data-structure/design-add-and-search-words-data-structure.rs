#[derive(Debug, Default)]
struct WordDictionary {
    is_end: bool,
    children: [Option<Box<WordDictionary>>;26],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    
    fn add_word(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|c| (c - b'a') as usize){
            curr = curr.children[i].get_or_insert(Box::new(WordDictionary::new()));
        }
        curr.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let mut curr = self;
        for (i,c) in word.iter().enumerate(){
            match c{
                b'.'=> {
                    for node in curr.children.iter() {
                        if node.is_none(){
                            continue;
                        }
                        let new_word = String::from_utf8(word[i+1..].to_vec()).unwrap();
                        if Self::search(node.as_ref().unwrap(), new_word){
                            return true;
                        }
                    }
                    return false;
                },
                b'a'..=b'z' => {
                    let i = (c - b'a') as usize;
                    match &curr.children[i]{
                        None => return false,
                        Some(node) => curr = node,
                    }
                },
                _ => panic!(),
            }
        }
        curr.is_end
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */