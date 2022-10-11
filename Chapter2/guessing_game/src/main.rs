use std::cmp::Ordering;
use rand::Rng;
use std::io;

//A Gateway from Ch3 to Ch6!

fn main() {
    //greet and ask the player
    println!("Howdy, welcome to the number guessing game!");

    //random number generator 
    //call method within thread_rng function
    let secret_number = rand::thread_rng().gen_range(1, 101);


    /*    to test    */
    //println!("The secret number: {}", secret_number);
    /* done testing */

    //counting guesses
    let mut score = 0;

    loop {

        //ask user for a number
        if score <1{
        println!("Now, guess a number?");
        }
        else{
            println!("Choose another.");
        }
        //huh?
        let mut guess = String::new();

        score = score + 1;

        //get input and output stream
        io::stdin()
            //what? //get user input io::stdin.read_line(&mut guess).expect("Failed to read line");
            //use String::new() instead of String::form
            .read_line(&mut guess)
            .expect("Failed to read line");



        //convert string into variable int
        //add type info (bytes)
        //.trim() takes out \n when compiled
        //Parse returns a Return type = return 0;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        //{} in ""?    
        println!("You guessed: {}", guess);


        //match the numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if score <=1{
                    println!("Right on the ba{}l!", score);
                }
                else{
                    println!("You have guessed {} times", score);
                }
                break;
            }
        }
    }
}


//def immutable == unchanging over time 


//fn main()
//function main block, () empty because no parameters to take



//huh? explanation
//std::cout... :: is use in c++ remember, but using std namespace eliminates the need to use it
//let mut stores user input
//let is used to create a variable 
//mutable = mut
//================== after variable name guess =======================
//:: static method showing that ::new is associated to the string type as a function
/*
"To summarize, the let mut guess = String::new(); 
line has created a mutable variable that is currently bound to a new, empty instance of a String"
 -Chapter 2, Rust Book
*/


//what? explanation 
//read_line method gets an input from the user
//read_line takes whatever the user types and puts in the string variable guess
//& indicates this arg is a reference 
//& is immutable so must have mut in order to change
//aka get user input which is bound to change
//that whole thing is one arg until end of ;
//expect catches errors a us of io::Results

//{} is in ""
//because it is a placeholder for a value so
//"yo {}, man {}, i got you",catchPhrase, name -- 
//means it will use the stored value in catchPhrase first then use name in the other


//use Ordering
//The first new bit here is another use statement 
//bringing a type called std::cmp::Ordering into scope from the standard library. Like Result, Ordering is another enum, but the variants for Ordering are Less, Greater, and Equal. 
//These are the three outcomes that are possible when you compare two values.




//Return to when more exp on error handling 
/*
Switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error. 
Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err. 
We’re using a match expression here, as we did with the Ordering result of the cmp method.

If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resulting number. 
That Ok value will match the first arm’s pattern, and the match expression will just return the num value that -
- parse produced and put inside the Ok value. 
That number will end up right where we want it in the new guess variable we’re creating.

If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error. 
The Err value does not match the Ok(num) pattern in the first match arm, 
but it does match the Err(_) pattern in the second arm. 
The underscore, _, is a catchall value; 
in this example, we’re saying we want to match all Err values, no matter what information they have inside them. 
So the program will execute the second arm’s code, continue, which tells the program to go to the next - 
- iteration of the loop and ask for another guess. 
So, effectively, the program ignores all errors that parse might encounter!
*/