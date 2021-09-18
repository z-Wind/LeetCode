impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in (0..grid.len()){
            for j in (0..grid[0].len()){
                if island_check(&mut grid, i ,j){
                    count += 1;
                }
            }
        }
        // println!("{:?}", grid);
        count
    }
}

fn island_check(grid: &mut Vec<Vec<char>>, i:usize, j:usize) -> bool{
    if i == grid.len() || j == grid[0].len(){
        return false;
    }
    
    match grid[i][j]{
        '1' => {
            grid[i][j] = '_';
            island_check(grid,i.saturating_sub(1),j);
            island_check(grid,i+1,j);
            island_check(grid,i,j.saturating_sub(1));
            island_check(grid,i,j+1);
            return true;
        },
        _ => return false,
    }
}