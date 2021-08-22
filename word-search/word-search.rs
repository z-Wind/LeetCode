impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        
        for i in (0..m){
            for j in (0..n){
                if backtrack(&mut board, &(m,n), 0, &word.chars().collect(), i as i32,j as i32){
                    return true;
                }    
            }
        }
        false
    }
}

fn backtrack(board: &mut Vec<Vec<char>>, size:&(usize,usize), pos:usize, word:&Vec<char>, i:i32, j:i32) -> bool{
    let x = i as usize;
    let y = j as usize;
    if pos == word.len(){
        return true;
    } else if i<0 || j<0 || x>= size.0 || y>= size.1 || 
                board[x][y]=='.' || board[x][y] != word[pos]{
        return false;
    } 
    //println!("{}:{} ==? board:{}",pos,word[pos],board[i][j]);
    
    board[x][y] = '.';
    if backtrack(board, size, pos+1, word, i,j-1) || 
       backtrack(board, size, pos+1, word, i,j+1) ||
       backtrack(board, size, pos+1, word, i-1,j) ||
       backtrack(board, size, pos+1, word, i+1,j) {
        return true;
    }
    board[x][y] = word[pos];
    return false;
}