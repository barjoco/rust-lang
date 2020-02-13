struct Rectangle(u16, u16);

impl Rectangle {
    fn print_details(&self) {
        println!("Dimensions: {} x {}", self.0, self.1);
    }
    fn is_square(&self) -> bool {
        return self.0 == self.1;
    }
}

fn main() {
    let rect = Rectangle(10, 5);

    rect.print_details();

    println!(
        "The rectangle is{} a square",
        if !rect.is_square() {
            " not"
        } else {
            ""
        }
    );
}
