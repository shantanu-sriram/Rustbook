use std::io;

fn main() {
    println!("Hello, world!");
    let converted = temp_conversion(25.0);
    println!("Converted temperature: {converted}")
}
fn temp_conversion(temp: f32) -> f32{
    println!("Input c for Celsius to Fahrenheit conversion:");
    println!("Input f for Fahrenheit to Celsius conversion:");

    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");
    match temp_type.trim().to_lowercase().as_str(){
        "c" => (temp* 1.8) + 32.0,
        "f" => (temp-32.0) * 5.0/9.0,
        _ => {
            println!("Invalid type");
            temp}
    }
}

