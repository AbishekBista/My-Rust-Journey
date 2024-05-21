fn main() {
    let cardinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let items = ["A partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    for num in 0..12 {
        println!("On the {} day of Christmas,", cardinals[num]);
        println!("my true lover gave to me");

        for item_num in (0..num+1).rev() {
            println!("{}", items[item_num]);
        }

        println!("\n");
    }
}