// cat_dog

mod lib;
use lib::{Database,Animal::*};

fn main() {
    let mut shelter = Database::default();
    shelter.push(Cat("Barsik"));
    shelter.push(Cat("Mursik"));
    shelter.push(Dog("Mukhtar"));
    println!("pop: {}",shelter.pop());
    println!("cat: {}",shelter.pop_cat());
    println!("dog: {}",shelter.pop_dog());
}


