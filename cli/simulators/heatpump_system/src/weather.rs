pub fn get_daily_temperature(day_of_year: u32) -> f64 {
    let average_temp = 10.0; // average temperature in Celsius for Netherlands
    let amplitude = 12.0; // difference between peak summer and winter
    average_temp + amplitude * (day_of_year as f64 * 2.0 * std::f64::consts::PI / 365.0).sin()
}
