A Rust [crate](https://crates.io/crates/era-date) to generate text descriptions for date precisions, from day (ISO) to millennium.
Currently supports English and German output.

# Usage
```rust
use time::Date;
use era_date::*;

// Various precisions
assert_eq!(Era::day(-910, 9, 17).to_string(), "-910-09-17");
assert_eq!(Era::month(2024, 10).to_string(), "2024-10");
assert_eq!(Era::year(2024).to_string(), "2024");
assert_eq!(Era::decade(-910).to_string(), "910s BCE");
assert_eq!(Era::century(310).to_string(), "4th century");
assert_eq!(Era::millennium(-2001).to_string(), "3rd millennium BCE");

// Use other output languages
let dr = Era::century(-910).language(Language::German);
assert_eq!(dr.to_string(), "10. Jahrhundert v.Chr.");

// Convert from `Date`
let date = Date::from_calendar_date(-910, Month::September, 17).unwrap();

// From `Date` to Era uses day precision
let dr = Era::from(&date);
assert_eq!(dr.to_string(), "-910-09-17");

// Convert `Date` to millennium
let dr = Era::date(&date, Precision::Millennium);
assert_eq!(dr.to_string(), "1st millennium BCE");
```
