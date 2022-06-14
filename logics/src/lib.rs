use std::{io, process};

/// `TempUnit` is a type that represents the unit of temperature.
pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Sends a error when when an Error of Failed to Get occurs.
fn failed_to_get(err: io::Error) {
    eprintln!("Failed to get: {}", err);
}

/// Asks and gets a base temperature unit.
///
/// Returns the `Result<TempUnit, ()>`
pub fn get_temp_unit() -> Result<TempUnit, ()> {
    println!("Select the base temperature unit(a, b, c)");
    println!("a.Celsius(°C)");
    println!("b.Fahrenheit(°F)");
    println!("c.Absolute(Kelvin)(K)");

    let mut base_temp_unit = String::new();
    io::stdin()
        .read_line(&mut base_temp_unit)
        .unwrap_or_else(|err| {
            failed_to_get(err);
            process::exit(1);
        });

    // It's a string, so convert it to a string slice.
    // And remove leading and trailing whitespace.
    let base_temp_unit: &str = &base_temp_unit.trim();

    match base_temp_unit {
        "a" => Ok(TempUnit::Celsius),
        "b" => Ok(TempUnit::Fahrenheit),
        "c" => Ok(TempUnit::Kelvin),
        _ => {
            eprintln!("[Invalid value]");
            eprintln!("Please enter `a` or `b` or `c`");
            eprintln!("========================================");
            Err(())
        }
    }
}

/// Asks and gets a base temperature value.
///
/// Returns the `Result<f64, ()>`.
pub fn get_temp_val() -> Result<f64, ()> {
    println!("Enter the temperature value");

    let mut temp_value = String::new();
    io::stdin()
        .read_line(&mut temp_value)
        .unwrap_or_else(|err| {
            failed_to_get(err);
            process::exit(1);
        });

    // It's a string, so convert it to a f64.
    let temp_value: f64 = match temp_value.trim().parse() {
        Ok(num) => num,
        _ => {
            eprintln!("Please enter the numerical value:(");
            return Err(());
        }
    };
    Ok(temp_value)
}

/// Holds three different temperature units.
pub struct Temps {
    pub celsius: f64,
    pub fahrenheit: f64,
    pub kelvin: f64,
}

impl Temps {
    /// Creates a new "Temps" based on the given three temperatures.
    pub fn new(celsius: f64, fahrenheit: f64, kelvin: f64) -> Self {
        Self {
            celsius,
            fahrenheit,
            kelvin,
        }
    }

    /// Sends the result.
    pub fn show_result(self) {
        println!(
            "{}",
            if self.kelvin < 0.0 {
                "*Temperature is below absolute zero."
            } else {
                "[Converted successfully]"
            }
        );
        println!("Celsius: {}°C", self.celsius);
        println!("Fahrenheit: {}°F", self.fahrenheit);
        println!("Absolute(Kelvin): {}K", self.kelvin);
    }
}

/// Calculates temperatures based on the given base temperatures and unit.
///
/// And returns the Result<Temps, ()>
/// 
/// # Examples:
/// 
/// ```
/// let temps = calc_temp(TempUnit::Celsius, 36.5).unwrap();
/// let result = format!("\
/// Celsius: {}
/// Fahrenheit: {}
/// Absolute(Kelvin): {}",
///     temps.celsius,
///     temps.fahrenheit,
///     temps.kelvin
/// );
/// assert_eq!(
///     result,
///     "\
/// Celsius: 36.5
/// Fahrenheit: 97.7
/// Absolute(Kelvin): 309.65"
/// );
/// ```
pub fn calc_temp(temp_unit: TempUnit, temp_value: f64) -> Result<Temps, ()> {
    println!("Calculating...");

    let abs0: f64 = 273.15; // absolute zero in celsius temp.
    let c_temp = match temp_unit {
        TempUnit::Celsius => temp_value, // temp_value is already Celsius temp.
        TempUnit::Fahrenheit => (temp_value - 32.0) / 1.8, // °F -> °C
        TempUnit::Kelvin => temp_value - abs0, // K -> °C
    };
    let f_temp = match temp_unit {
        TempUnit::Celsius => temp_value * 1.8 + 32.0, // °C -> °F
        TempUnit::Fahrenheit => temp_value,           // temp_value is already Fahrenheit temp.
        TempUnit::Kelvin => (temp_value - abs0) * 1.8 + 32.0, // K -> °F
    };
    let k_temp = match temp_unit {
        TempUnit::Celsius => temp_value + abs0, // °C -> K
        TempUnit::Fahrenheit => (temp_value - 32.0) / 1.8 + abs0, // °F -> K
        TempUnit::Kelvin => temp_value,         // temp_value is already Absolute(Kelvin) temp.
    };

    Ok(Temps::new(c_temp, f_temp, k_temp))
}

/// Asks the user if they wish to continue with the conversion.
///
/// Returns a bool.
pub fn ask_wanna_continue() -> bool {
    println!("\nWould you like to continue with the conversion?[y, n]");
    let mut will_continue = String::new();
    io::stdin()
        .read_line(&mut will_continue)
        .unwrap_or_else(|err| {
            failed_to_get(err);
            process::exit(1);
        });
    match will_continue.trim() {
        "y" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfull_temp_calc() {
        let temps = calc_temp(TempUnit::Celsius, 36.5).unwrap();
        let result = format!("\
Celsius: {}
Fahrenheit: {}
Absolute(Kelvin): {}",
            temps.celsius,
            temps.fahrenheit,
            temps.kelvin
        );
        assert_eq!(
            result,
            "\
Celsius: 36.5
Fahrenheit: 97.7
Absolute(Kelvin): 309.65"
        );
    }
}