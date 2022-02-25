// cat_dog

mod lib;
use lib::{Database,Animal::*};

 // enum-iterator = "0.7.0"
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, IntoEnumIterator, PartialEq)]
enum Day { 
    Monday = 10, 
    Tuesday, 
    Wednesday, 
    Thursday, 
    Friday,                              
    Saturday, 
    Sunday,
}


fn main() {
    println!("ANIMAL:");
    let mut shelter = Database::default();
    shelter.push(Cat,"Barsik");
    shelter.push(Cat,"Mursik");
    shelter.push(Dog,"Mukhtar");
    println!("pop: {}",shelter.pop().unwrap());
    println!("cat: {}",shelter.pop_certain(Cat).unwrap());
    println!("dog: {}",shelter.pop_certain(Dog).unwrap());
    shelter.clear();

    println!("DAY:");
    let mut shelter_1 = Database::default();
    shelter_1.push(Day::Monday,"Barsik");
    shelter_1.push(Day::Monday,"Mursik");
    shelter_1.push(Day::Tuesday,"Mukhtar");
    println!("pop: {}",shelter_1.pop().unwrap());
    println!("mon: {}",shelter_1.pop_certain(Day::Monday).unwrap());
    println!("tue: {}",shelter_1.pop_certain(Day::Tuesday).unwrap());
     shelter_1.clear();

}


