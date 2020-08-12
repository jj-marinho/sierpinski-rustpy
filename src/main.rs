use rand::Rng;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::stdout;

#[derive(Serialize)]
struct Points<'a>(&'a [f64], &'a [f64]);

fn main() {
    let number_of_points = input("How many points? ").parse::<usize>().unwrap();

    // Creating vectors that will hold x and y coordinate data
    let mut x = vec![0.; number_of_points];
    let mut y = vec![0.; number_of_points];

    // Calculating specific x and y position
    // I wanted to run this in parallel, but as it depends on previous random
    // value in the array, i wasn't able to come up with a solution
    for a in 1..number_of_points {
        let (x_extra, y_extra) = get_random_corner();

        x[a] = 0.5 * x[a - 1] + x_extra;
        y[a] = 0.5 * y[a - 1] + y_extra;
    }

    // Serializing data to json format
    let json = serde_json::to_vec_pretty(&Points(&x, &y)).unwrap();

    // Saving data in filesystem
    save_data("./exec/data.json", &json);
}

/// save data to file
fn save_data(filename: &str, data: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write(data).unwrap();
}

/// Gets random number from 0 to 2, and calculates the random corner
fn get_random_corner() -> (f64, f64) {
    match rand::thread_rng().gen_range(0, 3) {
        0 => (0.0, 0.0),
        1 => (0.25, 3.0f64.sqrt() / 4.0),
        _ => (0.5, 0.0),
    }
}

/// input mimics python input function
pub fn input(msg: &str) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s).unwrap();

    s.strip_suffix("\n").unwrap().to_string()
}
