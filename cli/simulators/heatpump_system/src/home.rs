pub struct Home {
    pub square_meters: f64,
    pub insulation_efficiency: f64, // Value between 0 (no insulation) to 1 (perfect insulation)
    pub current_temperature: f64,
}

impl Home {
    pub fn compute_heat_loss(&self, ambient_temperature: f64) -> f64 {
        let temp_difference = self.current_temperature - ambient_temperature;
        // Heat loss is proportional to the difference in temperature
        // and inversely proportional to insulation efficiency
        temp_difference / self.insulation_efficiency
    }
}
