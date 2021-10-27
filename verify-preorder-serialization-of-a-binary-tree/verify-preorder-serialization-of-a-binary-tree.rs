impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut nodes:Vec<&str> = preorder.split(',').rev().collect();
        is_valid_serialization(&mut nodes) && nodes.len() == 0
    }
}

fn is_valid_serialization(nodes: &mut Vec<&str>) -> bool {
    // println!("{:?}", nodes.iter().rev().collect::<Vec<_>>());
    match nodes.pop(){
        None => false,
        Some("#") => true,
        Some(_) => is_valid_serialization(nodes) && is_valid_serialization(nodes), 
    }    
}