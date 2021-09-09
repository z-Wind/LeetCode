use std::collections::VecDeque;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut queue: VecDeque<(usize,usize, i32)> = VecDeque::new();
        
        for i in (0..gas.len()){
            if cost[i] <= gas[i]{
                let end = i.checked_sub(1).unwrap_or(gas.len()-1);
                queue.push_back((i,end,gas[i]-cost[i]))
            }
        }
        while !queue.is_empty(){
            let len = queue.len();
            for _ in (0..len){
                let (mut cur,end,mut rest) = queue.pop_front().unwrap();
                let i = (cur + 1) % gas.len();
                rest += gas[i]-cost[i];
                if i == end && rest >= 0{
                    return ((end+1) % gas.len()) as i32;
                } else if rest >= 0{
                    queue.push_back((i,end,rest));
                }
            }
        }
        -1
    }
}