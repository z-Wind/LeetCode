use std::collections::BTreeMap;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:BTreeMap<i32,usize> = BTreeMap::new();
        for (i,num) in numbers.into_iter().enumerate(){
            if let Some((_,&j)) = map.get_key_value(&num){
                return vec![j as i32 + 1,i as i32 + 1];
            } else {
                map.insert(target-num,i);
            }
        }
        vec![]
    }
}