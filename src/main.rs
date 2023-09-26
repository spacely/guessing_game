use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    println!("Please input your number guess");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is {}", secret_number);

    
     let mut guess = String::new();
     io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

     let guess: u32 = guess.trim().parse()
    .expect("Please enter a number");



    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big"),
        Ordering::Less => println!("Too small")
    }


}
