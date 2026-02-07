use num::Complex::Complex;
fn main() {
    let calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0);
}

// Zn+1 = Zn^2 + c
// where c = x + yi (coordinates on a (2d) plane)
fn calculate_mandelbrot(
    max_iters: usize, // this will be our n?
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) {
    let mut c = Complex { re: x, im: y };
    for something in max_iters {

    }
}

fn mandelbrot_point(
    max_iters: usize,
    c: Complex<f64>,
) {
    let z = Complex::new(c.re, c.im);
    for i in 0..max_iters {
        z = Complex::new(c.re, c.im);
        if (z > 2) {
            return i;
        }
        z = z * z + c;
    }
}

fn render_mandelbrot(
    escape_values: Vec<Vec<usize>>,
) {
    for row in escape_values {
        for point in row {
            let current_value = match point {
                // Copy-pasted as I was given no ASCI art perks when spawned
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '•',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%',
            };
            print!("{}", current_value);
        }
        println!();
    }
}