impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {   
        let mut ops = expression.chars()
            .enumerate()
            .filter_map(|e| if matches!(e.1,'+' | '-' | '*') { Some(e.0) } else { None })
            .collect::<Vec<usize>>();
        // println!("{:?}", ops);
        compute(&expression, &mut ops, 0, expression.len()-1)
    }
}

fn compute(s: &str, ops:&mut Vec<usize>, start:usize, end:usize) -> Vec<i32> {
    // println!("{}", &s[start..=end]);
    if end-start < 2{
        return vec![s[start..=end].parse().unwrap()];
    }
    let mut ans = Vec::new();
    for i in (0..ops.len()){
        if ops[i] < start || end < ops[i]{
            continue;
        }
        let op_idx = ops[i];
        match &s[op_idx..op_idx+1]{
            "+" => {
                for x in compute(s,ops,start,op_idx-1){
                    for y in compute(s,ops,op_idx+1,end){
                        ans.push(x+y);        
                    }
                }
            },
            "-" => {
                for x in compute(s,ops,start,op_idx-1){
                    for y in compute(s,ops,op_idx+1,end){
                        ans.push(x-y);        
                    }
                }
            },
            "*" => {
                for x in compute(s,ops,start,op_idx-1){
                    for y in compute(s,ops,op_idx+1,end){
                        ans.push(x*y);        
                    }
                }
            },
            _ => panic!(),
        }
    }
    ans
}