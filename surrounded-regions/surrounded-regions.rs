impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        // border
        for i in (0..m){
            if board[i][0] == 'O'{
                fill(board,i,0,'C');  
            }
            if board[i][n-1] == 'O'{
                fill(board,i,n-1,'C');  
            }
        }
        for j in (0..n){
            if board[0][j] == 'O'{
                fill(board,0,j,'C');  
            }
            if board[m-1][j] == 'O'{
                fill(board,m-1,j,'C');  
            }
        }
        
        for i in (1..m){
            for j in (1..n){
                if board[i][j] == 'O'{
                    board[i][j] = 'X';
                }
            }
        }
        // println!("{:?}", board);
        for i in (0..m){
            for j in (0..n){
                match board[i][j]{
                    'C' => board[i][j] = 'O',
                    _ => (),
                }
            }
        }
    }
}

fn fill(board: &mut Vec<Vec<char>>, i:usize, j:usize, c:char){
    let m = board.len();
    let n = board[0].len();
    match board[i][j]{
        'O' => {
            board[i][j] = c;
            if i > 0{
                fill(board,i-1,j,c);
            }
            if i < m-1{
                fill(board,i+1,j,c);
            }
            if j > 0{
                fill(board,i,j-1,c);
            }
            if j < n-1{
                fill(board,i,j+1,c);
            }
        },
        _ => (),
    }
}