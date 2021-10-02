impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0{
            return String::from("Zero");
        }
        let mut ans = String::new();
        let mut digits = 0;
        while num != 0{
            let mut n = num%1000;
            if n != 0{  
                match digits{
                    0 => (),
                    3 => ans = number_to_word(1000) + " " + &ans,
                    6 => ans = number_to_word(1000_000) + " " + &ans,
                    9 => ans = number_to_word(1000_000_000) + " " + &ans,
                    _ => panic!("digits:{}", digits),
                }
                match_1_999_to_words(&mut ans,n);
            }
            // println!("{}:{:?}",n,ans);
            num /= 1000;
            digits += 3;
        }
        ans.trim_end().to_string()
    }
}

fn match_1_999_to_words(ans:&mut String, num: i32){
    match num{
        0 => (),
        1..=20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => *ans = number_to_word(num) + " " + &ans,
        21..=99 => {
            *ans = number_to_word(num%10) + " " + &ans;
            *ans = number_to_word(num/10 * 10) + " " + &ans;
        },
        100..=999 => {
            match_1_999_to_words(ans, num%100);
            *ans = number_to_word(100) + " " + &ans;
            *ans = number_to_word(num/100) + " " + &ans;
        }
        x => panic!("num:{}", x),
    }
}

fn number_to_word(num: i32) -> String {
    match num{
        1  => String::from("One"),
        2  => String::from("Two"),
        3  => String::from("Three"),
        4  => String::from("Four"),
        5  => String::from("Five"),
        6  => String::from("Six"),
        7  => String::from("Seven"),
        8  => String::from("Eight"),
        9  => String::from("Nine"),
        10 => String::from("Ten"),
        11 => String::from("Eleven"),
        12 => String::from("Twelve"),
        13 => String::from("Thirteen"),
        14 => String::from("Fourteen"),
        15 => String::from("Fifteen"),
        16 => String::from("Sixteen"),
        17 => String::from("Seventeen"),
        18 => String::from("Eighteen"),
        19 => String::from("Nineteen"),
        20 => String::from("Twenty"),
        30 => String::from("Thirty"),
        40 => String::from("Forty"),
        50 => String::from("Fifty"),
        60 => String::from("Sixty"),
        70 => String::from("Seventy"),
        80 => String::from("Eighty"),
        90 => String::from("Ninety"),
        100 => String::from("Hundred"),
        1000 => String::from("Thousand"),
        1000_000 => String::from("Million"),
        1000_000_000 => String::from("Billion"),
        x => panic!("{}", x),
    }
}