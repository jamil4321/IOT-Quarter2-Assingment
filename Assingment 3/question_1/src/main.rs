fn main() {
let x = String::from("PIAIC");
let word = || x;
question_1(word);
}

fn question_1<F>(y: F)
    where F: FnOnce() -> String
{

    println!("Welcome to {}", y());

    println!("Internet of Things\nBatch 2 Quarter 2");
}