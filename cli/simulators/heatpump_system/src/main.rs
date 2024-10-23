mod home;
mod system;
mod utils;
mod weather;

use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use home::Home;
use system::{AirHandler, Boiler, ElectricBattery, HeatPump, HomeSystem};
use weather::get_daily_temperature;

const HEAT_LOSS_THRESHOLD: f64 = 1.5; 

fn simulate_day(system: &mut HomeSystem, home: &mut Home, ambient_temperature: f64) {
    for _hour in 0..24 {
        let heat_loss = home.compute_heat_loss(ambient_temperature);

        if heat_loss > HEAT_LOSS_THRESHOLD {
            system.regulate_temperature(ambient_temperature);
        }

        if system.peak_moment() {
            system.distribute_heat();
        } else {
            system.boil_water();
        }
    }
}


fn main() {
    let mut home = Home {
        square_meters: 100.0,
        insulation_efficiency: 0.8,
        current_temperature: 20.0,
    };

    let heat_pump = HeatPump::new();
    let battery = ElectricBattery::new();
    let boiler = Boiler::new();
    let air_handler = AirHandler::new();

    let mut system = HomeSystem {
        heat_pump,
        battery,
        boiler,
        air_handler,
        desired_temperature: 22.0,
    };

    let mut results = vec![];

    for day in 0..365 {
        let ambient_temperature = get_daily_temperature(day);
        simulate_day(&mut system, &mut home, ambient_temperature);

        results.push((
            day,
            ambient_temperature,
            home.current_temperature,
            system.water_temperature(),
            system.battery_level(),
        ));
    }
    
}



pub fn display_simulation_results(results: &Vec<(u32, f64, f64, f64, f64)>) {
    let table: Vec<Vec<CellStruct>> = results
        .into_iter()
        .map(|(day, ambient_temp, home_temp, water_temp, battery_level)| {
            vec![
                day.cell().justify(Justify::Right),
                format!("{:.2}", ambient_temp).cell().justify(Justify::Right),
                format!("{:.2}", home_temp).cell().justify(Justify::Right),
                format!("{:.2}", water_temp).cell().justify(Justify::Right),
                format!("{:.2}", battery_level).cell().justify(Justify::Right),
            ]
        })
        .collect();

    let table = table
        .table()
        .title(vec![
            "Day".cell().bold(true),
            "Ambient Temp".cell().bold(true),
            "Home Temp".cell().bold(true),
            "Water Temp".cell().bold(true),
            "Battery Level".cell().bold(true),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
