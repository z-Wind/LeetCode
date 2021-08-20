impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red = 0;
        let mut blue = nums.len()-1;
        let mut white = 0;
        while white <= blue{
            match nums[white]{
                0 => {
                    nums.swap(red, white);
                    red += 1;
                    white += 1;
                },
                1 => white += 1,
                2 => {
                    nums.swap(blue, white);
                    match blue.checked_sub(1){
                        None => break,
                        Some(x) => blue = x,
                    }
                },
                _ => panic!(),
            }
        }
    }
}