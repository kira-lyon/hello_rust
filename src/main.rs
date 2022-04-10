use std::env;

fn main() {
    let phrase = env::args().nth(1).expect("No phrase given");
    let number = env::args().nth(2).expect("No count given");

    let mut count : usize = match (number).trim().parse() {
        Ok(num) if num > 10 => 0,
        Ok(num) => num,
        Err(_) => 0,
    };

    while count > 0 {
        println!("{}", phrase);
        count-=1;
    }
}
