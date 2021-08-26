// https://stackoverflow.com/questions/4166106/the-nth-gray-code/4166615#4166615
// G(i) = i^ (i/2)
 //    N                    N>>1                  gray
 // 0000           .        0000           .      0000           .
 //    | >xor = 0001             >xor = 0000           >xor = 0001
 // 0001          .         0000          .       0001          .
 //   || >xor = 0011           | >xor = 0001           >xor = 0010
 // 0010           .        0001           .      0011           .
 //    | >xor = 0001             >xor = 0000           >xor = 0001
 // 0011         .          0001         .        0010         .
 //  ||| >xor = 0111          || >xor = 0011           >xor = 0100
 // 0100                    0010                  0110
// (i xor i+1) xor (i>>1 xor i+1>>1) == (i xor i>>1) xor (i+1 xor i+1>>1) == gray (i) xor gray (i+1)
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let length = 2_i32.pow(n as u32);
        let mut result = vec![0; length as usize];
        
        for i in 0..length {
            result[i as usize] = i ^ (i >> 1);
        }
        return result;
    }
}

