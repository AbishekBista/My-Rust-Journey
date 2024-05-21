use std::io;

fn main() {
    let mut tempFInput = String::new();
    let mut tempCInput = String::new();

    println!("Enter the temperatures in fahrenheit: ");

    io::stdin().read_line(&mut tempFInput).expect("Failed to read line.");

    println!("TempF received: {tempFInput}");

    let tempC: f32 = fahrenheit_to_celsius(tempFInput.trim().parse().expect("Failed to convert"));

    println!("TempC received from function: {tempC}C");

    println!("Enter the temperature in celsius: ");
    io::stdin().read_line(&mut tempCInput).expect("Failed to read line");

    println!("TempC received: {tempC}");
    let tempF: f32 = celsius_to_fahrenheit(tempCInput.trim().parse().expect("Failed to convert"));
    println!("TempF is received from function: {tempF}F");
}

fn fahrenheit_to_celsius(temp_in_fahrenheit: f32) -> f32 {
    return (temp_in_fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp_in_celsius: f32) -> f32 {
    9.0/5.0 * temp_in_celsius + 32.0
} 
