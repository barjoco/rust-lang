fn main() {
    print_numbers_to(255);
    println!("Is 5 an even number? {}", is_even(5));
}

fn print_numbers_to(to_num: u8) {
    for num in 1..to_num {
        println!("{}", num);
    }
}

fn is_even(num: u8) -> bool {
    return num % 2 == 0;
}