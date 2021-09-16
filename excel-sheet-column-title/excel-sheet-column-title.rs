impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        
        let mut q = column_number;
        while q != 0{
            let r = (q-1) % 26;
            result.push((b'A' + r as u8) as char);
            match q%26{
                0 => q=q/26-1,
                _ => q/=26,
            }
        } 
        
        result.chars().rev().collect::<String>()
    }
}