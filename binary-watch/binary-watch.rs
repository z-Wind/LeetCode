impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let times = vec![8,4,2,1,32,16,8,4,2,1];
        let mut ans = Vec::new();
        read_binary_watch(&mut ans, turned_on, &times, 0, 0, 0);
        ans
    }
}

fn read_binary_watch(ans:&mut Vec<String>, n: i32, times:&Vec<i32>, start:usize, hour:i32, minute:i32){
    if hour>11 || minute>59 || start > times.len(){
        return;
    }
    if n == 0{
        ans.push(format!("{}:{:02}", hour, minute));
        return;
    }    
    
    for i in start..times.len(){
        if i < 4{
            read_binary_watch(ans, n-1, times, i+1, hour+times[i], minute);
        } else {
            read_binary_watch(ans, n-1, times, i+1, hour, minute+times[i]);
        }
    }
}