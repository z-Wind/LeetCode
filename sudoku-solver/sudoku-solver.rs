use std::collections::HashSet;
use std::collections::HashMap;

struct Sudoku{
    row_map: Vec<HashSet<char>>,
    col_map: Vec<HashSet<char>>,
    square_map: Vec<HashSet<char>>,
    num_space: i32,
}

impl Sudoku{
    fn new() -> Self{
        Sudoku{
            row_map: vec![HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9)],
            col_map: vec![HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9)],
            square_map: vec![HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9),HashSet::with_capacity(9)],
            num_space: 0,
        }
    }
    fn view(&mut self, board: &mut Vec<Vec<char>>){
        for (i,row) in board.iter().enumerate(){
            for (j,c) in row.iter().enumerate(){
                let k = j/3+(i/3)*3;
                match c{
                    '.' => {
                        self.num_space += 1;
                    },
                    &c @'1'..='9' => {
                        self.row_map[i].insert(c);
                        self.col_map[j].insert(c);
                        self.square_map[k].insert(c);
                    },
                    _ => panic!(),
                }
            }
        }
    }
    fn check(&self, i:usize, j:usize, c: &char) -> bool{
        let k = j/3+(i/3)*3;
        !self.row_map[i].contains(c) && 
        !self.col_map[j].contains(c) && 
        !self.square_map[k].contains(c)
    }
    fn insert(&mut self, i:usize, j:usize, c: char){
        let k = j/3+(i/3)*3;
        self.row_map[i].insert(c);
        self.col_map[j].insert(c);
        self.square_map[k].insert(c);
    }
    fn remove(&mut self, i:usize, j:usize, c: char){
        let k = j/3+(i/3)*3;
        self.row_map[i].remove(&c);
        self.col_map[j].remove(&c);
        self.square_map[k].remove(&c);
    }
    fn fill_right(&mut self, board: &mut Vec<Vec<char>>){
        while self.num_space > 0{
            let mut done = true;
            for (i,row) in board.iter_mut().enumerate(){
                for (j,c) in row.iter_mut().enumerate(){
                    if *c == '.'{
                        let mut v = vec![];
                        for c in ('1'..='9'){
                            if self.check(i,j,&c){
                                v.push(c);   
                            }
                        }
                        //println!("{},{} => {:?}",i,j,v); 
                        if v.len() == 1{
                            self.insert(i,j,v[0]);
                            *c = v[0];
                            self.num_space -= 1;
                            done = false;
                        }
                    }
                }
            }
            //println!("{:?}", board);
            if done{
                break;
            }
        }
    }
    fn fill_only_by_row(&mut self, board: &mut Vec<Vec<char>>){
        for i in (0..board.len()){
            let mut map:HashMap<char,i32> = HashMap::new();
            for j in (0..board[0].len()){
                if board[i][j] == '.'{
                    for c in ('1'..='9'){
                        if self.check(i,j,&c){
                            *map.entry(c).or_insert(0) += 1;   
                        }
                    }
                }
            }
            //println!("{:?}", map);
            for (k,v) in map{
                if v == 1{
                    for j in (0..board[0].len()){
                        if board[i][j] == '.'{
                            if self.check(i,j,&k){
                                self.insert(i,j,k);
                                board[i][j] = k;
                                self.num_space -= 1;
                            }
                        }
                    }
                }
            }
        }
    }
    fn fill_only_by_col(&mut self, board: &mut Vec<Vec<char>>){
        for j in (0..board[0].len()){
            let mut map:HashMap<char,i32> = HashMap::new();
            for i in (0..board.len()){
                if board[i][j] == '.'{
                    for c in ('1'..='9'){
                        if self.check(i,j,&c){
                            *map.entry(c).or_insert(0) += 1;   
                        }
                    }
                }
            }
            //println!("{:?}", map);
            for (k,v) in map{
                if v == 1{
                    for i in (0..board.len()){
                        if board[i][j] == '.'{
                            if self.check(i,j,&k){
                                self.insert(i,j,k);
                                board[i][j] = k;
                                self.num_space -= 1;
                            }
                        }
                    }
                }
            }
        }
    }
    fn fill_only_by_square(&mut self, board: &mut Vec<Vec<char>>){
        //let k = j/3+(i/3)*3;
        for k in (0..9){
            let mut map:HashMap<char,i32> = HashMap::new();
            for i in (0..3){
                for j in (0..3){
                    let j = j+k%3*3;
                    let i = i+k/3*3;
                    if board[i][j] == '.'{
                        for c in ('1'..='9'){
                            if self.check(i,j,&c){
                                *map.entry(c).or_insert(0) += 1;   
                            }
                        }
                    }
                }
            }
            //println!("{:?}", map);
            for (key,v) in map{
                if v == 1{
                    for i in (0..3){
                        for j in (0..3){
                            let j = j+k%3*3;
                            let i = i+k/3*3;
                            if board[i][j] == '.'{
                                if self.check(i,j,&key){
                                    self.insert(i,j,key);
                                    board[i][j] = key;
                                    self.num_space -= 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn fill_try(&mut self, board: &mut Vec<Vec<char>>, mut i:usize, mut j:usize) -> bool{  
        if self.is_done(){ return true };        
        
        
        while board[i][j] != '.'{
            i = i + (j+1)/board[i].len();
            j = (j + 1) % board[i].len();
        }

        let mut v = vec![];
        for c in ('1'..='9'){
            if self.check(i,j,&c){
                v.push(c);   
            }
        }
        
        for c in v{
            self.insert(i,j,c); 
            board[i][j] = c;
            self.num_space -= 1;

            let i_next = i + (j+1)/board[i].len();
            let j_next = (j + 1) % board[i].len();
            if self.is_done() || self.fill_try(board,i_next,j_next){
                return true;
            }

            board[i][j] = '.';
            self.num_space += 1;
            self.remove(i,j,c); 
        }
        
        return false;    
    }
    fn is_done(&self) -> bool{
        self.num_space == 0
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {        
        let mut sudoku = Sudoku::new();
        sudoku.view(board);
        //println!("row_map: {:?}",sudoku.row_map);
        //println!("col_map: {:?}",sudoku.col_map);
        //println!("square_map: {:?}",sudoku.square_map);
        
        sudoku.fill_right(board);
        // println!("{:?}",board);
        sudoku.fill_only_by_row(board);
        // println!("{:?}",board);
        sudoku.fill_only_by_col(board);
        // println!("{:?}",board);
        sudoku.fill_only_by_square(board);
        // println!("{:?}",board);
        sudoku.fill_right(board);
        //println!("{:?}",board);
        sudoku.fill_try(board,0,0);
        //println!("{:?}",board);
    }
}