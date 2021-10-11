// find the center of longest path
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1{
            return vec![0];
        } else if n == 2{
            return edges[0].clone();
        }
        
        let n = n as usize;
        let mut links:Vec<Vec<i32>> = vec![vec![];n];
        for edge in edges{
            // a-b
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            
            links[a].push(edge[1]);
            links[b].push(edge[0]);
        }
        
        // println!("{:?}", links);
        // println!("{:?}", edge_num);
        
        let mut record = vec![false;n];
        let (v1,v2) = find_longest(0, &mut record, &links);
        let ans = if v1.len() >= v2.len() { v1 } else { v2 };
        // println!("{:?}", ans);
        
        if ans.len()%2 == 0{
            let mid = ans.len()/2;
            return vec![ans[mid-1],ans[mid]];
        } else {
            let mid = ans.len()/2;
            return vec![ans[mid]];
        }
    }
}

fn find_longest(node:i32, record:&mut Vec<bool> , links: &Vec<Vec<i32>>) -> (Vec<i32>,Vec<i32>){    
    let mut v = vec![node];
    let i = node as usize;
    record[i] = true;
    
    let next_nodes = links[i].iter();    
    let mut max_connected = vec![vec![];2];
    let mut max_breaked = vec![];
    for &j in next_nodes{
        if record[j as usize]{
            continue;
        }
        let (v1,v2) = find_longest(j, record, links);
        if v1.len() > max_connected[0].len(){
            max_connected.swap(0,1);
            max_connected[0] = v1;
        } else if v1.len() > max_connected[1].len(){
            max_connected[1] = v1;
        }
        if v2.len() > max_breaked.len(){
            max_breaked = v2
        }
    }
    v.append(&mut max_connected[0]);
    let mut temp = v.clone();
    temp.reverse();
    temp.append(&mut max_connected[1]);
    if temp.len() >= max_breaked.len(){
        return (v, temp);
    } else {
        return (v, max_breaked);
    }
    
}