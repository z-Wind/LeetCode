use std::collections::VecDeque;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as usize;
        if n <= 2{
            return (0..n as i32).collect();
        }
        
        let mut links:Vec<Vec<usize>> = vec![Vec::new();n];
        let mut edge_num:Vec<i32> = vec![0;n];
        for edge in edges{
            // a-b
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            
            links[a].push(b);
            links[b].push(a);
            edge_num[a] += 1;
            edge_num[b] += 1;
        }
        
        // println!("{:?}", links);
        let mut leaves = VecDeque::new();
        for i in (0..n){
            if edge_num[i] == 1{
                leaves.push_back(i);
            }
        }
        
        while n > 2{
            n-=leaves.len();
            for _ in (0..leaves.len()){
                let i = leaves.pop_front().unwrap();
                for &node in links[i].iter(){
                    edge_num[node] -= 1;
                    if edge_num[node] == 1{
                        leaves.push_back(node);
                    }
                }
            }
        }
        leaves.into_iter().map(|x| x as i32).collect()
    }
}
