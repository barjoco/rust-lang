fn main() {
    let tup1 = (12, 3.4, "My string", false, (5, 6, 7));

    println!("{}", tup1.2);
    println!("{}", (tup1.4).1);

    let (a, b, c, d, e) = tup1;

    println!("{}", c);
}