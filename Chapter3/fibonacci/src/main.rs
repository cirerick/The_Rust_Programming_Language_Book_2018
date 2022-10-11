use std::io;

fn number_signifier(nth : u32) -> String{
    let nth_last_digit_to_char = nth.to_string().pop().unwrap();
    let nth_signifier;

    match nth_last_digit_to_char{
        '1'=>nth_signifier = String::from("st"),
        '2'=>nth_signifier = String::from("nd"),
        '3'=>nth_signifier = String::from("rd"),
        _=>nth_signifier = String::from("th"), 
    }

    nth_signifier
    
}

fn fib<'a>(mut n_1 : &'a mut u32, mut n_2 : &'a mut u32) -> u32  {
    let result_n : u32 = *n_1 + *n_2;
    *n_1 = *n_2;
    *n_2 = result_n;    

    result_n
}


fn main() {

    /*
        fib means this f(n) = f_(n-2) + f_(n-1) - n being the previous motion, we know the 9th fib = 34
        n1 = 0
        n2 = 0 + 1 = 1
        n3 = 1 + 1 = 2
        n4 = 1 + 2 = 3
        n5 = 2 + 3 = 5
        n6 = 3 + 5 = 8
        n7 = 5 + 8 = 13
        n8 = 8 + 13 = 21
        n9 = 13 + 21 = 34
    */

    //store the nth fibonacci
    let mut nth = String::new();
    //get what will be added
    let mut n_1 : u32 = 0; 
    let mut n_2 : u32 = 1;
    
    println!("Enter the nth most fibonacci number you would like to see");
    //get user input
    io::stdin().read_line(&mut nth)
        .expect("Failed to read line.");

    println!(" ");

    let nth : u32 = nth.trim().parse()
        .expect("Please enter a whole number.");

    print!("Fibonacci Results: 0,");
    for _i in 1..nth{
        if _i != nth - 1 {
            if _i != nth - 2{  
                print!(" {},", fib(&mut n_1,&mut n_2));
            } else {
                print!(" {}", fib(&mut n_1,&mut n_2));
            }
        } else {
            println!(""); 
            println!("The {}{} = {}"
                ,nth, number_signifier(nth), fib(&mut n_1,&mut n_2));
        }
    }




}
