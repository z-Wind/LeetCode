use std::collections::BTreeMap;
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {        
        let mut map:BTreeMap<String,Vec<String>> = BTreeMap::new();
        let mut len = 1;
        for ticket in tickets{
            map.entry(ticket[0].clone()).or_insert(Vec::new()).push(ticket[1].clone());
            len += 1;
        }
        for v in map.values_mut(){
            v.sort_unstable_by(|a, b| b.cmp(a));
        }
        // println!("{}: {:?}", len, map);
        
        let ans = find_itinerary(&mut Vec::new(), map, String::from("JFK"), len);
        ans.unwrap()
    }
}

fn find_itinerary(temp:&mut Vec<String>, mut map: BTreeMap<String,Vec<String>>, start:String, len:usize) -> Option<Vec<String>>{
    temp.push(start.clone());
    // println!("{:?} => {:?}", temp, map);
    if temp.len() == len{
        return Some(temp.clone());
    }
    
    if let Some(v) = map.get(&start){
        for i in (0..v.len()).rev(){
            let mut map_clone = map.clone();
            let next = map_clone.get_mut(&start).unwrap().remove(i);
            match find_itinerary(temp, map_clone, next, len){
                None => (),
                x => return x,
            }
        }
    }
    
    temp.pop();
    None
}