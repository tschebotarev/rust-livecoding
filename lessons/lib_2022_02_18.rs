// cat_dog

/*
// main

mod lib;
use lib::{Database,Animal::*};

fn main() {
    let mut shelter = Database::default();
    shelter.push(Cat("Barsik"));
    shelter.push(Cat("Mursik"));
    shelter.push(Dog("Mukhtar"));
    println!("pop: {}",shelter.pop().unwrap());
    println!("cat: {}",shelter.pop_cat().unwrap());
    println!("dog: {}",shelter.pop_dog().unwrap());
    shelter.clear();
}
*/

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Animal {
    Dog(&'static str),
    Cat(&'static str),
}

use std::fmt::{Display, Formatter};
impl Display for Animal {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.get_name())
    }
}

impl Animal {
    fn get_name(&self) -> &str {
        match self {
            Animal::Cat(a) => a,
            Animal::Dog(a) => a,
        }
    }
}

struct MyData {
    name: Animal,
    age: u32,
}

pub struct Database {
    cat: Vec<MyData>,
    dog: Vec<MyData>,
    counter: u32,
}

impl Default for Database {
    fn default() -> Self {
        let cat:Vec<MyData> = vec![];
        let dog:Vec<MyData> = vec![];
        let counter = 0u32;
        Self {
            cat,
            dog,
            counter,
        }
    }
}

impl Database {
    pub fn push(&mut self, data: Animal) {
        self.counter+=1;
        match data {
            Animal::Cat(a) => self.cat.push(MyData{name:Animal::Cat(a), age:self.counter}),
            Animal::Dog(a) => self.dog.push(MyData{name:Animal::Cat(a), age:self.counter}),
        }
    }
    pub fn pop(&mut self) -> Option<Animal> {
        let test_cat = self.cat.len()>0;
        let test_dog = self.dog.len()>0;
        if test_cat && test_dog {
            if self.cat[0].age<self.dog[0].age { 
                return self.pop_cat(); 
            }
            else { 
                return self.pop_dog(); 
            }
        }
        else if test_cat { 
            return self.pop_cat();
        }
        else { 
            return self.pop_dog();
        }
    }
    pub fn pop_cat(&mut self) -> Option<Animal> {
        if self.cat.len()>0 { 
            let t = self.cat[0].name;
            self.cat.remove(0);
            return Some(t);
        }
        else {
            return Option::None;
        }
    }
    pub fn pop_dog(&mut self) -> Option<Animal> {
        if self.dog.len()>0 { 
            let t = self.dog[0].name;
            self.dog.remove(0);
            return Some(t);
        }
        else {
            return Option::None;
        }
    }
    pub fn clear(&mut self) {
        self.cat.clear();
        self.dog.clear();
        self.counter = 0;
    }
}