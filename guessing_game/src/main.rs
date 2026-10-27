use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main(){
    let mut guess = String::new();
    println!("enter your guess number");
    io::stdin()
    .read_line(&mut guess)
    .expect("error reading message");

    println!("you have guessed: {guess}");

    let secretnum = rand::rng().random_range(1..100);

    let guess:u32 = guess.trim().parse().expect("there was a error pls enter a number");

    match guess.cmp(&secretnum){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("u got it"),
    }



}