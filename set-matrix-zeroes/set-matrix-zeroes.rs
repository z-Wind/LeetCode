use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zeros = Zeros::new(matrix);
        zeros.bump();
    }
}

struct Zeros<'a>{
    matrix: &'a mut Vec<Vec<i32>>,
    
    zeros_i:HashSet<usize>,
    zeros_j:HashSet<usize>,
}

impl <'a>Zeros<'a>{
    fn new(matrix: &'a mut Vec<Vec<i32>>) -> Self{
        Zeros{
            matrix,
            zeros_i: HashSet::new(),
            zeros_j: HashSet::new(),
        }
    }
    fn bump(&mut self){
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        'outer: for i in (0..m){
            if self.zeros_i.contains(&i){
                continue;
            }
            for j in (0..n){
                if self.zeros_j.contains(&j){
                    continue;
                }
                if self.matrix[i][j] == 0 {
                    //println!("bump:{}, {}",i,j);
                    self.zeros_i.insert(i);
                    self.zeros_j.insert(j);
                    self.bump_i(i);
                    self.bump_j(j);
                    continue 'outer;
                }
            }
        }
    }
    
    fn bump_i(&mut self, i:usize){
        for j in (0..self.matrix[0].len()){
            if self.matrix[i][j] == 0 && !self.zeros_j.contains(&j){
                //println!("bump_i:{}",i);
                self.zeros_j.insert(j);
                self.bump_j(j);
            } else{
                self.matrix[i][j] = 0;
            }
        }
    }
    fn bump_j(&mut self, j:usize){
        for i in (0..self.matrix.len()){
            if self.matrix[i][j] == 0 && !self.zeros_i.contains(&i){
                //println!("bump_j:{}",j);
                self.zeros_i.insert(i);
                self.bump_i(i);
            } else{
                self.matrix[i][j] = 0;
            }
        }  
    }
}