// Polymorphic Trait
trait MakeSound {
    fn make_sound(&self);
}

// Subclass 1
struct Cat;
impl MakeSound for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

// Subclass 2
struct Dog;
impl MakeSound for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// Function to demonstrate polymorphism
fn make_animal_sound(animal: &dyn MakeSound) {
    animal.make_sound();
}
