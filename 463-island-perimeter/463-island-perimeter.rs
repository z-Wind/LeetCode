impl Solution {
    pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for i in 0..grid.len(){
            for j in 0..grid[0].len(){
                if grid[i][j] == 1{
                    island_perimeter(&mut perimeter, &mut grid, i, j);
                    return perimeter   
                }
            }
        }
        panic!("impossible");
    }
}

fn island_perimeter(perimeter:&mut i32, grid: &mut Vec<Vec<i32>>, i:usize, j:usize){
    if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0{
        *perimeter += 1;
        return;
    }
    if grid[i][j] == 2{
        return;
    }
    grid[i][j] = 2;
    
    island_perimeter(perimeter, grid, i-1, j);
    island_perimeter(perimeter, grid, i+1, j);
    island_perimeter(perimeter, grid, i, j-1);
    island_perimeter(perimeter, grid, i, j+1);
}