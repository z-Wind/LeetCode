use std::collections::HashMap;
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let sum = minimum_total(&mut triangle, 0, 0);
        //println!("{:?}",triangle);
        sum
    }
}

fn minimum_total(triangle: &mut Vec<Vec<i32>>, row:usize, col:usize) -> i32{
    if row == triangle.len()-1{
        return triangle[row][col];
    }
    let mut sum=0;
    if col == 0{
        sum = triangle[row][col] + minimum_total(triangle, row+1,col).min(minimum_total(triangle, row+1,col+1));
    } else {
        sum = triangle[row][col] + triangle[row+1][col].min(minimum_total(triangle, row+1,col+1));
    }
    triangle[row][col] = sum;
    sum    
}