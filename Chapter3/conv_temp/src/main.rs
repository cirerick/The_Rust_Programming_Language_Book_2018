//get std io lib
// Known issues - while in terminal will take non-char keystrokes as input and read them into string
use std::io;

//function to remove whitespaces
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn validate_temp_char(temp_char : char) -> bool {
    
    //store if valid
    let mut valid : bool = false;
    
    //char to validate
    let valid_char: [char; 4] = ['f','F','c','C'];

    //check if it matches one of them
    for v_char in valid_char.iter() {
        if v_char == &temp_char {
            valid = true;
            break;
        }
    }

    valid

}

fn validate_temp_digits(temp_digit_string : &String, counter : usize, len : usize) -> bool {
    //obligitory bool
    let mut valid : bool = false;

    //store if string is valid or not
    let mut _n : usize = counter;
    let valid_num: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    //iterate through each char in string to validate, if not valid
    let mut t : u8 = 0; 
    for &num in &valid_num{
        if &num == &temp_digit_string.chars().nth(counter.try_into().unwrap()).unwrap() && t == 0 {
            valid = true;
            t = 1;
        }
    }


    if valid && _n < len - 1 {
        return validate_temp_digits(&temp_digit_string, _n + 1, len);
    }
 
    valid

}



fn validate(passed_temp : String, temp_signifier : &mut char, digits : &mut f32) -> bool {
    //store temp_to_validate String
    let mut temp_to_validate = passed_temp;

    //remove all white spacing
    remove_whitespace(&mut temp_to_validate);

    //Innocent until proven guilty
    let mut valid : bool = true;
    //store last char to test if c/C for celsius/ f/F for Fahrenheit
    //also removes last char - leaving supposed digits 
    let temp_char : char = temp_to_validate.pop().unwrap();

    //calculate len and store reference
    let len = &temp_to_validate.chars().count();

    if !validate_temp_char(temp_char) || !validate_temp_digits(&temp_to_validate, 0, *len) {
        valid = false;
    } else {
        *temp_signifier = temp_char;
        *digits = temp_to_validate.parse::<f32>().unwrap();
    }
    

    //retrun boolean
    
    valid   

}

fn convert(digits : f32, temp_signifier : char) -> String {
    //String it will store the converted piece
    let converted_temp_string;
    //Store converted text
    let mut temp_digit_results : f32;

    //(F − 32) × 5/9 = C
    //(C(5)/9) + 32 = F
    if temp_signifier == 'f' || temp_signifier == 'F' {
        temp_digit_results = digits - 32.0;
        temp_digit_results = temp_digit_results * (5.0/9.0);
        converted_temp_string =  temp_digit_results.to_string() + &String::from("° C");
    } else if temp_signifier == 'c' || temp_signifier == 'C' {
        temp_digit_results = (digits * 9.0) / 5.0;
        temp_digit_results = temp_digit_results + 32.0;
        converted_temp_string =  temp_digit_results.to_string() + &String::from("° F");
    } else {
        converted_temp_string = String::from("Invalid temp signifier.");
    }

    converted_temp_string

}

fn main() {
    //We need to variables to store F and C
    //Ask user which they would like to input string
    //Determine which user would like to convert to seperate string to int and letter
    //Proceed with conversion
    //To store temp char and digit
    let mut digits : f32 = 0.0;
    let mut temp_signifier : char = '?'; 

    println!("Put in a temperature followed by the letter F or C."); 

    //get user input. 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{}",input);

    if validate(input, &mut temp_signifier, &mut digits) {
        println!("Temp you have entered: {}{}", digits, temp_signifier);
        println!("Temp converted: {}", convert(digits, temp_signifier));
    } else {
        println!("Not valid.");
    }


}
