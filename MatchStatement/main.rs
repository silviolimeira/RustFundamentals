fn match_statement()
{
    let country_code = 7777; // 1 999

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknow",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    match_statement();
}