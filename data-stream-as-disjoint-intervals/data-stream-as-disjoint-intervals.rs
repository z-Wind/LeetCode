use std::collections::BTreeSet;
struct SummaryRanges {
    set:BTreeSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    fn new() -> Self {
        Self{
            set:BTreeSet::new(),
        }
    }
    
    fn add_num(&mut self, val: i32) {
        self.set.insert(val);
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = vec![];
        let mut cur = None;
        let mut i = 0;
        for &val in self.set.iter(){
            // println!("{}: {:?}", val, cur);
            match cur{
                Some(x) => {
                    if x+1 == val{
                        cur = Some(val);
                        continue;
                    }
                    
                    ans[i].push(x);
                    ans.push(vec![val]);
                    i+=1;
                    cur = Some(val);
                },
                None => {
                    ans.push(vec![val]);
                    cur = Some(val);
                },
            }
        }
        ans[i].push(cur.unwrap());
        ans
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */