use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_map:HashMap<char,i32> = HashMap::new();
        let mut col_map:Vec<HashMap<char,i32>> = vec![HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new()];
        let mut square_map:Vec<HashMap<char,i32>> = vec![HashMap::new(),HashMap::new(),HashMap::new()];
        
        for (i,row) in board.iter().enumerate(){
            //println!("clear row_map");
            row_map.clear();
            if i%3 == 0{
                for m in (0..3){
                    square_map[m].clear();
                }
                //println!("clear square_map");
            }
            for (j,c) in row.iter().enumerate(){
                match c{
                    '.' => continue,
                    &c @'1'..='9' => {
                        //println!("{},{}: {}",i,j,c);
                        if row_map.insert(c, 1).is_some(){
                            //println!("break row_map: {:?}", row_map);
                            return false;
                        }  
                        if col_map[j].insert(c, 1).is_some(){
                            //println!("break col_map[{}]: {:?}", j, col_map[j]);
                            return false;
                        }
                        let k = j/3;
                        if square_map[k].insert(c, 1).is_some(){
                            //println!("break square_map[{}]: {:?}",k, square_map[k]);
                            return false;
                        }
                    },
                    _ => panic!(),
                }
            }
        }
        
        true
    }
}