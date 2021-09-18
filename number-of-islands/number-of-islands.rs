impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in (0..grid.len()){
            for j in (0..grid[0].len()){
                if grid[i][j] == '1'{
                    island_connect(&mut grid, i ,j);
                    count += 1;
                }
            }
        }
        // println!("{:?}", grid);
        count
    }
}

fn island_connect(grid: &mut Vec<Vec<char>>, i:usize, j:usize){
    if i == grid.len() || j == grid[0].len() || grid[i][j] != '1'{
        return;
    }
    
    grid[i][j] = '_';
    island_connect(grid,i.saturating_sub(1),j);
    island_connect(grid,i+1,j);
    island_connect(grid,i,j.saturating_sub(1));
    island_connect(grid,i,j+1);

}