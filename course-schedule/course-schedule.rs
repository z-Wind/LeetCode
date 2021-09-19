// http://web.ntnu.edu.tw/~algo/DirectedAcyclicGraph.html
// topological sorting
use std::collections::VecDeque;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut edges:Vec<i32> = vec![0;num_courses];
        let mut adj:Vec<Vec<usize>> = vec![vec![];num_courses];
        for prer in prerequisites{
            // [a, b] means that you to take 'a' you need to take 'b' first  
            let a = prer[0] as usize;
            let b = prer[1] as usize;
            //  b -> a
            adj[b].push(a);
            edges[a] += 1;
        }
        
        let mut queue = VecDeque::new();
        for i in (0..num_courses){
            if edges[i] == 0{
                queue.push_back(i);
            } 
        }
        
        let mut result = vec![];
        for i in (0..num_courses){
            if queue.is_empty(){
                break;
            }
            let s = queue.pop_front().unwrap();
            edges[s] = -1;
            result.push(s);
            for &t in adj[s].iter(){
                edges[t] -= 1;
                if edges[t] == 0{
                    queue.push_back(t);
                } 
            }
        }
        
        result.len() == num_courses
    }
}