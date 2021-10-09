struct NumMatrix {
    sums: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let sums:Vec<Vec<i32>> = matrix.iter().map(|nums| {
            nums.iter()
                .scan(0, |sum, &x| {
                    *sum += x;
                    Some(*sum)
                })
                .collect::<Vec<i32>>()
        }).collect();
        // println!("{:?}", sums);
        Self { sums }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        
        // println!("({},{}), ({},{})", row1, col1, row2, col2);
        let mut sum = 0;
        for i in (row1..=row2){
            // println!("{},{}",self.sums[i][col1],self.sums[i][col2]);
            if col1 == 0{
                sum += self.sums[i][col2];
            } else {
                sum += (self.sums[i][col2] - self.sums[i][col1-1]);
            }
        }
        // println!("=========");
        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */