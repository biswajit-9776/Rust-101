extern crate rand;

use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        
        println!("Enter a number between 1 and 100: ");

        io::stdin().read_line( &mut guess)
            .expect("Invalid input");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
            
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        }

    }
// we
/*

wrf
tre
*/
}