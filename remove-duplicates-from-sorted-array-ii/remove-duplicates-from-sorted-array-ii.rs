impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut i = 0;
        let mut num:Option<i32> = None;
        loop{
            if i == nums.len(){
                break;
            }
            match num{
                None => {
                    count = 1;
                    num = Some(nums[i]);
                },
                Some(x) if x==nums[i] => {
                    count += 1;
                    if count > 2{
                        nums.remove(i);
                        i-=1;
                    }
                },
                _ => {
                    count = 1;
                    num = Some(nums[i]);
                },
            }
            i+=1;
        }
        nums.len() as i32
    }
}