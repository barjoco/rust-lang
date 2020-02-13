fn main() {
    // let my_vector: Vec<i8> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4, 5];

    println!("Vector at 2: {}", my_vector[2]);

    my_vector.push(6);
    my_vector.remove(2);

    for n in my_vector {
        println!("{}", n);
    }
}
