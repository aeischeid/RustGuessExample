use std::io;

fn main() {
    println!("input a temp ( in °F ) to convert ( to °C )");
    println!("---------------------------");

    let mut got_a_temp = false;
    let mut start_temp:f32 = 100.0;

    while !got_a_temp {
        let mut temp_to_convert = String::new();
        io::stdin().read_line(&mut temp_to_convert)
            .expect("Failed to read line");

        start_temp = match temp_to_convert.trim().parse() {
            Ok(num) => {
                got_a_temp = true;
                num
            },
            Err(_) => {
                println!("stop goofing - how aboout a number now?!");
                continue;
            },
        };
    }
    convert_temp(start_temp);
}

fn convert_temp(start_temp: f32) {
    let abs_zero = -459.67;
    if start_temp < abs_zero {
        println!("Absolute zero is -459.67°F and -273.15°C");
        println!("Anything less than that is off the scale and has no meaningful conversion");
    } else {
        let converted_temp = (start_temp - 32.0)/1.800;
        println!("converts to {}", converted_temp);
    }
}
