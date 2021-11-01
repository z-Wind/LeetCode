// https://leetcode.com/problems/reconstruct-itinerary/discuss/78768/Short-Ruby-Python-Java-C%2B%2B

use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {        
        let mut targets = BTreeMap::new();
        for ticket in tickets{
            targets.entry(ticket[0].clone())
            .or_insert(BinaryHeap::new())
            .push(Reverse(ticket[1].clone()));
        }
        // println!("{:?}", targets);
        let mut route = Vec::new();
        visit(&mut route, &mut targets, String::from("JFK"));
        route.reverse();
        route
    }
}

fn visit (route:&mut Vec<String>, targets:&mut BTreeMap<String,BinaryHeap<Reverse<String>>>, airport:String){
    while let Some(locs) = targets.get_mut(&airport){
        if locs.is_empty(){
            break;
        }
        let loc = locs.pop().unwrap().0;
        visit(route, targets, loc);
    }
    route.push(airport);
}