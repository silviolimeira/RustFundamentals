use std::mem;

fn operators() {
    // arithmetic

    let mut a = 2+3*4; // +-*/
    println!("{}", a);
    a = a+1; // -- ++ don't support
    a -= 2; // a = a - 2
            // -=  += /= %=

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    
}

fn main() {
    // Fundamental Data Types
    let a:u8 = 123;
    println!("a = {}", a);

    // a = 456;

    // mutable
    let mut b:i8 = 0; // mutable

    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789;  // 32-bit signed 32 i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // izeze/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", 
        z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // raw numbers

    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
    let e1:f32 = 2.5;  // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e1, mem::size_of_val(&e1));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4>0; // true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));


}