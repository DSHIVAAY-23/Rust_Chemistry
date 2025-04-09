// Parent Trait
trait Animal {
    fn eat(&self);
    fn sleep(&self);
}

// Subclass 1
struct Cat;
impl Animal for Cat {
    fn eat(&self) {
        println!("Cat is eating.");
    }
    fn sleep(&self) {
        println!("Cat is sleeping.");
    }
}

// Subclass 2
struct Dog;
impl Animal for Dog {
    fn eat(&self) {
        println!("Dog is eating.");
    }
    fn sleep(&self) {
        println!("Dog is sleeping.");
    }
}
