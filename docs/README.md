## D version
```d
import std.math;
import std.stdio;

double sqr(double a) {
	return a*a;
}

double sqr_prime(double a) {
	return 2*a;
}

alias Func = double function(double);
alias FuncPrime = double function(double);

double newton_raphson(Func func, FuncPrime func_prime, double A, double initialGuess, int maxIterations = 1_000, double tolerance = 1e-6) {
    double x = initialGuess;

    for (int i = 0; i < maxIterations; ++i) {
        double f_x = func(x) - A;
        double f_prime_x = func_prime(x);

        if (abs(f_prime_x) < tolerance) {
            writeln("Derivative too small, cannot continue.");
			
            break;
        }

        x = x - f_x / f_prime_x;

        if (abs(f_x) < tolerance) {
            writeln("\nConverged in ", i, " iterations");
			
            break;
        }
    }

    return x;
}

void main() {
    double A = 11_025.0; // Find the square root of 11_025

    double result = newton_raphson(&sqr, &sqr_prime, A, A/2.0);
    
    if (isNaN(result)) {
        writeln("Newton-Raphson method did not converge.");
    } else {
        writeln("Square root of ", A, " is approximately ", result);
    }

	writeln;
}
```

## Rust version

```rust
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
```

## Usage

```bash
git clone
```

## Output

```
Converged in 10 iterations
Square root of 11025 is approximately 105
```
