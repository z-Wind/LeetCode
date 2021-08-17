impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        
        let mut m = matrix[0].len();
        let mut n = matrix.len();
        let number = m*n;
        if number == 1{
            return vec![matrix[0][0]];
        } else if n == 1{
            return matrix[0].clone();
        } else if m == 1{
            return matrix.iter_mut().fold(vec![], |mut vec, mut x| {
                vec.append(&mut x);
                vec
                }
                );
        }
        
        let mut start_x = 0;
        let mut start_y = 0;
        let mut next = 0;
        'outer: loop{
            //println!("start: {},{}", start_x, start_y);
            for i in (0..m){
                //println!("1: {}", matrix[start_y][start_x+i]);
                ans.push(matrix[start_y][start_x+i]);
                next += 1;
                if next == number{
                    break 'outer;
                }
            }
            for j in (1..n){
                //println!("2: {}", matrix[start_y+j][start_x+m-1]);
                ans.push(matrix[start_y+j][start_x+m-1]);
                next += 1;
                if next == number{
                    break 'outer;
                }
            }
            for i in (0..m-1).rev(){
                //println!("3: {}", matrix[start_y+n-1][start_x+i]);
                ans.push(matrix[start_y+n-1][start_x+i]);
                next += 1;
                if next == number{
                    break 'outer;
                }
            }
            for j in (1..n-1).rev(){
                //println!("4: {}", matrix[start_y+j][start_x]);
                ans.push(matrix[start_y+j][start_x]);
                next += 1;
                if next == number{
                    break 'outer;
                }
            }
            m -= 2;
            n -= 2;
            start_x += 1;
            start_y += 1;
        }
        
        ans
    }
}