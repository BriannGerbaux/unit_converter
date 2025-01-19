use std::fmt;

use super::temperature::{celcius::Celcius, fahrenheit::Fahrenheit, kelvin::Kelvin};

pub enum Unit {
    Celcius(Celcius),
    Fahrenheit(Fahrenheit),
    Kelvin(Kelvin),
}

impl fmt::Display for Unit {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let r = match self {
            Unit::Fahrenheit(v) => v.fmt(f),
            Unit::Celcius(v) => v.fmt(f),
            Unit::Kelvin(v) => v.fmt(f),
        };
        r
    }
}