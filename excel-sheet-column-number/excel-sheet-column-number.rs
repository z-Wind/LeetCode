impl Solution {
    pub fn title_to_number(mut column_title: String) -> i32 {
        if column_title.is_empty(){
            return 0;
        }
        let c = column_title.pop().unwrap();
    
        (c as u8 - b'A' + 1) as i32 + 26 * Self::title_to_number(column_title)
    }
}