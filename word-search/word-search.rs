impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 1 && board[0].len() == 1{ 
            return word.len() == 1 && board[0][0] == word.chars().next().unwrap();
        }
        for i in (0..board.len()){
            for j in (0..board[0].len()){
                if backtrack(&mut board, 0, &word.chars().collect(), i,j){
                    return true;
                }    
            }
        }
        false
    }
}

fn backtrack(board: &mut Vec<Vec<char>>, pos:usize, word:&Vec<char>, i:usize, j:usize) -> bool{
    if pos == word.len(){
        return true;
    } else if i>= board.len() || j>= board[0].len() || board[i][j]=='.'{
        return false;
    } 
    //println!("{}:{} ==? board:{}",pos,word[pos],board[i][j]);
    if board[i][j] == word[pos]{
        let c = board[i][j];
        board[i][j] = '.';
        match i.checked_sub(1){
            None => (),
            Some(i) => {
                if backtrack(board, pos+1, word, i,j){
                    return true;
                }
            }
        }
        match j.checked_sub(1){
            None => (),
            Some(j) => {
                if backtrack(board, pos+1, word, i,j){
                    return true;
                }
            }
        }
        if backtrack(board, pos+1, word, i,j+1) | backtrack(board, pos+1, word, i+1,j){
            return true;
        }
        board[i][j] = c;
    }
    false
}