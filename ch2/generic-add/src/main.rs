use std::time;

fn main() {
    let float_sum = add(2.0, 3.3);
    println!("{}", float_sum);
    let int_sum = add(1,3);
    let add_durations = add(
        Duration::from_secs_f64(float_sum),
        Duration::from_secs_f64(int_sum),
    );
}

fn add<T : Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
