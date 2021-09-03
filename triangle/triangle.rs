use std::collections::HashMap;
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp:HashMap<(usize,usize),i32> = HashMap::new();
        minimum_total(&mut dp, &triangle, 0, 0)
    }
}

fn minimum_total(dp:&mut HashMap<(usize,usize),i32>, triangle: &Vec<Vec<i32>>, row:usize, col:usize) -> i32{
    if row == triangle.len()-1{
        return triangle[row][col];
    }
    if dp.contains_key(&(row,col)){
        return *dp.get(&(row,col)).unwrap()
    }
    
    let sum = triangle[row][col] + minimum_total(dp, triangle, row+1,col).min(minimum_total(dp, triangle, row+1,col+1));
    dp.insert((row,col), sum);
    sum
}