impl Solution {
    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len(){
            for j in 0..board[0].len(){
                if board[i][j] == 'X'{
                    count += 1;
                    mark_battleships(&mut board, i, j);   
                }
            }
        }
        count
    }
}

fn mark_battleships(board: &mut Vec<Vec<char>>, i:usize, j:usize){
    if i >= board.len() || j >= board[0].len(){
        return;
    }
    
    match board[i][j] {
        'X' => {
            board[i][j] = 'O';
            mark_battleships(board, i-1, j);
            mark_battleships(board, i+1, j);
            mark_battleships(board, i, j-1);
            mark_battleships(board, i, j+1);
        },
        _ => (),
    }
}