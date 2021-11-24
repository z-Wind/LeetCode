// it does not care range like [246,176,131,161]
// so use String::from_utf8 would cause fail

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let n = data.len();
        let mut count = 0;
        let mut i = 0;
        while i < n{
            count = if (data[i]>>7) == 0b0{
                1
            } else if  (data[i]>>5) == 0b110{
                2
            } else if  (data[i]>>4) == 0b1110{
                3
            } else if  (data[i]>>3) == 0b11110{
                4
            } else {
                return false;
            };
            // println!("{}:{}", data[i], count);
            for _ in 0..count-1{
                i+=1;
                // println!("{:08b}", data[i]);
                if i >= n || (data[i]>>6) != 0b10{
                    return false;
                }
            }
            i+=1;
        }
        true
    }
}