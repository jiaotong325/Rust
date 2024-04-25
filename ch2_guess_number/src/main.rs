mod rand_number;
use std::io;
use rand::Rng;

fn main() {
    // println!("Guess the Number:");
    // let secrt_number = rand::thread_rng().gen_range(1..=100);
    // let mut guess = String::new();
    // let msg = "无法读取.";
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect(msg);
    // println!("猜测的是：{}",guess);

    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5

    s.clear();
    println!("the first word is: {}", word);

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }
}
