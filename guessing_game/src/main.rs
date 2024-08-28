exrern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    //THIS LINE OF CODE GENERATES RANDOM NUMBER IN BETWEEN 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The Secret number is: {}", secret_number);

    println!("Enter your guess: ");
    //THIS LINE OF CODE CREATE A STRING VARIABLE
    let mut guess = String::new();
    //THIS LINE OF CODE TAKES INPUT FROM THE USER FOR THE VARIABLE
    io::stdin().read_line(&mut guess)
        .expect("Failed To Readline.")
    
    loop{
        //THIS LINE OF CODE CONVERS NUMBER STRING TO INTEGER
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => contine,
        };
        
        println!("your Guess is {}", guess);

        //THIS LINE OF CODE COMPARES THE SECRET NUMBER AND THE GUSSED NUMBER
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You guessed right.")
                break;
            }
        }
    }

}
