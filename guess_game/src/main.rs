extern crate rand;  // 使用外部库

use std::io;  // 使用标准io库，就像是我们的 import 语句一样
use std::cmp::Ordering;
use rand::Rng;  // 使用一个随机数生成库

fn main() {
    println!("Guess the number!");  // 这是一个宏

    let secret_number = rand::thread_rng().gen_range(1, 101); // 注意 let 语句

    println!("the secret number is {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");
        println!("Your guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too Small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
