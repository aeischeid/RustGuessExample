fn main() {

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("Hello, {}!", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
