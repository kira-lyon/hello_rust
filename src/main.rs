use std::io;

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let split: usize = first_word(&input);
    let phrase = &input[..split];

    let mut count: usize = match (&input[split..]).trim().parse() {
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
