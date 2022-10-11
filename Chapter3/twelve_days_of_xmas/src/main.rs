use std::io;

//not optimized

fn validate(num_of_days : u8) -> bool {
    let mut num_validate : bool = false;
    let valid_num : [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    for num in valid_num{
        if num_of_days == num {
            num_validate = true;
            break;
        }
    }

    num_validate

}

fn num_of_days_signifier(num_of_days : u8) -> String {
    //store signifier and then return it
    let string_signifier; 

    match num_of_days {
        1 => string_signifier = String::from("first"), 
        2 => string_signifier = String::from("second"),
        3 => string_signifier = String::from("third"),
        4 => string_signifier = String::from("forth"),
        5 => string_signifier = String::from("fifth"),
        6 => string_signifier = String::from("sixth"),
        7 => string_signifier = String::from("seventh"),
        8 => string_signifier = String::from("eighth"),
        9 => string_signifier = String::from("ninth"),
        10 => string_signifier = String::from("tenth"),
        11 => string_signifier = String::from("eleventh"),
        12 => string_signifier = String::from("twelfth"),
        _=> string_signifier = String::from("null"),
    }

    string_signifier

}

fn sing_carol(num_of_days : u8) {

    println!("");

    println!("On the {} day of X-mas, my true love sent to me.", num_of_days_signifier(num_of_days));

    //first iteration checked
    if num_of_days == 1 {
        println!("A partridge in a pear tree"); 
    } else {
        //reverse order to match verses per number line
        for _i in (1..num_of_days + 1).rev() {
            match _i {
                12 => println!("Twelve drummers drumming"), 
                11 => println!("Eleven pipers piping"),
                10 => println!("Ten lords a-leaping"),
                9 => println!("Nine ladies dancing"),
                8 => println!("Eight maids a-milking"),
                7 => println!("Seven swans a-swimming"),
                6 => println!("Six geese a-laying"),
                5 => println!("Five gold rings (five golden rings)"),
                4 => println!("Four calling birds"),
                3 => println!("Three French hens"),
                2 => println!("Two turtledoves"),
                1 => println!("And a partridge in a pear tree"),
                _=> println!("null"),
            }
        }
    }

    if num_of_days == 12 {
        println!(" "); 
    }


}

fn main() {
    println!("How many days of X-mas would you like to carol for (Input 1-12)?");

    //store number of days to carol
    let mut num_of_days = String::new();

    //get number of days to carol     
    io::stdin().read_line(&mut num_of_days)
        .expect("Failed to read line.");
    let num_of_days : u8 = num_of_days.trim().parse()
        .expect("Please enter a positive whole number.");
    
    if validate(num_of_days) {
        for _i in 1..num_of_days + 1{
            sing_carol(_i);
        }
    } else {
        println!("Not valid my friend.");
    }
    
    
}
