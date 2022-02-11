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
    pub fn pop(&mut self) -> String {
        "test".to_string()
    }
    pub fn pop_cat(&mut self) -> String {
        //let t = self.cat[0]
        let mut answer = "NO".to_string();
        //if self.cat.len()>0 { 
        //    Animal::Cat(a) => 
        //}
        answer
    }
    pub fn pop_dog(&mut self) -> String {
        "test".to_string()
    }
}

/*pub fn Cat(name: &str) -> (String, String) {
    ("cat".to_string(), name)
}

pub fn Dog(name: &str) -> (String, String) {
    ("dog".to_string(), name)
}*/