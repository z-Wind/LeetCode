use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut map:HashMap<i32, Vec<i32>> = HashMap::new();
        for prer in prerequisites{
            map.entry(prer[1]).or_insert(vec![]).push(prer[0]);
        }
        // println!("{:?}", map);
        
        let mut checked:Vec<bool> = vec![false;num_courses as usize];
        let mut set:HashSet<i32> = HashSet::new();
        for i in (0..num_courses){
            if checked[i as usize]{
                continue;
            }
            if check_circle(&mut checked, &mut set, &map, i){
                return false;
            }
        }
        true
    }
}

fn check_circle(checked:&mut Vec<bool>, set:&mut HashSet<i32>, map:&HashMap<i32, Vec<i32>>, i:i32) -> bool{ 
    // println!("{}: {:?}", i, set);
    if set.contains(&i){
        return true;
    } else if !map.contains_key(&i) || checked[i as usize]{
        return false;
    }
    
    checked[i as usize] = true;
    set.insert(i);
    let pres = map.get(&i).unwrap();
    let ans = pres.iter().any(|pre| check_circle(checked,set,map,*pre));
    set.remove(&i);
    return ans;
    
}