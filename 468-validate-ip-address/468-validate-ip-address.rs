impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if is_ipv4(&query_ip){
            return String::from("IPv4");
        } else if is_ipv6(&query_ip){
            return String::from("IPv6");
        } else {
            return String::from("Neither");
        }
    }
}

fn is_ipv4(s: &str) -> bool{
    let nums: Vec<&str> = s.split('.').collect();
    if nums.len() != 4{
        return false;
    }
    for num in nums{
        if num.len() > 1 && num.starts_with("0"){
            return false;
        }
        if !num.parse::<i32>().map_or(false, |x| 0 <= x && x <= 255){
            return false;
        }
    }
    true
}

fn is_ipv6(s: &str) -> bool{
    let nums: Vec<&str> = s.split(':').collect();
    if nums.len() != 8{
        return false;
    }
    for num in nums{
        if num.len() > 4{
            return false;
        }
        if i32::from_str_radix(num, 16).is_err(){
            return false;
        }
    }
    true
}