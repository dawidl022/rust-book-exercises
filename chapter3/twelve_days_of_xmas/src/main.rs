fn main() {
    for i in 0..12 {
        print_verse(i);
    }
}

fn print_verse(day: usize) {
    println!("On the {} day of Christmas", CARDINALS[day]);
    println!("My true love sent to me");

    for i in (1..=day).rev() {
        println!("{}", GIFTS[i])
    }

    if day == 0 {
        println!("A {}", GIFTS[0]);
    } else {
        println!("And a {}", GIFTS[0])
    }

    println!();
}

const CARDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
