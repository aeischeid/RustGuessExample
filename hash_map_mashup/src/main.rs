fn main() {
    println!("hey-o map-os!");
    
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("{}", scores.get(String::from("Yellow")) );
}
