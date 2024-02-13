use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn game(){
    println!("Gusessing the number!");
    println!("Please enter number");

    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(0..=2);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("please enter a number!");
    // if guess == secret{
    //     println!("Your guess is correct!!");
    // }else{
    //     println!("Wrong guess!!");
    // }

    match guess.cmp(&secret){
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => println!("You Win!!")
    }

}