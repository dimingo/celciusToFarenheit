use std::io;

fn main() {
    println!("Please Input Degress in celcius");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed To Read the line");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Please Input The Temperature");

    let farenheit = convert_c_to_f(temperature);

    println!("The temperature in farenheit is {}", farenheit);
}

fn convert_c_to_f(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}
