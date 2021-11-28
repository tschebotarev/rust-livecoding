use lazy_static::lazy_static;
use dashmap::DashMap;

lazy_static! {
    static ref ANALYZING_COLLECTION: DashMap<&'static str, i32> = DashMap::new();
}

fn main() {
    ANALYZING_COLLECTION.insert("123", 456);
    ANALYZING_COLLECTION.insert("1234", 4567);
    //analyzing_collection.remove("1234");

    let t = *ANALYZING_COLLECTION.get("123").unwrap();
    println!("{:?}", t);

    let test_optional = ANALYZING_COLLECTION.get("1235");
    if let Some(test1) = test_optional { 
        let test = *test1;
        //println!("!");
    }
}