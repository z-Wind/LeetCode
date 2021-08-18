use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut mps = MPS::new();
        mps.min_path_sum(0,0,&grid)
    }
}

struct MPS{
    dp:HashMap<(usize,usize), i32>,
}

impl MPS{
    fn new() -> Self{
        MPS{
            dp: HashMap::new(),
        }
    }
    fn min_path_sum(&mut self, i: usize, j: usize, map:&Vec<Vec<i32>>) -> i32 {
        //println!("{},{}",i, j);        
        if self.dp.contains_key(&(i,j)){
            return *self.dp.get(&(i,j)).unwrap();
        }
        
        let sum:i32;
        if i == map[0].len() -1 && j == map.len() - 1{
            sum = map[j][i];
        } else if i == map[0].len() -1{
            sum = map[j][i] + self.min_path_sum(i, j+1, map);
        } else if j == map.len() - 1{
            sum = map[j][i] + self.min_path_sum(i+1, j, map);
        } else {
            sum = map[j][i] + min(self.min_path_sum(i+1, j, map), self.min_path_sum(i, j+1, map));
        }
        self.dp.insert((i,j), sum);
        
        sum
    }
}