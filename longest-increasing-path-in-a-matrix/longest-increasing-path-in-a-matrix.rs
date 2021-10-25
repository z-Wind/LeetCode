use std::cmp::Ordering;
impl Solution {
    pub fn longest_increasing_path(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![1;n];m];
        let mut map = vec![vec![true;n];m];
        
        let mut ans = 0;
        for i in 0..m{
            for j in 0..n{
                longest_increasing_path(&mut map, &mut ans, &mut dp, &matrix, i, j);      
            }
        }
        // println!("[");
        // for i in 0..m{
        //     println!("{:?}", dp[i]);    
        // }
        // println!("]");
        
        ans
    }
}

fn longest_increasing_path(map:&mut Vec<Vec<bool>>, ans:&mut i32, dp:&mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>, i:usize, j:usize) { 
    if !map[i][j]{
        return;
    }
    map[i][j] = false;
    
    let m = matrix.len();
    let n = matrix[0].len();   
    
    // up
    if i > 0{
        update(map, ans, dp, matrix, (i,j), (i-1,j));
    }
    // down
    if i < m-1{
        update(map, ans, dp, matrix, (i,j), (i+1,j));
    }
    // left
    if j > 0{
        update(map, ans, dp, matrix, (i,j), (i,j-1));
    }
    // right
    if j < n-1{
        update(map, ans, dp, matrix, (i,j), (i,j+1));
    }
    
    map[i][j] = true;
    *ans = (*ans).max(dp[i][j]);
}

fn update(map:&mut Vec<Vec<bool>>, ans:&mut i32, dp:&mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>, cur:(usize,usize), to:(usize,usize)){      
    // println!("{},{} => {},{}", cur.0, cur.1, to.0, to.1);
    match matrix[to.0][to.1].cmp(&matrix[cur.0][cur.1]){
        Ordering::Greater => {
            if dp[to.0][to.1] <= dp[cur.0][cur.1]{
                dp[to.0][to.1] = 1 + dp[cur.0][cur.1];    
                longest_increasing_path(map, ans, dp, matrix, to.0, to.1);
            }
        },
        Ordering::Less => (),
        Ordering::Equal => (),
    }
    
    // println!("[");
    // for i in 0..matrix.len(){
    //     println!("{:?}", dp[i]);    
    // }
    // println!("]");
}