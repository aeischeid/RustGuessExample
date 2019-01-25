fn main() {
    println!("Fibonacci numbers are named after Italian mathematician Leonardo of Pisa, later known as Fibonacci");

    println!("here they come ...");
    let mut prev_num1:u128 = 0;
    let mut prev_num2:u128 = 1;
    println!("{}", prev_num1);
    println!("{}", prev_num2);
    for _number in 1..160 {
        let new_number = prev_num1 + prev_num2;
        println!("{}", new_number);
        prev_num1 = prev_num2;
        prev_num2 = new_number;
    }

    println!("----- ");
    println!("that's probably enough for now, these numbers are just getting too big now.", );
}
