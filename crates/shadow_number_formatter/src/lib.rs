#![warn(clippy::pedantic)]

use num::{Float, ToPrimitive};
use std::fmt::Write;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum NumberFormatter {
    Float(FloatFormatter),
    Percent(PercentFormatter),
}

impl Default for NumberFormatter {
    fn default() -> Self {
        NumberFormatter::Float(FloatFormatter::default())
    }
}

impl NumberFormatter {
    #[must_use]
    pub fn float_default() -> NumberFormatter {
        NumberFormatter::Float(FloatFormatter::default())
    }

    #[must_use]
    pub fn float(digits: u8) -> NumberFormatter {
        NumberFormatter::Float(FloatFormatter::new(digits))
    }

    #[must_use]
    pub fn percent_default() -> NumberFormatter {
        NumberFormatter::Percent(PercentFormatter::default())
    }

    #[must_use]
    pub fn percent(decimal_places: usize) -> NumberFormatter {
        NumberFormatter::Percent(PercentFormatter::new(decimal_places))
    }

    pub fn format<F>(&self, value: F) -> String
    where
        F: Float,
    {
        match self {
            NumberFormatter::Float(formatter) => formatter.format(value),
            NumberFormatter::Percent(formatter) => formatter.format(value),
        }
    }

    pub fn format_option<F>(&self, value: Option<F>) -> String
    where
        F: Float,
    {
        match value {
            Some(value) => self.format(value),
            None => "N/A".to_owned(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FloatFormatter {
    digits: u8,
}

impl Default for FloatFormatter {
    fn default() -> Self {
        FloatFormatter { digits: 6 }
    }
}

impl FloatFormatter {
    #[must_use]
    pub fn new(digits: u8) -> FloatFormatter {
        FloatFormatter { digits }
    }

    pub fn format<T: Float>(&self, value: T) -> String {
        let value = value.to_f64().unwrap();
        if value.is_nan() {
            return "NaN".to_owned();
        }
        if value.is_infinite() {
            if value.is_sign_positive() {
                return "inf".to_owned();
            }
            return "-inf".to_owned();
        }
        if value == 0.0 || value == -0.0 {
            return "0".to_owned();
        }
        let digits = self.digits;
        let e = value.abs().log10().floor().to_i32().unwrap();
        let n = value / 10.0f64.powi(e);
        let (n, e) = if e.unsigned_abs() >= digits.into() {
            let digits = digits as usize - digits_before_decimal(n);
            let n = format!("{:.*}", digits, n);
            (n, Some(e))
        } else {
            let digits = digits as usize - digits_before_decimal(value);
            let n = format!("{:.*}", digits, value);
            (n, None)
        };
        let mut string = n;
        if string.contains('.') {
            string.truncate(string.trim_end_matches('0').len());
            string.truncate(string.trim_end_matches('.').len());
        }
        if let Some(e) = e {
            string.push('e');
            write!(&mut string, "{e}").unwrap(); 
        }
        string
    }

    pub fn format_option<F>(&self, value: Option<F>) -> String
    where
        F: Float,
    {
        match value {
            Some(value) => self.format(value),
            None => "N/A".to_owned(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PercentFormatter {
    decimal_places: usize,
}

impl Default for PercentFormatter {
    fn default() -> Self {
        PercentFormatter { decimal_places: 2 }
    }
}

impl PercentFormatter {
    #[must_use]
    pub fn new(decimal_places: usize) -> PercentFormatter {
        PercentFormatter { decimal_places }
    }

    pub fn format<T: Float>(&self, value: T) -> String {
        if (value - T::one()).abs() <= T::epsilon() {
            "100%".to_owned()
        } else {
            format!(
                "{:.1$}%",
                value.to_f64().unwrap() * 100.0,
                self.decimal_places
            )
        }
    }

    pub fn format_option<F>(&self, value: Option<F>) -> String
    where
        F: Float,
    {
        match value {
            Some(value) => self.format(value),
            None => "N/A".to_owned(),
        }
    }
}

pub fn format_float<T: Float>(value: T) -> String {
    FloatFormatter::default().format(value)
}

pub fn format_float_with_digits<T: Float>(value: T, digits: u8) -> String {
    FloatFormatter::new(digits).format(value)
}

pub fn format_option_float<T: Float>(value: Option<T>) -> String {
    FloatFormatter::default().format_option(value)
}

pub fn format_percent<T: Float>(value: T) -> String {
    PercentFormatter::default().format(value)
}

pub fn format_option_percent<T: Float>(value: Option<T>) -> String {
    PercentFormatter::default().format_option(value)
}

fn digits_before_decimal(value: f64) -> usize {
    let value = value.trunc().abs();
    if value == 0.0 {
        0
    } else {
        value.log10().floor().to_usize().unwrap() + 1
    }
}
