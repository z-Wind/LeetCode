// it does not care range like [246,176,131,161]
// so use String::from_utf8 would cause fail

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut count = 0;
        for c in data{
            if count == 0{
                if (c>>7) == 0b0{
                    count = 0;
                } else if  (c>>5) == 0b110{
                    count = 1;
                } else if  (c>>4) == 0b1110{
                    count = 2;
                } else if  (c>>3) == 0b11110{
                    count = 3;
                } else {
                    return false;
                }    
            } else {
                if (c>>6) != 0b10{
                    return false;
                }
                count -= 1;
            }
        }
        count == 0
    }
}