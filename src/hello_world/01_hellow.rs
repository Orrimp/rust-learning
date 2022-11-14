use std::fmt;

//#[derive(Debug)]
struct MyStructure(i32, String);

impl fmt::Display for MyStructure {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{text} with {number}", number=self.0, text=self.1)
    }
}

impl fmt::Debug for MyStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug MyStrcutre with following data {} {}", self.0, self.1)
    }
}

struct MyList(Vec<i32>);

impl fmt::Display for MyList {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec_start = &self.0;

        write!(f, "[")?;

        for (count, value) in vec_start.iter().enumerate() {
            if count != 0 { 
                write!(f, ",  ")? 
            }
            write!(f, "{} ", value)?;
            
        }
        write!(f, "]")
    }

}



fn main(){
    let x = 5 + 8 * 3;
    let y = 10;
    println!("Hello World with `x` = {} and `y` = {} ", x,y);
    println!("Hello Reverse World with `x` = {1} and `y` = {0} ", x,y);
    println!("Hello Named World with `x` = {x} and `y` = {y} ", x=x,y=y);
    println!("Debug Text x = {x:?}", x=(x*y));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("Print Floating point {number:>width$}");
    println!("Print Floating point {number:.3}");


    let hello_world: String =  String::from("Hello World");
    let somestruct:MyStructure = MyStructure(42, hello_world);

    //Struct Formatter

    println!("My Structure formatter {} ", somestruct);
    println!("Debug Structure formatter {:?}", somestruct);

    //List Formatter
    let vector = MyList(vec![1,2,3,]);
    println!("{}", vector);

}


// //Output:
// Hello World with `x` = 29 and `y` = 10 
// Hello Reverse World with `x` = 10 and `y` = 29 
// Hello Named World with `x` = 29 and `y` = 10 
// Debug Text x = 290
// Print Floating point     1
// Print Floating point 1.000
// My Structure formatter Hello World with 42 
// Debug Structure formatter Debug MyStrcutre with following data 42 Hello World
// [1 ,  2 ,  3 ]
