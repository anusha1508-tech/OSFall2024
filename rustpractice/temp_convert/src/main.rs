// Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_F: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;

    // Convert it to Celsius using the function and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);

    // Loop to convert and print the next 5 integer temperatures
    for _ in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, temp_c);
    }

    // Demonstrating celsius_to_fahrenheit usage
    let temp_c = 0.0; // 0°C (freezing point in Celsius)
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("{}°C is {:.2}°F", temp_c, temp_f);
}
