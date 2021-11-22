impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut chars:Vec<i32> = vec![0;26];
        for c in magazine.bytes(){
            let i = (c - b'a') as usize;
            chars[i] += 1;
        }
        for c in ransom_note.bytes(){
            let i = (c - b'a') as usize;
            chars[i] -= 1;
            if chars[i] < 0{
                return false;
            }
        }
        true
    }
}