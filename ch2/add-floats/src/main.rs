fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // commented from book example to allow the rest to execut. uncomment to see failure
    //assert!(xyz.0 + xyz.1 == xyz.2);

    // Second part of example what happens is:
    // Is the distance between the numbers small enough?
    // Not: are they identical?
    // this allows s to use epsilon as a distance from a to b, and ask if enough to e different
    // epsilon is 1e-10 or 0.0000000001
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
}
