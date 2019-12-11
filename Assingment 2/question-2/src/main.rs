// fn main() {
//     let x =||println!("hello world");
//     x();
// }
// fn main() {
//     let x = |z|z+1;//Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
//     let y = 10;
//     println!("The function returns: {}",x(y)); 
// }
// fn main() {
//     let mut c = 1;
//     let mut x =||{c=c+1;}; //Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }
// // Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
fn main() {
    let x = ||format!("I Can Write Closure Functions");

    fn hello(){
            x();
      };
    println!("{}",hello());
    
}
// // Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
// fn main() {
//     let x = |y| y + 1;

//     fn hello<T:Fn(u32)->u32>(x:T)->u32{
//             x(2)
//         };
//         println!("{}",hello(x));
// }