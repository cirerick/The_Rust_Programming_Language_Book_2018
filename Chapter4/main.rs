//Notes as illulstrated within the book

fn string_movement_example() {   
    //s is not valid here; it's not yet decalred
    
    let s = "hello"; //s is valid from this point forward
    
    //do stuff with s 
    
} //this scope is now over and s is no longer valid, call drop()

fn append_literal_example() {
    let mut s = String::from("hello");

    s.push_str(", world!"); //push_str() appends a literal to a String

    println("{}", s); //this will print 'hello, world!'
}

fn aloccation_example() {
    let s = String::from("hello"); //s is valid from this point forward
    
    //do stuff with s

} // this scope is now over and s is no longer valid

fn int_assignment_example() {
    let x = 5; //allocate new memory //assign 5 to it
    let y = x; //allocate new memory //get the value of x and assign it to y
}

fn move_string_example() {
    let s1 = String::from("hello"); //allocate new memory //contain pointer, length, and capacity. 
    let s2 = s1; //appoint to the same memory location as s1 //copy other attributes 
    
    println!("{}, world!", s1); //will result in error as the value had been moved
    //heap memory means waiting a and sitting people at these tables
}

fn deeply_copy_string_example() {
    let s1 = String::from("hello"); //allocate new memory 
    let s2 = s1.clone(); //copy heap data, but allocated new memory 

    println!("s1 = {}, s2 = {}", s1, s2); //s1 and s2 are seperate entites
}

fn integer_copy_example() {
    let x = 5; //allocate new memeory //assign 5 to x
    let y = x; //allocate new memory //make copy of x

    println!("x = {}, y = {}", x, y); //easy to copy as memory is stores entirely of on stack
    //stack memory means taking orders for one table
}

//======================= Ownership Rules =======================//

fn ownership_function_example() {
    let s = String::from("hello"); //s comes into scope

    take_ownership(s); //s's value moves into the function...
                                   //... and so is no longer valid here

    let x = 5; //x comes into scope

    makes_copy(x); //x would move into the function,
                                //but i32 is Copy, so it's okay to... 
                                //...still user x aftward - x is copied before committing function

}

fn take_ownership(some_string : String) { //some_string comes into scope
    println!("{}", some_string);
} //Here, some_string goes out of scope and 'drop' is called. The backing...
  //memory is freed.

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("{}", some_integer);
}//Here, some_integer goes out of scope. Nothing more, nothing less. 


//======================= Returning Vales from Functions =======================//

fn return_main_example() {
    let s1 = gives_ownership(); //gives_ownership moves its return...
                                        //... value to s1 

    let s2 = String::from("hello"); //s2 comes into scope

    let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back,
                                                    //which also moves its return into s3
} //s2 goes out of scope, but was moved to s3. s3 and s1 go out of scope and are dropped. 

fn gives_ownership() -> String { //gives_ownership will move its return value into the function
                                 //return value into the funtion that calls it

    let some_string = String::from("hello"); //some_string comes to scope

    some_string //some_string is returned and is moved to where the function is being called from

}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string : String) -> String { //a_string comes into scope

    a_string //a_string is returned and moves to where the function is being called from

}

//Returning multiple values using Tuple
fn return_multi_value_example(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); //declare tuple

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s : String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a String

    (s, length)
}

//======================= References and Borrowing =======================//

fn refernce_main_example(){
    let s1 = String::from("hello"); //s1 comes into scope

    let len = calculate_length_2nd(&s1); //get length value of s1
                                                //pass s1 as refernce to not invalidate s1

    println!("The length of {} is {}.", s1, len);
}

fn calculate_length_2nd(s : &String) -> usize {
    s.len() // return length of refernced String -essentially it's attributes-
}

//--------------------------------------------------------------------------
fn incorrect_mutating_immutable_refernce() {
    let s = String::from("hello");

    change(&s);

}

fn change(some_string : &String) {
    some_string.push_str(", world");
}

//-------------------------------------------------------------------------

fn correct_mutating_mutable_refernce() {
    let s = String::from("hello");

    change(&mut s);

}

fn correct_change(some_string : &mut String) {
    some_string.push_str(", world");
}

//Mutible References can only be mutated once per scope-------------------
//This will fail
fn double_mutation_of_reference() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
}


fn assigning_the_same_refernce_within_the_same_function() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }//goes out of scope here, so we can make a new refernce with no problems

    let r2 = &mut s;
}

fn incorrectly_from_refernce_to_mut_reference() {
    let mut s = String::from("hello");

    let r1 = &s; //no problem
    let r2 = &s; //no problem
    let r3 = &mut s; //BIG PROBLEM, refence is already immutable

    println!("{}, {}, and {}", r1, r2, r3);


}

//------------------- Dangling Reference ----------------------------------

fn incorrect_dangling_refernce_example() {
    let refernce_to_nothing = incorrect_dangle();

}

fn incorrect_dangle() -> &String { //will throw error stating missing lifetime parameter
    let s = String::from("hello"); // s is a new String

    &s //return refernce of s
} //s is deallocated (dropped), the refernce is referring to nothing basically

//The correcy way

fn correct_dangle() -> String { //get rid of refernces 
    let s = String::from("hello");

    s
} //Just normally pass the value without referencing? 


//==================== Slice Type ======================

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes_iter().enumrate() {
        if item == b' ' {
            return i;
        }
    }

    s.len();
}

fn main() {
    let mut s = String::from("");

    let mut word = first_word(&s); //word will get the value 5

}

s.clear(); //word empties the String, making it equal to **

//word still has value, there's no more string that
//we could meaningfully use the value 5 with. word is now that
//invalid


fn string_slices(){
    let s = String::from("");

    let hello = &s[0..5];
    let world = &s[6..11];
}
