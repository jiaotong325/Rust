use std::io;

fn main() {
    println!("Guess the Number:");
    let mut guess = String::new();
    let msg = "无法读取.";
    io::stdin()
        .read_line(&mut guess)
        .expect(msg);
    println!("猜测的是：{}",guess);
}
