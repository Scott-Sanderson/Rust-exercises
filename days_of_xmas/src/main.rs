fn main() {
    println!("Lyrics to the 12 Days of Christmas!");

    let ordinals = ["first", "second", "third", "fourth", "fifth",
    "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = ["a partridge in a pear tree.", "two turtle doves, and", "three french hens",
    "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming",
    "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping",
    "twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me: {}", ordinals[i], lyrics[i]);

        let mut j: usize = i;
        loop {
            if j == 0 {
                break;
            } else {
                j -= 1;
                println!("\t{}", lyrics[j]);
            }
        }
       
    }
}
