impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut n = n as usize;
        let mut matrix:Vec<Vec<i32>> = vec![vec![0;n];n];
        
        let number:i32 = (n*n) as i32;
        let mut start_x = 0;
        let mut start_y = 0;
        let mut next = 1;
        loop{
            //println!("start: {},{}", start_x, start_y);
            for i in (0..n){
                matrix[start_y][start_x+i] = next;
                next += 1;
            }
            if next > number{
                break;
            }
            
            for j in (1..n){
                matrix[start_y+j][start_x+n-1] = next;
                next += 1;
            }
            if next > number{
                break;
            }
            
            for i in (0..n-1).rev(){
                matrix[start_y+n-1][start_x+i] = next;
                next += 1;
            }
            if next > number{
                break;
            }
            
            for j in (1..n-1).rev(){
                matrix[start_y+j][start_x] = next;
                next += 1;
            }
            if next > number{
                break;
            }
            
            n -= 2;
            start_x += 1;
            start_y += 1;
        }
        
        //println!("{:?}", matrix);
        matrix
    }
}