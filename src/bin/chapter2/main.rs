use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number ( 1 - 100 )");

    let answer = rand::thread_rng().gen_range(1..=100);

    let mut game_finished = false;

    while !game_finished {

        println!("Please input your guess.");

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = guess_str.trim()
            .parse()
            .expect("Please type a number!");

        if guess > answer {
            println!("Too high");
        } else if guess < answer {
            println!("Too low");
        } else {
            println!("You Win !");
            game_finished = true;
        }
    }
}