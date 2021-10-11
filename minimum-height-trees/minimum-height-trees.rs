use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as usize;
        if n <= 2{
            return (0..n as i32).collect();
        }
        
        let mut links:Vec<HashSet<usize>> = vec![HashSet::new();n];
        for edge in edges{
            // a-b
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            
            links[a].insert(b);
            links[b].insert(a);
        }
        
        // println!("{:?}", links);
        let mut leaves = VecDeque::new();
        for i in (0..n){
            if links[i].len() == 1{
                leaves.push_back(i);
            }
        }
        
        while n > 2{
            n-=leaves.len();
            for _ in (0..leaves.len()){
                let i = leaves.pop_front().unwrap();
                let node = links[i].drain().next().unwrap();
                links[node].remove(&i);
                if links[node].len() == 1{
                    leaves.push_back(node);
                }
            }
        }
        leaves.into_iter().map(|x| x as i32).collect()
    }
}
