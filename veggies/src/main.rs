mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


#![allow(unused_variables)]
fn main() {
    // use std::fmt;
    use std::fmt::Result;
    // use std::io::Result ;
    use std::io as IoResult;

    fn function1() -> Result {
        Ok(())
    }
    fn function2() -> IoResult<()> {
        Ok(())
    }



    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
