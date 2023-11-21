fn main() {

    let mut num = Number { get_number: 10};

    num.add(10).sub(5); // expect 15;
    num.print();


}


struct Number {
    get_number: i32,
}
trait Calc {

    fn add(&mut self, num: i32) -> self;
    fn sub(&mut self, num: i32) -> self;
    fn mul(&mut self, num: i32) -> self;
    fn div(&mut self, num: i32) -> self;

    fn print(&self);
}


impl Calc for Number {

    fn add(&mut self, num: i32) -> self {

        let temp = self.get_number + num;
        self { get_number : temp }
    }

    fn sub(&mut self, num: i32) -> self {

        let temp = self.get_number - num;
        self { get_number : temp }
    }

    fn mul(&mut self, num: i32) -> self{

        let temp = self.get_number * num;
        self { get_number : temp }
    }

    fn div(&mut self, num: i32) -> self {
        let temp = self.get_number / num;
        self { get_number : temp }
    }

    fn print(&self){
        println!("{:?}", self.get_number);
    }
}