use std::ops::{Add};
use std::time::{Duration};

fn main() {
    let float_sum = add(2.0, 10.3);
    let int_sum = add(1,8);
    let add_durations = add(
        Duration::new(15,0),
        Duration::new(10, 0),
    );

    println!("{}", float_sum);
    println!("{}", int_sum);
    println!("{:?}", add_durations); //  :? debug formatter
}

fn add<T : Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
