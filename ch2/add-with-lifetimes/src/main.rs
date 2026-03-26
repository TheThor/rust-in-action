fn main() {
    let a = 20;
    let b = 30;

    let sum = add_with_lifetimes(&a, &b);
    println!("{}", sum)
}

fn add_with_lifetimes<'a,'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j // adds values, not the references to them
}