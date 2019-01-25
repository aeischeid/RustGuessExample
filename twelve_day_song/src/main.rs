fn main() {
    println!("OHHHHHH!");
    let ordinal_names = ["first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];

    let gifts = [
        "partidge in a pear tree",
        "turtle doves",
        "frech hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];
    // let mut (day_number, day_name, gift_type) = (0, "first", "bird in tree");

    for day_number in 1..=12 {
        let mut day_minus1 = day_number - 1;
        println!("on the {} day of Christmas my true love gave to me...", ordinal_names[day_minus1]);
        println!("{} {}", day_number, gifts[day_minus1]);
        while day_minus1 != 0 {
            println!("{} {}", day_minus1, gifts[day_minus1-1]);
            day_minus1 = day_minus1 - 1;
        }
        println!("---");
    }


}
