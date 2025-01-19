use crate::units::temperature::fahrenheit::*;
use crate::units::temperature::kelvin::*;
use crate::units::unit::Unit;

use std::fmt;

pub struct Celcius {
    pub degrees: f32,
}

impl Celcius {
    pub fn new(value: f32) -> Self {
        Celcius { degrees: value }
    }
}

impl From<Fahrenheit> for Celcius {
    fn from(item: Fahrenheit) -> Self {
        Celcius { degrees: ((item.degrees - 32.0) * 5.0/9.0) }
    }
}

impl From<Kelvin> for Celcius {
    fn from(item: Kelvin) -> Self {
        Celcius { degrees: item.degrees - 273.15 }
    }
}

impl From<Unit> for Celcius {
    fn from(item: Unit) -> Self {
        let r = match item {
            Unit::Fahrenheit(v) => Celcius::from(v),
            Unit::Kelvin(v) => Celcius::from(v),
            _ => panic!("Cannot convert")
        };
        r
    }
}

impl fmt::Display for Celcius {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}°C", self.degrees)
    }
}