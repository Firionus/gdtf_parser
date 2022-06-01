//! Module for the unit Rotation used in GDTF

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

///The Rotation matrix consists of 3*3 floats. Stored as row-major matrix, i.e. each row of the matrix is stored as a 3-component vector. Mathematical definition of the matrix is column-major, i.e. the matrix rotation is stored in the three columns. Metric system, right-handed Cartesian coordinates XYZ
#[derive(Debug, PartialEq, Clone)]
pub struct Rotation(pub [[f32; 3]; 3]);

impl Rotation {
    ///Parses a string defined in gdtf-xml-description to Rotation
    /// ```rust
    /// use gdtf_parser::utils::units::rotation::Rotation;
    ///
    /// assert_eq!(
    ///             Rotation::new_from_str("{1.1,1.2,1.3}{2.1,2.2,2.3}{3.1,3.2,3.3}").unwrap(),
    ///             Rotation([
    ///                 [1.1, 1.2, 1.3],
    ///                 [2.1, 2.2, 2.3],
    ///                 [3.1, 3.2, 3.3],
    ///             ])
    ///         );
    /// assert!(Rotation::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3}").is_err());
    /// assert!(Rotation::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1}").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfRotationError> {
        use GdtfRotationError as E;
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"[-+]?([0-9]*\.[0-9]+|[0-9]+)").unwrap();
        }
        let mut caps = REGEX.captures_iter(s);

        let m11 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m12 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m13 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        let m21 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m22 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m23 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        let m31 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m32 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m33 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        if caps.next().is_some() {
            return Err(GdtfRotationError::WrongFormatError);
        }

        Ok(Self([[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]))
    }
}

#[derive(Debug)]
pub enum GdtfRotationError {
    ParseFloatError(ParseFloatError),
    WrongFormatError,
}

impl Display for GdtfRotationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GdtfRotationError::ParseFloatError(e) => write!(f, "Rotation must be formatted in {{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}. {}", e),
            GdtfRotationError::WrongFormatError => write!(f, "Rotation must be formatted in {{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}")
        }
    }
}

impl Error for GdtfRotationError {}

impl From<ParseFloatError> for GdtfRotationError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl<'de> Deserialize<'de> for Rotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(RotationVisitor)
    }
}

struct RotationVisitor;

impl<'de> Visitor<'de> for RotationVisitor {
    type Value = Rotation;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid Rotation String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match Rotation::new_from_str(v) {
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
    use crate::utils::units::rotation::{GdtfRotationError, Rotation};

    #[test]
    fn test_new_from_str() -> Result<(), GdtfRotationError> {
        assert_eq!(
            Rotation::new_from_str("{1.1,1.2,1.3}{2.1,2.2,2.3}{3.1,3.2,3.3}").unwrap(),
            Rotation([[1.1, 1.2, 1.3], [2.1, 2.2, 2.3], [3.1, 3.2, 3.3], ])
        );
        assert_eq!(
            Rotation::new_from_str("{1.1,1.2,1.3},{2.1,2.2,2.3},{3.1,3.2,3.3}").unwrap(),
            Rotation([[1.1, 1.2, 1.3], [2.1, 2.2, 2.3], [3.1, 3.2, 3.3], ])
        );
        assert!(
            Rotation::new_from_str("{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3}").is_err()
        );
        assert!(Rotation::new_from_str(
            "{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},4.1"
        )
            .is_err());
        Ok(())
    }
}
