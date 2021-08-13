use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut i:usize = 0;
        let mut v:Vec<usize> = vec![];
        let mut set:HashSet<usize> = HashSet::new();
        for num in nums{
            if num.is_positive(){
                let num = num as usize;
                if set.contains(&num){
                    continue;
                }
                
                //println!("len:{},num:{}",v.len(),num);
                if v.len() == 0 || (num > v.len() && num > *v.last().unwrap()) {
                    v.push(num);
                } else if  num > v.len() && num < *v.last().unwrap(){
                    let mut j=v.len()-1;
                    while j>= 0 && v[j] > num{
                        match j.overflowing_sub(1){
                            (x,true) => {
                                j=x;
                                break;
                            },
                            (x,false) => j=x,
                        }
                    }
                    v.insert(j+1,num);
                } else if num < v[0]{
                    v.insert(0,num);
                } else if num > v[0]{
                    let mut m=0;
                    while m < v.len() && v[m] < num{
                        m+=1;
                    }
                    v.insert(m,num);
                }else {
                    v.insert(num-1,num);
                }
                if num == i+1 {
                    i = num;
                    while i < v.len() && i+1 == v[i]{
                        i+=1;
                    }
                }
                //println!("{:?}",v);
                set.insert(num);
            }
        }
        //println!("{:?}",v);
        (i+1) as i32
    }
}