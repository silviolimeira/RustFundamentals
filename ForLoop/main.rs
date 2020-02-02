fn for_loop()
{
    for x in 1..11 {

        if x == 3 {
            println!("x = **{}**", x);
            continue;
        }

        if x == 8 {
            println!("x = **{}** force break", x);
            break;
        }

        println!("x = {}", x);

    }


    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn main() {
    for_loop();

}