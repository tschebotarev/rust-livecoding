pub enum Animal {
    Dog(&'static str),
    Cat(&'static str),
}

struct MyData {
    name: String,
    age: u32,
}


impl Default for MyData {
    fn default() -> Self {
        let name = "".to_string();
        let age = 0u32;
        Self {
            name,
            age,
        }
    }
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

fn add_data(name: &str, n: u32) -> MyData {
    let mut add = MyData::default();
    add.name = name.to_string();
    add.age = n;
    add
}

impl Database {
    pub fn push(&mut self, data: Animal) {
        self.counter+=1;
        match data {
            Animal::Cat(a) => self.cat.push(add_data(a,self.counter)),
            Animal::Dog(a) => self.dog.push(add_data(a,self.counter)),
        }
    }
    pub fn pop(&mut self) -> Option<String> {
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
    pub fn pop_cat(&mut self) -> Option<String> {
        if self.cat.len()>0 { 
            let t = self.cat[0].name.to_string();
            self.cat.remove(0);
            return Some(t);
        }
        else {
            return Option::None;
        }
    }
    pub fn pop_dog(&mut self) -> Option<String> {
        if self.dog.len()>0 { 
            let t = self.dog[0].name.to_string();
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