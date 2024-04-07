use std::io::{self, Write};

const VERSION: &str = "v0.1.0";

fn main() {
    println!(
        "kepler-solver {} - by Ethan Barry <ethan.barry@howdytx.technology>",
        VERSION
    );
    println!("This program computes the eccentric anomaly from");
    println!("the mean anomaly and the orbit's eccentricity.");
    println!("----------");
    // First we'll collect the two parameters we need...
    // And yes, Rust's string I/O is gross.
    print!("Enter the planet's eccentricity (e): ");
    io::stdout().flush().expect("Flush failed.");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O Failed.");
    let e: f64 = line.trim().parse().expect("Invalid value.");

    line.clear(); // Clear the string.

    print!("Enter the mean anomaly (M): ");
    io::stdout().flush().expect("Flush failed.");
    io::stdin().read_line(&mut line).expect("I/O Failed.");
    let mean_anomaly: f64 = line.trim().parse().expect("Invalid value.");

    // ...then compute the actual value of E within the tolerance.
    let res = newton_raphson(e, mean_anomaly, 0_f64, 0.000001);
    println!("Eccentric anomaly: {res}");
}

/// A simple Newton-Raphson solver for the Kepler equation.
fn newton_raphson(e: f64, mean_anomaly: f64, initial_guess: f64, tolerance: f64) -> f64 {
    let mut guess = initial_guess;
    let mut next_guess = initial_guess + 1_f64;
    while (next_guess - guess).abs() > tolerance {
        next_guess = guess - ((guess - e * guess.sin() - mean_anomaly) / (1_f64 - e * guess.cos()));
        guess = next_guess;
    }
    guess
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_solution() {
        //! Values from <http://www.csun.edu/~hcmth017/master/node16.html>
        //! This just goes to show how precise we can get! Very cool, if you ask me!

        let e = 0.00001;
        let mean_anom = 0.5235988; // Roughly π/6.
        let ans = newton_raphson(e, mean_anom, mean_anom, 0.000001);

        assert!((ans - 0.5236038).abs() < 0.000001);
    }
}
