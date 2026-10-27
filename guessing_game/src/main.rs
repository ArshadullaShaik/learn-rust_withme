use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main(){
    
    

    let secretnum = rand::rng().random_range(1..100);
loop{let mut guess = String::new();
    println!("enter your guess number");
    io::stdin()
    .read_line(&mut guess)
    .expect("error reading message");
let guess:u32 = match guess.trim().parse() {
    Ok(num)=>num,
    Err(_)=>continue,
};
    println!("you have guessed: {guess}");

    match guess.cmp(&secretnum){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {println!("u got it");break;}
    }


}
}