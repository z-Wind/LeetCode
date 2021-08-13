impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut csum = CSum::new(candidates, target);
        csum.explore(&mut vec![], 0, 0);
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
        candidates.sort_unstable();
        CSum{
            candidates,
            target,
            ans:vec![],
        }
    }
    fn explore (&mut self, v:&mut Vec<i32>, i_start:usize, mut cur_sum:i32){
        if cur_sum > self.target{
            return;
        } else if cur_sum == self.target{
            self.ans.push(v.clone());
            return;
        }
        for i in (i_start..self.candidates.len()){
            if i>i_start && self.candidates[i] == self.candidates[i-1]{
                continue;
            }
            v.push(self.candidates[i]);
            self.explore(v,i+1,cur_sum + self.candidates[i]);
            v.pop();
        }            
    }
}