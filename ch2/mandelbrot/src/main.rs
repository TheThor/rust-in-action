use num::Complex;

// Mandelbrot formula is (Z_{n+1} = Z_n^2 + c), where (c = x + yi)

fn main() {
    calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0);
}

fn calculate_mandelbrot(
    max_iters: usize, // this will be our n?
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) {
    // Starting with point x_min and y_max because
    // the y-axis is inverted in the terminal (top to bottom) and
    // we want to start from the top left corner of the plane

    let mut mandelbrot_escape_map: Vec<Vec<usize>> = Vec::new();

    // Define our view
    let width = 200;  // pixel width
    let height = 48; // pixel height

    // might be worth moving the settings of the "window" to the fn parameters
    // i also think that points should be float due to being able to have a bit more "definition"
    //let y_window_top = 800; // need to check this behavior
    //let y_window_bottom = 600;

    // iterate from each point by going from each line x, for each column
    for y in 0..height
    {
        let mut row: Vec<usize> = Vec::new();

        // this check is almost valid but needs to
        // take into account the adjustments on next comment
        for x in 0..width
        {
            // Okay we do the following as a visual example:
            // For example. I zoom into x_min = -1.0 and x_max = 3.0.
            // That gives me 4.0 (math vector from A to B)/800 (width) an x 0,005,
            // so it should "jump" from 0,005 to 0,005. In short each 1 point of
            // the 800 points, will be -1.0 + 0,005 + 0,005+ 0,005... so on and so forth.
            let step_x = (x_max - x_min) / width as f64;
            let step_y = (y_max - y_min) / height as f64;

            // after getting the step we need to get, for given example,
            // x_min = -2.0, x_max = 1.0, width = 800,
            // I need to calculate that pixel (x point in iter) multiplies by
            // step_x/y then you add that result to x_min, i.e., for x iteration 45,
            // step_x = (1 - (-2)) / 800 = 0.00375 then
            // real_x = -2 (our beginning) + [45 * 0.0035] (this gets "length" of 45 steps)
            // giving a real_x of -2 + 0,16875 = -1,83125
            // in short, this step of our resolution should show this point x.

            let real_x = x_min + (x as f64) * step_x;
            let real_y = y_min + (y as f64) * step_y;


            let c : Complex<f64> = Complex { re: real_x, im: real_y };
            let escape_value = mandelbrot_point(max_iters, c);
            row.push(escape_value)
        }
        mandelbrot_escape_map.push(row);
    }

    render_mandelbrot(mandelbrot_escape_map);
}

fn mandelbrot_point(
    max_iters: usize,
    c: Complex<f64>,
) -> usize {
    // start with a Z for z = (z * z) + c; as 0
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }

    max_iters
}


// this one I could never get there without looking at ascii art
fn render_mandelbrot(
    escape_values: Vec<Vec<usize>>,
) {
    for row in escape_values {
        for point in row {
            let current_value = match point {
                // Copy-pasted as I was given no ASCI art perks when spawned
                0..=2 => ' ',
                3..=4 => '.',
                5..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            print!("{}", current_value);
        }
        println!();
    }
}