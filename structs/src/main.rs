struct Colour(u8, u8, u8);

struct Colour_with_identifiers{
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Colour(255, 255, 255);

    bg.1 = 0;

    println!("{}", bg.1); 
}
