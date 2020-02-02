#![allow(dead_code)]
#![allow(unused_variables)]
// mod sh;
use std::mem;

enum Color { 
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),  // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct
}

fn enums() {
    // let c:Color = Color::Blue;
    //let c:Color = Color::RgbColor(0,0,0); // black
    //let c:Color = Color::RgbColor(10,0,0);
    
    //let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 33, black: 0};
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 33, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) 
        | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        _ => println!("some other color") // or  '==> ()' to do anything
    }
}

fn main() {
    enums();
}
