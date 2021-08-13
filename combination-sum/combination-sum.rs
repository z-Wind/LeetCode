impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut csum = CSum::new(candidates, target);
        csum.explore(vec![], 0, 0);
        csum.ans
    }
}

struct CSum{
    candidates: Vec<i32>, 
    target: i32,
    ans: Vec<Vec<i32>>,
}

impl CSum{
    fn new(mut candidates: Vec<i32>, target: i32) -> Self{
        candidates.sort();
        CSum{
            candidates,
            target,
            ans:vec![],
        }
    }
    fn explore (&mut self, mut v:Vec<i32>, i_start:usize, mut cur_sum:i32){
       for i in (i_start..self.candidates.len()){
           //println!("{}:{} {:?} sum:{}",i,self.candidates[i],v,sum);
           let sum = cur_sum + self.candidates[i];
           if sum == self.target{
               let mut t = v.clone();
               t.push(self.candidates[i]);
               self.ans.push(t);
           } else if sum < self.target{
               let mut t = v.clone();
               t.push(self.candidates[i]);
               self.explore(t,i,sum);
           } else{
               break;
           }
        }            
    }
}
