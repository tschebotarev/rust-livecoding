
use enum_iterator::IntoEnumIterator; // enum-iterator = "0.7.0"

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, IntoEnumIterator)]
pub enum Animal {
    // Dog(String),
    // Cat(String),
    Dog,
    Cat,
}

pub enum Animal2 {
    Pig,
    Frog,
    Snake,
}

// use std::fmt::{Display, Formatter};
// impl Display for Animal {
//     #[rustfmt::skip]
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}",self.get_name())
//     }
// }

// impl Animal {
//     fn get_name(&self) -> &str {
//         match self {
//             Animal::Cat(a) => a,
//             Animal::Dog(a) => a,
//         }
//     }
// }

#[derive(Clone)]
struct MyData {
    name: String,
    //name: T,
    age: u32,
}

pub struct Database<T> {
    name_list: Vec<T>,
    data_list: Vec<Vec<MyData>>,
    counter: u32,
}

impl<T> Default for Database<T> 
where T: IntoEnumIterator + PartialEq + Clone
{
    fn default() -> Self {
        let name_list: Vec<T> = T::into_enum_iter().collect();
        let data_list: Vec<Vec<MyData>> = vec![vec![];T::VARIANT_COUNT]; 
        let counter = 0u32;
        Self {
            name_list,
            data_list,
            counter,
        }
    }
}

impl<T> Database<T> 
where T: PartialEq
{
    pub fn push(&mut self, data: T, name: &str) {
        self.counter+=1;
        
        for i in 0..self.name_list.len() {
            if self.name_list[i]==data {
                self.data_list[i].push(MyData{name:name.to_string(), age:self.counter});
                return;
            }
        } 
        // match data {
        //     Animal::Cat(a) => self.cat.push(MyData{name:Animal::Cat(a), age:self.counter}),
        //     Animal::Dog(a) => self.dog.push(MyData{name:Animal::Cat(a), age:self.counter}),
        // }
    }
    pub fn pop(&mut self) -> Option<String> {
        let mut age = u32::MAX;
        for i in 0..self.name_list.len() {
            if self.data_list[i].len()>0 {
                if self.data_list[i][0].age<age {
                    age = self.data_list[i][0].age;
                }
            }
        } 
        if age<u32::MAX {
            for i in 0..self.name_list.len() {
                if self.data_list[i][0].age==age {
                    let t = self.data_list[i][0].name.to_string();
                    self.data_list[i].remove(0);
                    return Some(t);
                }
            }
        }
        return Option::None;
    }
    pub fn pop_certain(&mut self, data: T) -> Option<String> {
        for i in 0..self.name_list.len() {
            if self.name_list[i]==data {
                if self.data_list[i].len()>0 {
                    let t = self.data_list[i][0].name.to_string();
                    self.data_list[i].remove(0);
                    return Some(t);
                }
                else {
                    return Option::None;
                }
            }
        } 
        return Option::None;
    }
    pub fn clear(&mut self) {
        //self.data_list.clear();
        for _i in 0..self.data_list.len() {
            self.data_list.clear();
        }
        self.counter = 0;
    }
}