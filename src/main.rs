mod units;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use dialoguer::Input;
use units::temperature::celcius::Celcius;
use units::temperature::fahrenheit::Fahrenheit;
use units::temperature::kelvin::Kelvin;
use units::unit::Unit;

fn main() {
    // Available units
    let items = vec!["Celcius", "Fahrenheit", "Kelvin"];

    // Select unit to convert from
    let selected_from_unit_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the unit to convert from")
        .items(&items)
        .interact()
        .unwrap();

    // Input the value to convert
    let from_value = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("The value that you want to convert: ")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.parse::<f32>().is_ok() {
                Ok(())
            } else {
                Err("This is not a number")
            }
        })
        .default("0".to_string())
        .show_default(true)
        .interact()
        .unwrap()
        .parse::<f32>()
        .unwrap();

    // Remove the selected "from" unit (celcius cannot be converted to celcius)
    let mut items_without_from_unit = items.clone();
    items_without_from_unit.remove(selected_from_unit_index);


    // Select the unit to convert to
    let selected_to_unit_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the unit you wish to convert the value to")
        .items(&items_without_from_unit)
        .interact()
        .unwrap();

    // Construct the original unit object
    let from_unit: Option<Unit> = match items[selected_from_unit_index] {
        "Celcius" => Some(Unit::Celcius(units::temperature::celcius::Celcius::new(from_value))),
        "Fahrenheit" => Some(Unit::Fahrenheit(units::temperature::fahrenheit::Fahrenheit::new(from_value))),
        "Kelvin" => Some(Unit::Kelvin(units::temperature::kelvin::Kelvin::new(from_value))),
        _ => None,
    };


    // Make the conversion
    let to_converted_unit: Option<Unit> = match items_without_from_unit[selected_to_unit_index] {
        "Celcius" => Some(Unit::Celcius(Celcius::from(from_unit.expect("Cannot convert")))),
        "Fahrenheit" => Some(Unit::Fahrenheit(Fahrenheit::from(from_unit.expect("Cannot convert")))),
        "Kelvin" => Some(Unit::Kelvin(Kelvin::from(from_unit.expect("Cannot convert")))),
        _ => None,
    };


    // Print the result
    println!("{}", to_converted_unit.expect("Cannot convert"))
}
