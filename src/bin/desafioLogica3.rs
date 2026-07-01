fn main() {
    // Initialize variables
    let is_sunny: bool = true;
    let wind_speed: f64 = 5.4;
    let temperature: i32 = 23;
    let solar_panel_output: i32 = 9;
    let is_cloudy: bool = false;
    
    // The complete logical expression
    let result: bool = ((is_sunny && wind_speed < 10.0)&&(solar_panel_output<15 && temperature>20) && !(is_cloudy));
    
    // Don't delete the lines below
    println!("Checking conditions for solar energy production...");
    println!("1. Is it sunny? {}", is_sunny);
    println!("2. Is wind speed safe? {}", (wind_speed < 10.0));
    println!("3. Can panels produce more? {}", (solar_panel_output < 15));
    println!("4. Is temperature good OR no clouds? {}", (temperature > 20 || !is_cloudy));
    println!("\\nFinal result - Good day for solar energy production: {}", result);
}
