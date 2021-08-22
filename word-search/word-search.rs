impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for i in (0..board.len()){
            for j in (0..board[0].len()){
                if backtrack(&mut board, 0, &word.chars().collect(), i as i32,j as i32){
                    return true;
                }    
            }
        }
        false
    }
}

fn backtrack(board: &mut Vec<Vec<char>>, pos:usize, word:&Vec<char>, i:i32, j:i32) -> bool{
    let x = i as usize;
    let y = j as usize;
    if pos == word.len(){
        return true;
    } else if i<0 || j<0 || x>= board.len() || y>= board[0].len() || 
                board[x][y]=='.' || board[x][y] != word[pos]{
        return false;
    } 
    //println!("{}:{} ==? board:{}",pos,word[pos],board[i][j]);
    
    let c = board[x][y];
    board[x][y] = '.';
    if backtrack(board, pos+1, word, i,j-1) || 
       backtrack(board, pos+1, word, i,j+1) ||
       backtrack(board, pos+1, word, i-1,j) ||
       backtrack(board, pos+1, word, i+1,j) {
        return true;
    }
    board[x][y] = c;
    return false;
}