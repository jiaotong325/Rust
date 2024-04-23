mod rand_number;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number:");
    let secrt_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    let msg = "无法读取.";
    io::stdin()
        .read_line(&mut guess)
        .expect(msg);
    println!("猜测的是：{}",guess);
}
