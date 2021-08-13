impl Solution {
    pub fn count_and_say(n: i32) -> String {
        count_and_say(n)
    }
}

fn count_and_say(n: i32) -> String {
    if n == 1{
        return 1.to_string();
    }
    
    let mut s = count_and_say(n-1);
    //println!("{} => {}",n-1,s);
    let t = s.chars().fold((String::from(""), '-', 0) , |(ans,prev,count), c| {
            match c==prev{
                true => (ans, c, count+1),
                false => {
                    //println!("{},{},{}",ans,prev,count);
                    if prev == '-'{
                        return (String::from(""), c, 1);
                    }
                    return (format!("{}{}{}",ans,count,prev), c, 1);
                },
            }
        }
    );
    format!("{}{}{}",t.0,t.2,t.1)
}