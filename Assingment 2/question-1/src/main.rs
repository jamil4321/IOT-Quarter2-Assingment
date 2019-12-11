use std::io;
fn main() {
    println!("======== Welcome To My Calculator ========");

    let mut flag = false;
    while flag != true {
        println!("1) Addition \n2) Subtraction \n3) Multiplication \n4) Division \n5) Power \n0) Exit");
        println!("Enter The Task Number You Want to Perform");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: i32 = user_input
            .trim()
            .parse()
            .ok()
            .expect("Program only processes numbers, Enter number");

        if user_input == 0 {
            println!("Thanks For using my Calculator");
            flag = true;
        } else {
            println!("Enter Your First Number");
            let mut num1 = String::new();
            let mut num2 = String::new();
            io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            println!("Enter Your First Number");
            io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let mut f = false;
            let mut f2 = false;
            for i in num1.chars() {
                if i == '.' {
                    f = true;
                    break;
                }
            }
            for i in num2.chars() {
                if i == '.' {
                    f2 = true;
                    break;
                }
            }
            if f == true || f2 == true {
                let mut num1: f32 = num1
                    .trim()
                    .parse()
                    .ok()
                    .expect("Program only processes numbers, Enter number");
                let mut num2: f32 = num2
                    .trim()
                    .parse()
                    .ok()
                    .expect("Program only processes numbers, Enter number");
                    if user_input == 1 {
                        println!("Addition of {} & {} is {}", num1, num2, num1 + num2)
                    } else if user_input == 2 {
                        println!("Subtraction of {} & {} is {}", num1, num2, num1 - num2)
                    } else if user_input == 3 {
                        println!("Multiplication of {} & {} is {}", num1, num2, num1 * num2)
                    } else if user_input == 4 {
                        println!("Division of {} & {} is {}", num1, num2, num1 / num2)
                    } else if user_input == 5 {
                        println!("Sorry Can't calculate power for floating value")
                    }else {
                        println!("You Enter an invalid number")
                    }
            } else {
                let mut num1: i32 = num1
                    .trim()
                    .parse()
                    .ok()
                    .expect("Program only processes numbers, Enter number");
                let mut num2: i32 = num2
                    .trim()
                    .parse()
                    .ok()
                    .expect("Program only processes numbers, Enter number");
                    if user_input == 1 {
                        println!("Addition of {} & {} is {}", num1, num2, num1 + num2)
                    } else if user_input == 2 {
                        println!("Subtraction of {} & {} is {}", num1, num2, num1 - num2)
                    } else if user_input == 3 {
                        println!("Multiplication of {} & {} is {}", num1, num2, num1 * num2)
                    } else if user_input == 4 {
                        println!("Division of {} & {} is {}", num1, num2, num1 / num2)
                    }else if user_input == 5 {
                        let mut ans = num1;
                        for i in 1..num2{
                          ans = ans*num1;
             
                        }
                       
                        println!("Division of {} & {} is {}", num1, num2,  ans)
                    } else {
                        println!("You Enter an invalid number")
                    }
            };

        };
        println!("Would you like to calculate again? (y/n)");
        let mut promt = String::new();
        io::stdin()
            .read_line(&mut promt)
            .expect("Failed to read line");
        let promt: String = promt.trim().parse().unwrap();
        if promt == "n" {
            println!("Thanks For using my Calculator");
            flag = true;
        }
        println!("=============================================")
    }
}
