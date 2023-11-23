

fn main() {
    let num = Calc::new(10);

    num.add(10)
        .sub(5)
        .mul(2)
        .div(3); // expect ((((10 + 10) - 5) * 2)  / 3) = 10

    println!("Number: {:?}", &num);
}

#[derive(Debug, Copy, Clone)]
struct Calc {
    number: i32,
}


impl Calc {
    pub fn new(num: i32) -> Self {
        Self {
            number: num
        }
    }

    fn add(mut self, num: i32) -> Self {
        self.number += num;
        self
    }

    fn sub(mut self, num: i32) -> Self {
        self.number -= num;
        self
    }

    fn mul(mut self, num: i32) -> Self {
        self.number *= num;
        self
    }

    fn div(mut self, num: i32) -> Self {
        self.number /= num;
        self
    }

}