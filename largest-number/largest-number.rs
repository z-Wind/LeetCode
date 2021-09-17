use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>();
        nums.sort_unstable_by(|a, b| cmp(a.as_bytes(), b.as_bytes()).reverse());
        let s = nums.join("").trim_start_matches('0').to_string();
        if &s == ""{
            return String::from("0");
        }
        s
    }
}

fn cmp(a:&[u8], b:&[u8]) -> Ordering{
    // println!("{},{}", String::from_utf8(a.to_vec()).unwrap(), String::from_utf8(b.to_vec()).unwrap());
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut i = 0;
    loop{
        match (a_iter.next(), b_iter.next()){
            (None, None) => return Ordering::Equal,
            (Some(a), Some(b)) => {
                match a.cmp(b){
                    Ordering::Equal => (),
                    x => return x,
                }
            },
            (Some(x), None) => {
                return cmp(&a[i..],b);
            },
            (None, Some(x)) => {
                return cmp(a,&b[i..]);
            },
        }
        i+=1;
    }
}