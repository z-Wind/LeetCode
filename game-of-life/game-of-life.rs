// https://leetcode.com/problems/game-of-life/discuss/73223/Easiest-JAVA-solution-with-explanation
// [2nd bit, 1st bit] = [next state, current state]
// - 00  dead (next) <- dead (current)
// - 01  dead (next) <- live (current)  
// - 10  live (next) <- dead (current)  
// - 11  live (next) <- live (current) 


impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        for i in (0..m){
            for j in (0..n){
                // println!("{},{}",i,j);
                let mut life = 0;
                for k in (i.saturating_sub(1)..=(m-1).min(i+1)){
                    for l in (j.saturating_sub(1)..=(n-1).min(j+1)){
                        // println!("  {},{}",k,l);
                        life += board[k][l] & 1;
                    }
                }
                match life - (board[i][j] & 1){
                    2 if (board[i][j] & 1) == 1 => board[i][j] = 0b11,
                    3 => board[i][j] |= 0b10,
                    _ => (),
                }
            }
        }
        // println!("{:?}", board);
        for i in (0..m){
            for j in (0..n){
                board[i][j] >>= 1;
            }
        }
    }
}