use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    guess();
    // test();
}

fn guess() {
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    loop {
        println!("Введите число:");
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Не удалось считать ввод пользователя");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Не число!");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Greater => println!("{} is too high", guess),
            Ordering::Less => println!("{} is too low", guess),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test() {
    let mut a = 10u64;
    let mut b = a;
    a = 12u64;
    println!("{}", b);
    b = 14u64;
    println!("{}", a);
}
