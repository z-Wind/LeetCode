use std::cmp::Ordering;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split(".");
        let mut v2 = version2.split(".");
        
        loop{
            match (v1.next(), v2.next()){
                (None, None) => return 0,
                (Some(rev1), Some(rev2)) => {
                    match rev1.parse::<i32>().unwrap().cmp(&rev2.parse::<i32>().unwrap()){
                        Ordering::Less => return -1,
                        Ordering::Greater => return 1,
                        _ => (),
                    }
                },
                (Some(rev), None) => {
                    match rev.parse::<i32>().unwrap() == 0{
                        true => (),
                        false => return 1,
                    }
                },
                (None, Some(rev)) => {
                    match rev.parse::<i32>().unwrap() == 0{
                        true => (),
                        false => return -1,
                    }
                },
            }
        }
    }
}