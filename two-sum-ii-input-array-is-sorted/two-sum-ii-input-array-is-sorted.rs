impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in (0..numbers.len()){
            let rest = target - numbers[i];
            for j in (i+1..numbers.len()){
                if numbers[j] > rest{
                    break;
                } else if rest == numbers[j]{
                    return vec![i as i32 + 1,j as i32 + 1];
                }
            }
        }
        vec![]
    }
}