use std::convert::From;
use std::convert::TryFrom;


#[derive(Debug)]
struct SuperNumber {
    value: i32,
}

impl From<i32> for SuperNumber {
    fn from(item: i32) -> Self {
        SuperNumber { value: item }
    }
}

impl TryFrom<&str> for SuperNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = value.parse::<i32>();
        Ok(SuperNumber{value: result.unwrap()})
    }
}

fn main(){
    let decimal: f32 = 65.43221;
    let integer: u8 = decimal as u8;

    println!("Decimal {} to Integer {} ", decimal, integer);
    println!("Java devide error: 1/2 as u8 integers: {}", 1_u8/2_u8);
    println!("Java devide error: 1/2 as i32 integers: {}", 1i32/2i32);
    println!("Java devide error: 1/2 as f32 integers: {}", 1f32/2f32);
    println!("Not a Number {}", f32::NAN);

    println!("Size of bool is {} bytes", std::mem::size_of_val(&true));
    println!("Size of u8 is {} bytes", std::mem::size_of_val(&1_u8));
    println!("Size of i32 is {} bytes", std::mem::size_of_val(&1_i32));
    println!("Size of f32 is {} bytes", std::mem::size_of_val(&1_f32));
    println!("Size of f64 is {} bytes", std::mem::size_of_val(&1_f64));

    //Conversions
    //::from
    //::into
    let _num: SuperNumber = SuperNumber::from(50);
    println!("My number: {:?}", _num);
    let _num2:SuperNumber = 5.into();

    let _num3 : Result<SuperNumber, _> = SuperNumber::try_from("Hello3");
    println!("My number: {:?}", _num3);
    
    let _num4 : Result<SuperNumber, _> = SuperNumber::try_from("Hello World");
    println!("My number: {:?}", _num4)
}