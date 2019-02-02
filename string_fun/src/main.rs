fn main() {

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("Hello, {}!", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("format is just Rust's string concatenator");
    println!("similar to groovy gString, \"foo $var thing\" or js strings with `idont $know etc.` {}", s);


    // println!("{:?}", );
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
