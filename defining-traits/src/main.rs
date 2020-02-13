struct Person {
    name: &'static str,
    age: u8,
}

impl Person {
    fn print_details(&self) {
        println!("\nName: {}\nAge: {}\nCan speak: {}", self.name, self.age, self.can_speak());
    }
}

trait VoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl VoiceBox for Person {
    fn speak(&self) {
        println!("Hello my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        return self.age > 4;
    }
}

fn main() {
    let adam = Person {
        name: "Adam",
        age: 19,
    };

    adam.print_details();
    adam.speak();
}
