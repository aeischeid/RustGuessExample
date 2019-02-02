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
    let firstLetter1 = s.trim_start().chars().next().unwrap();
    assert_eq!('E', firstLetter1);
    println!("first letting in LTR lang string {}", firstLetter1);

    // how the eff does it know directionality?!?!?
    let s = "  עברית  ";
    let firstLetter2 = s.trim_start().chars().next().unwrap();
    assert!('ע' == firstLetter2);
    println!("first letting in RTL lang string {}", firstLetter2);

    // println!("{:?}", );
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
