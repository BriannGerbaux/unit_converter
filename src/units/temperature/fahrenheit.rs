use crate::units::temperature::celcius::Celcius;
use crate::units::temperature::kelvin::*;
use crate::units::unit::Unit;

use std::fmt;

pub struct Fahrenheit {
    pub degrees: f32,
}

impl Fahrenheit {
    pub fn new(value: f32) -> Self {
        Fahrenheit { degrees: value }
    }
}

impl From<Celcius> for Fahrenheit {
    fn from(item: Celcius) -> Self {
        Fahrenheit { degrees: ((9.0 / 5.0) * item.degrees) + 32.0 }
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(item: Kelvin) -> Self {
        Fahrenheit { degrees: (item.degrees - 273.15) * 1.8 + 32.0 }
    }
}

impl From<Unit> for Fahrenheit {
    fn from(item: Unit) -> Self {
        let r = match item {
            Unit::Celcius(v) => Fahrenheit::from(v),
            Unit::Kelvin(v) => Fahrenheit::from(v),
            _ => panic!("Cannot convert")
        };
        r
    }
}

impl fmt::Display for Fahrenheit {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}°F", self.degrees)
    }
}