//! Module for the unit Date used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::{FromStr, Utf8Error};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

///Date representation used in GDTF
///Date and time corresponding to UTC +00:00 (Coordinated Universal Time):
#[derive(Debug, PartialEq, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Date {
    ///Parses a date string defined in gdtf-xml-description to a Date
    /// ```rust
    /// use gdtf_parser::utils::units::date::Date;
    /// assert_eq!(
    ///    Date::new_from_str("2021-04-01T15:23:16").unwrap(),
    ///    Date{
    ///        year: 2021,
    ///        month: 4,
    ///        day: 1,
    ///        hour: 15,
    ///        minute: 23,
    ///        second: 16
    ///    }
    /// );
    /// assert!(Date::new_from_str("Something invalid").is_err());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfDateError> {
        let value: Vec<&str> = value.split('T').collect();
        if value.len() != 2 {
            return Err(GdtfDateError {});
        }
        let date = value[0];
        let time = value[1];

        let date: Vec<&str> = date.split('-').collect();
        if date.len() != 3 {
            return Err(GdtfDateError {});
        }
        let time: Vec<&str> = time.split(':').collect();
        if time.len() != 3 {
            return Err(GdtfDateError {});
        }

        Ok(Date {
            year: u16::from_str(date[0])?,
            month: u8::from_str(date[1])?,
            day: u8::from_str(date[2])?,
            hour: u8::from_str(time[0])?,
            minute: u8::from_str(time[1])?,
            second: u8::from_str(time[2])?,
        })
    }
}

#[derive(Debug)]
/// Error that occures if the format of Date is wrong e.q. not yyyy-mm-ddThh:mm:ss
pub struct GdtfDateError {}

impl Display for GdtfDateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The Date must be formatted yyyy-mm-ddThh:mm:ss instance of T != 1"
        )
    }
}

impl Error for GdtfDateError {}

impl From<ParseIntError> for GdtfDateError {
    fn from(_: ParseIntError) -> Self {
        GdtfDateError {}
    }
}

impl From<Utf8Error> for GdtfDateError {
    fn from(_: Utf8Error) -> Self {
        GdtfDateError {}
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(DateVisitor)
    }
}

struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid Date String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match Date::new_from_str(v) {
            Ok(item) => Ok(item),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::units::date::Date;

    #[test]
    fn test_new_from_str() {
        assert_eq!(
            Date {
                year: 2021,
                month: 5,
                day: 16,
                hour: 17,
                minute: 41,
                second: 12,
            },
            Date::new_from_str("2021-05-16T17:41:12").unwrap()
        );
        assert_eq!(
            Date {
                year: 2022,
                month: 2,
                day: 8,
                hour: 17,
                minute: 12,
                second: 12,
            },
            Date::new_from_str("2022-02-08T17:12:12").unwrap()
        );
        assert!(Date::new_from_str("2021-05/16T17:41:12").is_err());
        assert!(Date::new_from_str("something invalid").is_err());
        assert!(Date::new_from_str("2021-05-1617:41:12").is_err());
        assert!(Date::new_from_str("2021/05-12T17:41:12").is_err());
        assert!(Date::new_from_str("202105-16T17:41:12").is_err());
        assert!(Date::new_from_str("").is_err());
        assert!(Date::new_from_str("2021-05-16T17:4112").is_err());
        assert!(Date::new_from_str("2021-05-16T1741:12").is_err());
    }
}
