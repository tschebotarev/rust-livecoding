// cat_dog

mod lib;
use lib::{Database,Animal::*};
use lib::*;

use enum_iterator::IntoEnumIterator; // enum-iterator = "0.7.0"
#[derive(Debug, IntoEnumIterator, PartialEq)]
enum Day { 
    Monday = 1, 
    Tuesday, 
    Wednesday, 
    Thursday, 
    Friday, 
    Saturday, 
    Sunday,
}


fn main() {
    let mut shelter = Database::default();
    shelter.push(Cat("Barsik"));
    shelter.push(Cat("Mursik"));
    shelter.push(Dog("Mukhtar"));
    println!("pop: {}",shelter.pop().unwrap());
    println!("cat: {}",shelter.pop_cat().unwrap());
    println!("dog: {}",shelter.pop_dog().unwrap());
    shelter.clear();

    let t:Vec<Day> = Day::into_enum_iter().collect();
    let e = Day;

    if t[0]==Day::Monday {
        println!("!");
    }

    for i in t {
        println!("{:?}",i);
    }

    println!("{:?}",Day::into_enum_iter().next().unwrap());
    println!("{:?}",Day::VARIANT_COUNT);
    println!("{:?}",Day::into_enum_iter().last().unwrap());
}


