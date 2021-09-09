use std::collections::VecDeque;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut n = 0;       
        let mut rest = 0;
        for i in (0..gas.len()*2){
            let i = i % gas.len();
            rest += gas[i]-cost[i];
            if rest < 0{
                rest = 0;
                n = 0;
            } else {
                n += 1;
                if n == gas.len(){
                    return ((i+1) % gas.len()) as i32;
                }
            }
        }
        -1
    }
}