use std::collections::HashSet;
#[derive(Debug, Default)]
struct WordDictionary {
    is_end: bool,
    children: [Option<Box<WordDictionary>>;26],
}

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
    
    fn search_char(&self, c: u8) -> &Option<Box<WordDictionary>> {
        let mut curr = self;
 
        let i = (c - b'a') as usize;
        return &curr.children[i]
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut words_dict = WordDictionary::new();
        for word in words{
            words_dict.add_word(word);
        }
        
        let mut ans:HashSet<String> = HashSet::new();
        let mut temp = String::new();
        let words_dict = Some(Box::new(words_dict));
        for i in (0..board.len()){
            for j in (0..board[0].len()){
                find_words(&mut ans,&mut temp, &mut board, &words_dict, i, j) 
            }
        }
        
        ans.into_iter().collect()
    }
}

fn find_words(ans: &mut HashSet<String>,temp: &mut String, board: &mut Vec<Vec<char>>, node: &Option<Box<WordDictionary>>, i:usize, j:usize) {
    // println!("{},{}: {}",i,j,temp);
    if i >= board.len() || j >= board[0].len(){
        return;
    }    
    
    match board[i][j]{
        '_' => return,
        c => {
            board[i][j] = '_';
            let next_node = node.as_ref().unwrap().search_char(c as u8);
            if next_node.is_some(){
                temp.push(c);
                if next_node.as_ref().unwrap().is_end{
                    ans.insert(temp.clone());
                }
                find_words(ans, temp, board, next_node,i-1,j);
                find_words(ans, temp, board, next_node,i+1,j);
                find_words(ans, temp, board, next_node,i,j-1);
                find_words(ans, temp, board, next_node,i,j+1);
                temp.pop();
            }
            board[i][j] = c;
        }
    }        
}