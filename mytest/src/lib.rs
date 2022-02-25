
use enum_iterator::IntoEnumIterator; // enum-iterator = "0.7.0"

#[derive(Clone, Hash, PartialEq, Eq, Debug, IntoEnumIterator)]
pub enum Animal {
    Dog(String),
    Cat(String),
}

pub enum Animal2 {
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

struct MyData<T> {
    name: T,
    //name: T,
    age: u32,
}

pub struct Database<T> {
    name_list: Vec<T>,
    data_list: Vec<Vec<MyData<T>>>,
    counter: u32,
}

impl<T> Default for Database<T> 
where T: IntoEnumIterator + PartialEq
{
    fn default() -> Self {
        let name_list: Vec<T> = T::into_enum_iter().collect();
        let data_list: Vec<Vec<MyData<T>>> = vec![];
        let counter = 0u32;
        Self {
            name_list,
            data_list,
            counter,
        }
    }
}

impl<T> Database<T> {
    pub fn push(&mut self, data: T) {
        self.counter+=1;
        for i in 0..self.name_list.len() {
            //if self.name_list[i]==data {

            //}
        } 
        /*match data {
            Animal::Cat(a) => self.cat.push(MyData{name:Animal::Cat(a), age:self.counter}),
            Animal::Dog(a) => self.dog.push(MyData{name:Animal::Cat(a), age:self.counter}),
        }*/
    }
    /*pub fn pop(&mut self) -> Option<Animal> {
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
    }*/
    pub fn clear(&mut self) {
        self.data_list.clear();
        self.counter = 0;
    }
}