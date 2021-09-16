impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        if column_number == 0{
            return String::from("");
        }
        
        let q = column_number-1;
        Self::convert_to_title(q / 26) + &((b'A' + (q%26) as u8) as char).to_string()
    }
}