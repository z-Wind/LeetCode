use std::collections::HashMap;
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut dp:HashMap<usize, i32> = HashMap::new();
        
        let mut max_e = 0;
        for i in 0..envelopes.len(){
            max_e = max_e.max(max_envelopes(&mut dp, &envelopes, i));    
        }
        // println!("{:?}", dp);
        max_e
    }
}

fn max_envelopes(
    dp: &mut HashMap<usize, i32>,
    envelopes: &Vec<Vec<i32>>,
    start: usize,
) -> i32 {
    if dp.contains_key(&start) {
        return *dp.get(&start).unwrap();
    }

    let mut max_e:i32 = 0;
    for i in 0..envelopes.len() {
        if i != start
            && envelopes[i][0] > envelopes[start][0]
            && envelopes[i][1] > envelopes[start][1]
        {
            let temp = max_envelopes(dp, envelopes, i);
            if temp > max_e{
                max_e = temp;
            }
        }
    }
    max_e += 1;
    // println!("{:?}:{}", envelopes[start], max_e);
    dp.insert(start,max_e);
    return max_e;
}
