impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry:i32 = 0;
        for digit in digits.iter_mut().rev(){
            match digit{
                digit @ 0..=8 => {
                    *digit += 1;
                    return digits;
                }
                digit @ 9 => {
                    *digit = 0;
                },
                _ => panic!(),
            }
        }        
        
        digits.insert(0, 1);
        digits
    }
}