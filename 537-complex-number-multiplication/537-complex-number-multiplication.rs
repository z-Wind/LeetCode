impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1 = get_complex_number(num1);
        let num2 = get_complex_number(num2);
        let ans = num1.mul(&num2);
        ans.to_string()
    }
}

struct Complex {
    real: i32,
    imag: i32,
}

impl Complex {
    fn mul(&self, x: &Complex) -> Complex {
        Complex {
            real: self.real * x.real - self.imag * x.imag,
            imag: self.real * x.imag + self.imag * x.real,
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}+{}i", self.real, self.imag)
    }
}

fn get_complex_number(num: String) -> Complex {
    let v:Vec<&str> = num.split("+").collect();
    let real = v[0].parse::<i32>().unwrap();
    let imag = v[1].replace("i","").parse::<i32>().unwrap();
    Complex{real, imag}
}