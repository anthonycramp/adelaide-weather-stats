use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

const DATA: &str = include_str!("../data/IDCJAC0010_023000_1800_Data.csv");

fn main() {
    println!("Hello, world!");

    let mut coldest_temperature_by_year: HashMap<i32, f32> = HashMap::new();
    let mut coldest_day_by_year: HashMap<i32, NaiveDate> = HashMap::new();

    let mut first_line = true;
    for line in DATA.lines() {
        if first_line {
            // skip the first non-data line
            first_line = false;
            continue;
        }
        let mut fields = line.split(",");
        let _product_code = fields.next().unwrap();
        let _station_number = fields.next().unwrap();
        let year: i32 = fields.next().unwrap().parse().unwrap();
        let month: u32 = fields.next().unwrap().parse().unwrap();
        let day: u32 = fields.next().unwrap().parse().unwrap();
        let dt = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        if !coldest_temperature_by_year.contains_key(&year) {
            coldest_temperature_by_year.insert(year, f32::MAX);
            coldest_day_by_year.insert(year, dt);
        }

        let maximum_temperature = fields.next().unwrap();

        match maximum_temperature.parse::<f32>() {
            Ok(v) => {
                let coldest_temp_this_year = coldest_temperature_by_year.get_mut(&year).unwrap();
                if v < *coldest_temp_this_year {
                    *coldest_temp_this_year = v;
                    coldest_day_by_year.insert(year, dt);
                }
            }
            Err(e) => {}
        }
    }
    let mut years: Vec<_> = coldest_temperature_by_year.keys().collect();
    years.sort();

    let mut number_of_days = 0;
    let mut sum_of_day_numbers = 0;

    for year in years {
        let coldest_day = coldest_day_by_year.get(&year).unwrap();
        let coldest_temp = coldest_temperature_by_year.get(&year).unwrap();
        if *coldest_temp < 20.0 {
            number_of_days += 1;
            sum_of_day_numbers += coldest_day.ordinal();

            println!(
                "{}/{}/{}: {}",
                coldest_day.year(),
                coldest_day.month(),
                coldest_day.day(),
                coldest_temp
            );
        }
    }

    let average_day_number = sum_of_day_numbers / number_of_days;
    println!("{}", average_day_number);
}
