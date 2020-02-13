fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("{}", numbers[2]);

    for n in &numbers {
        println!("{}", n);
    }
}