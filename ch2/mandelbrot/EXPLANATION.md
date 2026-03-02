# Mandelbrot Implementation: My Thought Process

## Overview
This document captures my reasoning and step-by-step thinking while implementing the Mandelbrot set renderer in Rust. It includes my comments and explanations from the code, annotated for clarity.

---

### Mandelbrot Formula
- The Mandelbrot formula is \(Z_{n+1} = Z_n^2 + c\), where \(c = x + yi\).

### Starting Point
- I start with point `x_min` and `y_max` because the y-axis is inverted in the terminal (top to bottom) and I want to start from the top left corner of the plane.

### View Settings
- I define my view with `width` and `height` (e.g., 200x48 for terminal output).
- It might be worth moving the settings of the "window" to the function parameters for flexibility.
- I also think that points should be float due to being able to have a bit more "definition".

### Iteration Logic
- I iterate from each point by going from each line x, for each column.
- For example, I zoom into `x_min = -1.0` and `x_max = 3.0`. That gives me 4.0 (math vector from A to B) / 800 (width), so step_x = 0.005. Each of the 800 points will be -1.0 + 0.005 + 0.005 + ...
- After getting the step, for a given example (`x_min = -2.0`, `x_max = 1.0`, `width = 800`), I calculate that pixel (x point in iter) multiplies by step_x/y then add that result to x_min. For x iteration 45, step_x = 0.00375, so real_x = -2 + (45 * 0.00375) = -1.83125. This step of our resolution should show this point x.

### Mapping Explanation
- The mapping from pixel indices to complex coordinates is crucial. Instead of just calculating the step size, I use the pixel index to get the actual coordinate: `real_x = x_min + (x as f64) * step_x` and `real_y = y_min + (y as f64) * step_y`.
- This way, every pixel represents a unique point in the chosen range.

### Mandelbrot Point Calculation
- In `mandelbrot_point`, I apply the Mandelbrot iteration: start with z = 0, then repeatedly compute z = z^2 + c up to max_iters or until |z| > 2. Return the iteration count when escape occurs.
- The escape value is what gets added to the vector for that point.

### ASCII Rendering
- The `render_mandelbrot` function is a helper for visual representation, not mathematical reality. It maps the escape value to ASCII characters for visualization.
- Points that never escape (i.e., max_iters reached) are mapped to `%`. Values between 2 and max are mapped to other characters (`.`, `*`, `+`, etc.) to show how quickly each point escapes.
- Copied this from the book, basically, because I would have no way of figuring this artistic part out by myself
  - Each different amount of iterations defines a sort of artistic choice of using one of the ASCII, where each represent a different visual

### Summary
- The code structure follows the Mandelbrot algorithm: mapping pixel coordinates to complex numbers, calculating escape values, and rendering them in ASCII.
- The mapping formula lets me control which part of the set I see and at what resolution.
- The ASCII art is just a way to visualize the escape values, not a mathematical property.

---

*This document is a direct reflection of my learning and reasoning process while building the Mandelbrot set renderer in Rust.*

