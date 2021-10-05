impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        let mut lifes = vec![vec![0;n];m];
        for i in (0..m){
            for j in (0..n){
                // println!("{},{}",i,j);
                let mut life = 0;
                for k in (i.saturating_sub(1)..=(m-1).min(i+1)){
                    for l in (j.saturating_sub(1)..=(n-1).min(j+1)){
                        // println!("  {},{}",k,l);
                        life += board[k][l];
                    }
                }
                lifes[i][j] = life - board[i][j]; 
            }
        }
        // println!("{:?}", lifes);
        for i in (0..m){
            for j in (0..n){
                match lifes[i][j]{
                    2 => (),
                    3 => board[i][j] = 1,
                    _ => board[i][j] = 0,
                }
            }
        }
    }
}