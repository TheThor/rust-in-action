fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // if a < b { this one fails obviously
    if a < (b as i32) { // this one casts it
        println!("Ten is less than one hundred.");
    }


}
