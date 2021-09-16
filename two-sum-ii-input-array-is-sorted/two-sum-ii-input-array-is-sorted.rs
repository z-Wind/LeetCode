use std::collections::BTreeMap;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:BTreeMap<i32,Vec<i32>> = BTreeMap::new();
        for (i,num) in numbers.into_iter().enumerate(){
            map.entry(num).or_insert(vec![]).push(i as i32 + 1);
        }
        for (num,i) in map.iter(){
            let rest = target - num;
            if map.contains_key(&rest){
                for j in map.get(&rest).unwrap(){
                    if i[0] != *j{
                        return vec![i[0],*j];
                    }
                }
            }
            
        }
        vec![]
    }
}