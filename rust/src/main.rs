use std::f64;

type Func = fn(f64) -> f64;
type FuncPrime = fn(f64) -> f64;

fn sqr(a: f64) -> f64 {
    a * a
}

fn sqr_prime(a: f64) -> f64 {
    2.0 * a
}

fn newton_raphson(func: Func, func_prime: FuncPrime, a: f64, initial_guess: f64, max_iterations: usize, tolerance: f64) -> f64 {
    let mut x = initial_guess;

    for i in 0..max_iterations {
        let f_x = func(x) - a;
        let f_prime_x = func_prime(x);

        if f_prime_x.abs() < tolerance {
            eprintln!("Derivative too small, cannot continue.");
			
            break;
        }

        x = x - f_x / f_prime_x;

        if f_x.abs() < tolerance {
            eprintln!("\nConverged in {} iterations", i);
			
            break;
        }
    }

    x
}

fn main() {
    let a = 11_025.0; // Find the square root of 11025

    let result = newton_raphson(sqr, sqr_prime, a, a / 2.0, 1_000, 1e-6);
    
    if result.is_nan() {
        eprintln!("Newton-Raphson method did not converge.");
    } else {
        println!("Square root of {} is approximately {}", a, result);
    }
}
