#[derive(Debug, Default)]
struct WordDictionary {
    word: String,
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
        curr.word = word;
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut words_dict = Box::new(WordDictionary::new());
        for word in words{
            words_dict.add_word(word);
        }
        
        let mut ans:Vec<String> = Vec::new();
        for i in (0..board.len()){
            for j in (0..board[0].len()){
                find_words(&mut ans, &mut board, &mut words_dict, i, j) 
            }
        }
        
        ans
    }
}

fn find_words(ans: &mut Vec<String>, board: &mut Vec<Vec<char>>, node: &mut Box<WordDictionary>, i:usize, j:usize) {
    // println!("{},{}: {}",i,j,temp);
    if i >= board.len() || j >= board[0].len(){
        return;
    }    
    
    match board[i][j]{
        '_' => return,
        c => {
            board[i][j] = '_';
            if let Some(next_node) = &mut node.children[(c as u8 - b'a') as usize]{
                if !next_node.word.is_empty(){
                    ans.push(next_node.word.drain(..).collect());
                }
                find_words(ans, board, next_node,i-1,j);
                find_words(ans, board, next_node,i+1,j);
                find_words(ans, board, next_node,i,j-1);
                find_words(ans, board, next_node,i,j+1);
            }
            board[i][j] = c;
        }
    }        
}