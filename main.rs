//use std::mem;

fn operators() {
    // arithmetic

    let mut a = 2+3*4; // +-*/
    println!("{}", a);
    a = a+1; // -- ++ don't support
    a -= 2; // a = a - 2
            // -=  += /= %=

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // OR & AND ^ XOR ! NOR
                   // 01 OR 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("{} == 5 resp: {}", x, x_is_5);
}

fn scope_and_shadowing() {
    
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);

    }

    println!("outside, a = {}", a);
}

fn main() {

    //operators();
    scope_and_shadowing();
}