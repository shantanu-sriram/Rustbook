use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    loop {
        println!("Guess the number");
        println!("please input your number");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!(" You guessed: {}", guess);
        println!("the secret number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! \n"),
            Ordering::Greater => println!("Too big! \n"),
            Ordering::Equal => {println!("You win! \n");
            break; }
        }
    }
}
