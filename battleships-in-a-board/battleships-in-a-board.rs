// https://leetcode.com/problems/battleships-in-a-board/discuss/90902/Simple-Java-Solution

impl Solution {
    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len(){
            for j in 0..board[0].len(){
                if board[i][j] == '.' { continue; }
                if i > 0 && board[i-1][j] == 'X' { continue; }
                if j > 0 && board[i][j-1] == 'X' { continue; }
                count+=1;
            }
        }
        count
    }
}