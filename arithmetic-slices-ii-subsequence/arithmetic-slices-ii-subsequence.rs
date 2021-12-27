use std::collections::HashSet;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3{
            return 0;
        }
        let mut set = HashSet::new();
        let mut ans:i32 = 0;
        'outer: for i in 0..n-2{
            if set.contains(&nums[i]){
                continue;
            }
            for j in i+1..n-1{
                if nums[i] != nums[j]{
                    continue;
                }
                for k in j+1..n{
                    if nums[i] != nums[k]{
                        continue;
                    }
                    set.insert(nums[i]);
                    let mut count = 3;
                    for l in k+1..n{
                        if nums[i] == nums[l]{
                            count += 1;
                        }    
                    }
                    
                    for i in 3..=count{
                        ans += nChoosek(count,i);
                    }
                    continue 'outer;
                }
            }
        }
        for i in 0..n-2{
            for j in i+1..n-1{
                for k in j+1..n{
                    let prev = nums[j] as i64;
                    let cur = nums[k] as i64;
                    let diff = prev - nums[i] as i64;
                    if diff != 0 && diff == cur - prev{
                        number_of_arithmetic_slices(&mut ans, &nums, k+1, cur, diff);       
                    }
                }
            }
        }
        
        ans
    }
}

fn number_of_arithmetic_slices(ans:&mut i32, nums: &Vec<i32>, start:usize, prev:i64, diff:i64){
    *ans += 1;
    if start >= nums.len(){
        return;
    }
    
    for i in start..nums.len(){
        let cur = nums[i] as i64;
        if diff == cur - prev{
            number_of_arithmetic_slices(ans, nums, i+1, cur, diff);
        }
    }
}

fn nChoosek(n: usize, mut k: usize) -> i32{
    if k > n {return 0;}
    if k * 2 > n {k = n-k;}
    if k == 0 {return 1;}

    let mut result = n;
    for i in 2..=k {
        result *= (n-i+1);
        result /= i;
    }
    result as i32
}