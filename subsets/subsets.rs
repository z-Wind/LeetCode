// subset[n] = c(n,n) + c(n,n-1) + c(n,n-2) + .. c(n,1)
// c(n,k) = c(n-1,k-1) + c(n-1,k)
impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = vec![nums.to_vec()]; 

        for i in (0..nums.len()){
            ans.append(&mut combine(&mut nums,i));
        }

        ans
    }
}

fn combine(nums:&mut Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    //println!("{:?},{}", nums, k);
    let mut ans:Vec<Vec<i32>> = Vec::new();
    push(&mut ans, nums, 0, k, 0);
    ans
}


fn push(ans: &mut Vec<Vec<i32>>, nums:&mut Vec<i32>, pos:usize, len:usize, start:usize){
    if len == 0{
        ans.push(vec![]);
        return
    } else if pos == len{
        ans.push(nums[..len].to_vec());
        return;
    }

    if nums.len() - start < len - pos{
        return;
    }

    for i in (start..nums.len()){
        nums.swap(pos,i);
        push(ans, nums, pos+1, len, i+1);
        nums.swap(pos,i);
    }
}