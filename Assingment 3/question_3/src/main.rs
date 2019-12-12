use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..30 {
            if i%2 ==0{
                println!("{} is an even number in i loop",i)
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    for j in 1..15 {
        if j%2 ==1{
            println!("{} is an odd number in j loop",j)
        }
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}