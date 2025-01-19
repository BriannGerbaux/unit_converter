use crate::units::temperature::celcius::*;
use crate::units::temperature::fahrenheit::*;
use crate::units::unit::Unit;

use std::fmt;

pub struct Kelvin {
    pub degrees: f32,
}

impl Kelvin {
    pub fn new(value: f32) -> Self {
        Kelvin { degrees: value }
    }
}

impl From<Celcius> for Kelvin {
    fn from(item: Celcius) -> Self {
        Kelvin { degrees: item.degrees + 273.15 }
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(item: Fahrenheit) -> Self {
        Kelvin { degrees: Celcius::from(item).degrees + 273.15 }
    }
}

impl From<Unit> for Kelvin {
    fn from(item: Unit) -> Self {
        let r = match item {
            Unit::Fahrenheit(v) => Kelvin::from(v),
            Unit::Celcius(v) => Kelvin::from(v),
            _ => panic!("Cannot convert")
        };
        r
    }
}

impl fmt::Display for Kelvin {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}K", self.degrees)
    }
}