impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut temp:Vec<i32> = Vec::new();
        
        sum(&mut ans, &mut temp, 1, k as usize,n);
        
        ans
    }
}

fn sum(ans:&mut Vec<Vec<i32>>, temp:&mut Vec<i32>, start:i32, k:usize, target:i32){
    if temp.len() == k && target==0{
        ans.push(temp.clone());
        return;
    } else if temp.len() > k || target < 0{
        return;
    } 
    
    for num in (start..=9){
        temp.push(num);
        sum(ans,temp,num+1,k,target-num);
        temp.pop();
    }
}