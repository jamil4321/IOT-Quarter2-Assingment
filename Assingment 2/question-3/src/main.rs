#[derive(Debug)]
struct Cashier<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cashier<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cashier<T> {
        Cashier {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut cash = Cashier::new(|a| a);
    let v1 = cash.value(3);
    println!("{}", v1);
}
