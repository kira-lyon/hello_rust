use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let iter = &mut input.split_whitespace();
    let phrase = iter.next().unwrap_or_default();
    let mut count : usize = match iter.next().unwrap_or_default().trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if count > 10 {
        println!("Up to 10.")
    }
    else {
        while count > 0 {
            println!("{}", phrase);
            count-=1;
        }
    }
}
