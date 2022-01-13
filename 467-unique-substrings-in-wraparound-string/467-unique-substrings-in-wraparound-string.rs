// p: abcbcdcde
//    *
// count: 1
//         o
// counts: abcde...
// ===================
// abcbcdcde
//  *
// count: 2
//          o
//         oo
// counts: abcde...
// ===================
// abcbcdcde
//   *
// count: 3
//           o
//          oo
//         ooo
// counts: abcde...
// ===================
// abcbcdcde
//    *
// count: 1
//           o
//          oo
//         ooo
// counts: abcde...
// ===================
// abcbcdcde
//     *
// count: 2
//           o
//          oo
//         ooo
// counts: abcde...
// ===================
// abcbcdcde
//      *
// count: 3
//           oo
//          ooo
//         oooo
// counts: abcde...
// ===================
// abcbcdcde
//       *
// count: 1
//           oo
//          ooo
//         oooo
// counts: abcde...
// ===================
// abcbcdcde
//        *
// count: 2
//           oo
//          ooo
//         oooo
// counts: abcde...
// ===================
// abcbcdcde
//         *
// count: 3
//           ooo
//          oooo
//         ooooo
// counts: abcde...
// ===================
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut counts:[i32;26] = [0;26];
        let p = p.as_bytes();
        let n = p.len();
        
        let mut c = (p[0] - b'a') as usize;
        counts[c] = 1;
        let mut count = 1;
        for i in 1..n{
            if p[i] - p[i-1] == 1 || p[i-1] - p[i] == 25 {
                count += 1;
            } else {
                count = 1;
            }
            
            c = (p[i] - b'a') as usize;
            counts[c] = counts[c].max(count);
        }
        // println!("{:?}", counts);
        counts.iter().sum()
    }
}