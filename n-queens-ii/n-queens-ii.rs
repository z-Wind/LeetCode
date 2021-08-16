impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n == 1{
            return 1;
        }        
        
        let mut n_queens = NQueens::new(n);
        n_queens.find_locs();
        
        n_queens.results.len() as i32
    }
}

struct NQueens{
    n: usize,
    locs: Vec<(usize, usize)>,
    results: Vec<Vec<(usize, usize)>>,
    
    rows:Vec<usize>,
    cols:Vec<usize>
}

impl NQueens{
    fn new(n: i32) -> Self{
        NQueens{
            n: n as usize,
            locs: vec![],
            results: vec![],
            rows: (0..n as usize).into_iter().collect(),
            cols: (0..n as usize).into_iter().collect(),
        }
    }
    fn is_valid(&self, i:usize, j:usize) -> bool{
        for (x,y) in self.locs.iter(){
            if ((x-i) as i32).abs() == ((y-j) as i32).abs(){
                return false;
            }
        }
        true
    }
    fn find_locs(&mut self){
        if self.n == self.locs.len(){
            //println!("get: {:?}",self.locs);
            self.results.push(self.locs.clone());
            return;
        }
        
        let rows_num = self.rows.len() - (self.n-1-self.locs.len());
        for i in (0..rows_num){
            if self.rows[i] == self.n{
                continue;
            }
            let x = self.rows[i];
            self.rows[i] = self.n;
            for j in (0..self.cols.len()){
                if self.cols[j] == self.n{
                    continue;
                }
                //println!("{}:({},{})",self.locs.len(),i,j);
                let y = self.cols[j];
                self.cols[j] = self.n;
                if self.is_valid(x,y){
                    self.locs.push((x,y));
                    self.find_locs();
                    self.locs.pop();
                }
                self.cols[j] = y;
            }
            self.rows[i] = x;
        }
    }
}