use crate::{Language, Precision};
use std::fmt;
use time::Date;

#[derive(Debug, Clone, PartialEq)]
pub struct Era {
    year: i32,
    month: u8,
    day: u8,
    precision: Precision,
    language: Language,
}

impl Era {
    /// Pass a date and precision.
    /// Depending on precision, day and/or month will be ignored.
    pub fn date(date: &Date, precision: Precision) -> Self {
        let mut ret = Era::from(date);
        ret.precision = precision;
        ret
    }

    /// Pass a year, month, and day
    pub fn day(year: i32, month: u8, day: u8) -> Self {
        Self::new(year, month, day, Precision::Day)
    }

    /// Pass a year and month
    pub fn month(year: i32, month: u8) -> Self {
        Self::new(year, month, 0, Precision::Month)
    }

    // Pass a year
    pub fn year(year: i32) -> Self {
        Self::new(year, 0, 0, Precision::Year)
    }

    /// Pass a year to set the date to the decade
    pub fn decade(year: i32) -> Self {
        Self::new(year, 0, 0, Precision::Decade)
    }

    /// Pass a year to set the date to the century
    pub fn century(year: i32) -> Self {
        Self::new(year, 0, 0, Precision::Century)
    }

    /// Pass a year to set the date to the millennium
    pub fn millennium(year: i32) -> Self {
        Self::new(year, 0, 0, Precision::Millennium)
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Pass year, month, day, and precision.
    /// Depending on precision, day and/or month will be ignored.
    pub fn new(year: i32, month: u8, day: u8, precision: Precision) -> Self {
        Self {
            year,
            month,
            day,
            precision,
            language: Language::English, // Default language
        }
    }

    fn era(&self) -> &str {
        self.language.era(self.year)
    }

    fn year_to_decade(&self) -> String {
        if self.year == 0 {
            return "0".to_string();
        }
        let year = (self.year.abs() / 10) * 10;
        let era = self.era();
        let factor = self.language.decade();
        format!("{year}{factor}{era}")
    }

    fn year_to_century(&self) -> String {
        if self.year == 0 {
            return "0".to_string();
        }
        let year = (self.year.abs() + 99) / 100;
        let ext = self.language.extension(year);
        let era = self.era();
        let factor = self.language.century();
        format!("{year}{ext} {factor}{era}")
    }

    fn year_to_millennium(&self) -> String {
        if self.year == 0 {
            return "0".to_string();
        }
        let year = (self.year.abs() + 999) / 1000;
        let ext = self.language.extension(year);
        let era = self.era();
        let factor = self.language.millennium();
        format!("{year}{ext} {factor}{era}")
    }

    fn as_string(&self) -> String {
        match self.precision {
            Precision::Millennium => self.year_to_millennium(),
            Precision::Century => self.year_to_century(),
            Precision::Decade => self.year_to_decade(),
            Precision::Year => format!("{}", self.year),
            Precision::Month => format!("{}-{:0>2}", self.year, self.month),
            Precision::Day => format!("{}-{:0>2}-{:0>2}", self.year, self.month, self.day),
        }
    }
}

impl fmt::Display for Era {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl<'a> From<&'a Date> for Era {
    fn from(nd: &'a Date) -> Self {
        Self::new(nd.year(), nd.month().into(), nd.day(), Precision::Day)
    }
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;

    #[test]
    fn test_date_renderer_day() {
        // Day
        assert_eq!(Era::day(2024, 10, 2).to_string(), "2024-10-02");
        assert_eq!(Era::day(-2024, 10, 2).to_string(), "-2024-10-02");
    }

    #[test]
    fn test_date_renderer_month() {
        assert_eq!(Era::month(2024, 10).to_string(), "2024-10");
        assert_eq!(Era::month(-2024, 10).to_string(), "-2024-10");
    }

    #[test]
    fn test_date_renderer_year() {
        assert_eq!(Era::year(2024).to_string(), "2024");
        assert_eq!(Era::year(-2024).to_string(), "-2024");
        assert_eq!(Era::year(0).to_string(), "0");
    }

    #[test]
    fn test_date_renderer_decade() {
        assert_eq!(Era::decade(910).to_string(), "910s");
        assert_eq!(Era::decade(-910).to_string(), "910s BCE");
        assert_eq!(Era::decade(-909).to_string(), "900s BCE");
        assert_eq!(Era::decade(-900).to_string(), "900s BCE");
        assert_eq!(Era::decade(0).to_string(), "0");
    }

    #[test]
    fn test_date_renderer_century() {
        assert_eq!(Era::century(10).to_string(), "1st century");
        assert_eq!(Era::century(110).to_string(), "2nd century");
        assert_eq!(Era::century(210).to_string(), "3rd century");
        assert_eq!(Era::century(310).to_string(), "4th century");
        assert_eq!(Era::century(-10).to_string(), "1st century BCE");
        assert_eq!(Era::century(-110).to_string(), "2nd century BCE");
        assert_eq!(Era::century(-210).to_string(), "3rd century BCE");
        assert_eq!(Era::century(-310).to_string(), "4th century BCE");
        assert_eq!(Era::century(-1000).to_string(), "10th century BCE");
        assert_eq!(Era::century(-901).to_string(), "10th century BCE");
        assert_eq!(Era::century(-900).to_string(), "9th century BCE");
        assert_eq!(Era::century(0).to_string(), "0");
    }

    #[test]
    fn test_date_renderer_millennium() {
        assert_eq!(Era::millennium(1).to_string(), "1st millennium");
        assert_eq!(Era::millennium(1000).to_string(), "1st millennium");
        assert_eq!(Era::millennium(1001).to_string(), "2nd millennium");
        assert_eq!(Era::millennium(2001).to_string(), "3rd millennium");
        assert_eq!(Era::millennium(3001).to_string(), "4th millennium");
        assert_eq!(Era::millennium(-1000).to_string(), "1st millennium BCE");
        assert_eq!(Era::millennium(-1001).to_string(), "2nd millennium BCE");
        assert_eq!(Era::millennium(-2001).to_string(), "3rd millennium BCE");
        assert_eq!(Era::millennium(-3001).to_string(), "4th millennium BCE");
        assert_eq!(Era::millennium(0).to_string(), "0");
    }

    #[test]
    fn test_data_render_german() {
        let dr = Era::decade(910).language(Language::German);
        assert_eq!(dr.to_string(), "910er");
        let dr = Era::decade(-910).language(Language::German);
        assert_eq!(dr.to_string(), "910er v.Chr.");

        let dr = Era::century(910).language(Language::German);
        assert_eq!(dr.to_string(), "10. Jahrhundert");
        let dr = Era::century(-910).language(Language::German);
        assert_eq!(dr.to_string(), "10. Jahrhundert v.Chr.");

        let dr = Era::millennium(910).language(Language::German);
        assert_eq!(dr.to_string(), "1. Jahrtausend");
        let dr = Era::millennium(-910).language(Language::German);
        assert_eq!(dr.to_string(), "1. Jahrtausend v.Chr.");
    }

    #[test]
    fn test_from_date() {
        let date = Date::from_calendar_date(2024, Month::September, 1).unwrap();
        let dr = Era::from(&date);
        assert_eq!(dr.to_string(), "2024-09-01");
    }

    #[test]
    fn test_new_date() {
        let date = Date::from_calendar_date(-910, Month::September, 17).unwrap();
        let dr = Era::date(&date, Precision::Millennium);
        assert_eq!(dr.to_string(), "1st millennium BCE");
        let dr = Era::date(&date, Precision::Day);
        assert_eq!(dr.to_string(), "-910-09-17");
    }
}
