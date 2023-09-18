mod home;
mod system;
mod utils;
mod weather;

use cli_table::{format::Justify, print_stdout, Table, Cell, Row, Style, Color};
use home::Home;
use system::HomeSystem;
use weather::get_daily_temperature;

fn simulate_day(system: &mut HomeSystem, home: &mut Home, ambient_temperature: f64) {
    for hour in 0..24 {
        let heat_loss = home.compute_heat_loss(ambient_temperature);

        if heat_loss > some_threshold {
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

    let mut system = /* ... initialize your HomeSystem ... */;
    let mut results = vec![];

    for day in 0..365 {
        let ambient_temperature = get_daily_temperature(day);
        simulate_day(&mut system, &mut home, ambient_temperature);

        // Store results for the day
        results.push((
            day,
            ambient_temperature,
            home.current_temperature,
            system.water_temperature(),
            system.battery_level(),
        ));
    }

    // Create the table header
    let table = vec![
        Row::new(vec![
            "Day".cell().bold(true),
            "Ambient Temp".cell().bold(true),
            "Home Temp".cell().bold(true),
            "Water Temp".cell().bold(true),
            "Battery Level".cell().bold(true),
        ])
        .bold(true)
        .fg(Color::Cyan),
    ]
    .add_rows(
        results
            .into_iter()
            .map(|(day, ambient, home_temp, water_temp, battery)| {
                vec![
                    day.cell().justify(Justify::Right),
                    format!("{:.2}", ambient).cell(),
                    format!("{:.2}", home_temp).cell(),
                    format!("{:.2}", water_temp).cell(),
                    format!("{:.2}", battery).cell(),
                ]
            }),
    );

    // Print the table
    print_stdout(table.with(Style::pseudo()));
}
