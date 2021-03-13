fn main() {
    let gift_list = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "Seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let ordinal_list = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut idx = 0;

    println!("Twelve Days of Christmas\n");
    while idx < gift_list.len() {
        let mut j = idx;

        println!(
            "On the {} day of christmas, my true love gave me",
            ordinal_list[idx]
        );

        if idx == 0 {
            print!("{}", gift_list[0]);
            idx += 1;
            continue;
        }

        while j > 0 {
            print!("{}, ", gift_list[j]);
            j -= 1;
        }

        if j == 0 {
            print!("and {}", gift_list[j]);
        }

        idx += 1;

        println!("\n");
    }
}

// loop 12 times
// print the first quote (first, second, third...)
// have a gift list
// select the N gift
