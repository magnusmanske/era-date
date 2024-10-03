use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Precision {
    Day = 11,
    Month = 10,
    Year = 9,
    Decade = 8,
    Century = 7,
    Millennium = 6,
}

impl Precision {
    fn as_u8(&self) -> u8 {
        match self {
            Precision::Day => 11,
            Precision::Month => 10,
            Precision::Year => 9,
            Precision::Decade => 8,
            Precision::Century => 7,
            Precision::Millennium => 6,
        }
    }
}

impl fmt::Display for Precision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl TryFrom<u8> for Precision {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Precision::Millennium),
            7 => Ok(Precision::Century),
            8 => Ok(Precision::Decade),
            9 => Ok(Precision::Year),
            10 => Ok(Precision::Month),
            11 => Ok(Precision::Day),
            _ => Err("Unsupported precision value {value}; values 6-11 are supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_as_u8() {
        assert_eq!(Precision::Day.as_u8(), 11);
        assert_eq!(Precision::Month.as_u8(), 10);
        assert_eq!(Precision::Year.as_u8(), 9);
        assert_eq!(Precision::Decade.as_u8(), 8);
        assert_eq!(Precision::Century.as_u8(), 7);
        assert_eq!(Precision::Millennium.as_u8(), 6);
    }

    #[test]
    fn test_precision_display() {
        assert_eq!(format!("{}", Precision::Day), "11");
        assert_eq!(format!("{}", Precision::Month), "10");
        assert_eq!(format!("{}", Precision::Year), "9");
        assert_eq!(format!("{}", Precision::Decade), "8");
        assert_eq!(format!("{}", Precision::Century), "7");
        assert_eq!(format!("{}", Precision::Millennium), "6");
    }

    #[test]
    fn test_precision_from_u8() {
        assert_eq!(Precision::try_from(11).unwrap(), Precision::Day);
        assert_eq!(Precision::try_from(10).unwrap(), Precision::Month);
        assert_eq!(Precision::try_from(9).unwrap(), Precision::Year);
        assert_eq!(Precision::try_from(8).unwrap(), Precision::Decade);
        assert_eq!(Precision::try_from(7).unwrap(), Precision::Century);
        assert_eq!(Precision::try_from(6).unwrap(), Precision::Millennium);
        assert!(Precision::try_from(5).is_err());
        assert!(Precision::try_from(12).is_err());
    }
}
