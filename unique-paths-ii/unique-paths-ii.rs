use std::collections::HashMap;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[obstacle_grid.len()-1][obstacle_grid[0].len()-1] == 1{
            return 0;
        }
        let mut uni = UNI::new();
        uni.unique_paths(0,0,&obstacle_grid)
    }
}

struct UNI{
    dp:HashMap<(usize,usize), i32>,
}

impl UNI{
    fn new() -> Self{
        UNI{
            dp: HashMap::new(),
        }
    }
    fn unique_paths(&mut self, i: usize, j: usize, map:&Vec<Vec<i32>>) -> i32 {
        //println!("{},{}",i, j);
        if i == map[0].len()-1 && j == map.len()-1{
            return 1;
        } else if i == map[0].len() || j == map.len(){
            return 0;
        } if map[j][i] == 1{
            return 0;
        }
        
        if self.dp.contains_key(&(i,j)){
            return *self.dp.get(&(i,j)).unwrap();
        }
        
        let step = self.unique_paths(i+1, j, map) + self.unique_paths(i, j+1, map);
        self.dp.insert((i,j), step);
        
        step
    }
}