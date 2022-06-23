fn main() {
    let gifts = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh","twelfth"];

    for i in 0..days.len() {
        println!("On the {} day of Christmas\nMy true love sent to me", days[i]);
        
        println!("{}", gifts[i]);
        
        for index in (0..i).rev()  {
            println!("{}", gifts[index]);
        }
        println!("\n\n");
    }
}
