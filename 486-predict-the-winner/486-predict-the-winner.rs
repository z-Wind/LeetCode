use std::collections::VecDeque;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }
        let mut nums:VecDeque<i32> = nums.into_iter().collect();
        predict_the_winner(&mut nums, 0, 0, 1)
    }
}

fn predict_the_winner(nums: &mut VecDeque<i32>, score1: i32, score2: i32, player: i32) -> bool {
    if nums.is_empty() {
        return score1 >= score2;
    }
    
    let num = nums.pop_front().unwrap();
    let res1 = if player == 1 {
        predict_the_winner(nums, score1 + num, score2, player ^ 0b11)
    } else {
        predict_the_winner(nums, score1, score2 + num, player ^ 0b11)
    };
    nums.push_front(num);
    if player == 1 && res1 {
        return true
    }
    
    let num = nums.pop_back().unwrap();
    let res2 = if player == 1 {
        predict_the_winner(nums, score1 + num, score2, player ^ 0b11)
    } else {
        predict_the_winner(nums, score1, score2 + num, player ^ 0b11)
    };
    nums.push_back(num);
    if player == 1 && res2 {
        return true
    }
    
    res1 && res2
}