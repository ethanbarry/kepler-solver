use std::io::{self, Write};

fn main() {
    // First, we need the orbital elements.
    // Specifically, a, e, m_planet & m_star, the time of perihelion T,
    // the gravitational constant G, aaaaand... that's it.

    print!("Enter the planet's eccentricity (e): ");
    io::stdout().flush().expect("Flush failed.");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O Failed.");
    let e: f64 = line.trim().parse().expect("Invalid value.");

    line.clear();

    print!("Enter the mean anomaly (M): ");
    io::stdout().flush().expect("Flush failed.");
    io::stdin().read_line(&mut line).expect("I/O Failed.");
    let mean_anomaly: f64 = line.trim().parse().expect("Invalid value.");

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
