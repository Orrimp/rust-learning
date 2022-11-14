#![allow(dead_code)]

// C Struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}
struct Unit;
struct Pair(String, u8);

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 }
}

#[derive(Debug)]
enum Color { 
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
    Yellow = 0xffff00,
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed {} ", c),
        WebEvent::Paste(string) => println!("Pasted something {} ", string),
        WebEvent::Click { x,y } => { 
            println!("Cliced on coordinates {},{}", x,y);
        },
    }
}

fn main(){

    use crate::Color::{Red,Blue}; //alternative crate::Color::*;

    let name1 = String::from("Vitaliy Schreibmann");
    let name2 = String::from("Belinda Schreibmann");
    let age = 35;

    let _person1 = Person { name: name1, age: age } ;
    println!("Struct {:?}", _person1);
    let _person2 = Person { name: name2, .._person1 };
    println!("Updated Struct {:?}", _person2);
    
    //Enum
    let pressed = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("Some String".to_owned());
    let click = WebEvent::Click{ x: 30, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(paste);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("{:?}", Red);
    println!("{:?}", Blue);
    println!("{:?}", Color::Yellow);

    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    mutable_binding += 1;

}
