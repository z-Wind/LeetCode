// use 84. Largest Rectangle in Histogram
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty(){
            return 0;
        } else if matrix[0].is_empty(){
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prev:Vec<i32> = vec![0;n];
        
        let mut max_area = 0;
        let mut stack:Vec<(i32,i32)> = vec![(-1,0)];
        for i in (0..m){
            for j in (0..n){
                match matrix[i][j]{
                    '0' => prev[j] = 0,
                    '1' => prev[j] +=1,
                    _ => panic!(),
                }
                let h = prev[j];
                while h < stack.last().unwrap().1{
                    let (_,h_stack) = stack.pop().unwrap();
                    let w = j as i32 - stack.last().unwrap().0 -1;
                    max_area = max_area.max(w*h_stack);
                } 
                stack.push((j as i32, h));
            }
            while stack.len() > 1{
                let (_,h_stack) = stack.pop().unwrap();
                let w = n as i32 - stack.last().unwrap().0 -1;
                max_area = max_area.max(w*h_stack);
            } 
        }
        
        max_area as i32
    }
}

