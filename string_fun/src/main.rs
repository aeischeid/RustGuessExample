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


    let s = "  English  ";
    
    let first_letter1 = s.trim_start().chars().next();
    match first_letter1 {
        Some(str) => {
            assert_eq!('E', str);
            println!("first letting in LTR lang string {}", str);
        },
        _ => (),
    }

    // how the eff does it know directionality?!?!?
    let s = "  עברית  ";
    let first_letter2 = s.trim_start().chars().next().unwrap();
    assert!('ע' == first_letter2);
    println!("first letting in RTL lang string {}", first_letter2);

    // println!("{:?}", );
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
