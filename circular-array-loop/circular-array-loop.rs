#[derive(Clone, Copy, Debug)]
enum Status{
    New,
    Process,
    Fail,
}
impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut done = vec![Status::New;n];
        for i in 0..n{
            if circular_array_loop(&mut done, &nums, i, nums[i] > 0, 0){
                return true;
            }
        }
        false
    }
}

fn circular_array_loop(done:&mut Vec<Status>, nums: &Vec<i32>, i:usize, is_pos:bool, count:i32) -> bool {
    // println!("{}:{:?}", nums[i], done);
    if (nums[i] > 0) ^ is_pos{
        return false;
    }
    match done[i]{
        Status::Fail => false,
        Status::Process => count > 1,
        Status::New => {
            let n = nums.len();
            done[i] = Status::Process;
            let idx = (n + i + nums[i] as usize) % n;
            if i == idx{
                done[i] = Status::Fail;
                return false;
            }
            if circular_array_loop(done, nums, idx, is_pos, count+1){
                return true;
            }
            done[i] = Status::Fail;
            false       
        },
    }    
}