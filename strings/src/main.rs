fn main() {
    let my_string = "A quick brown fox jumps over the lazy dog";

    println!("Length: {}", my_string.len());
    println!("Empty? {}", my_string.is_empty());
    let my_string_split = my_string.split_at(my_string.len()/2);
    println!("Split in the middle: {}||{}", my_string_split.0, my_string_split.1);
    println!("Does the string contain 'dog'? {}", my_string.contains("dog"));
    let my_string_split_ws = my_string.split(" ");
    for w in my_string_split_ws { println!("{}", w); }
}
