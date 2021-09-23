impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut temp:Vec<i32> = Vec::new();
        
        sum(&mut ans, &mut temp, 1, k as usize, 0,n);
        
        ans
    }
}

fn sum(ans:&mut Vec<Vec<i32>>, temp:&mut Vec<i32>, start:i32, k:usize, curr:i32, target:i32){
    if temp.len() == k && curr == target{
        ans.push(temp.clone());
        return;
    } else if start > 9 || temp.len() > k || curr > target{
        return;
    } 
    
    for num in (start..=9){
        temp.push(num);
        sum(ans,temp,num+1,k,curr+num,target);
        temp.pop();
    }
}