
fn main() {
    let mut x = String::from("Welcome to");
    let mut z = || x.push_str(" PIAIC"); 
    question_2(z);
    println!("{}",x);
}

fn question_2<T:FnMut()>(mut y:T){
    y();
}