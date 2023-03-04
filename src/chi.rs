use chrono::NaiveDate;

#[derive(PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
}

pub trait Chi {
    /// Construct a Chi from a string, validating the modulus 11 check digit at the end of the
    /// value.
    fn from(string: &'static str) -> Self;

    /// Extract date of birth from Community Health Index (CHI) number
    ///
    /// The Community Health Index (CHI) is a population register used in Scotland
    /// for health care purposes. The CHI number uniquely identifies a person on the
    /// index. Note `cutoff_2000`. As CHI has only a two digit year, you need to
    /// specify whether year is 1900s or 2000s. The cut-off determines the year
    /// before that number is considered 2000s i.e. at cutoff_2000 = 20, "18" is
    /// considered 2018, rather than 1918.
    fn date_of_birth(&self, cutoff_2000: u32) -> NaiveDate;

    fn gender(&self) -> Gender;
}

impl Chi for &'static str {
    fn from(string: &'static str) -> Self {
        assert!(string.len() == 10, "CHI should be 10 characters long");
        let seq = (2..=10).rev();

        let sum: u32 = seq
            .zip(string[0..10].chars())
            .map(|(n, c)| n as u32 * (c as u8 - '0' as u8) as u32)
            .sum();
        let modulus = 11 - (sum % 11);
        let corrected = if modulus == 11 { 0 } else { modulus };
        assert!(
            corrected == (string.chars().nth(9).unwrap() as u8 - '0' as u8) as u32,
            "CHI last digit must pass modulus 11 test"
        );

        string
    }

    fn date_of_birth(&self, cutoff_2000: u32) -> NaiveDate {
        let day = self[0..2].parse().unwrap();
        let month = self[2..4].parse().unwrap();
        let year_end: i32 = self[4..6].parse().unwrap();
        let year = if year_end > cutoff_2000 as i32 {
            1900 + year_end
        } else {
            2000 + year_end
        };
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
        // NaiveDate::parse_from_str(&self[0..6], "%d%m%y").unwrap()
    }

    fn gender(&self) -> Gender {
        match (self.chars().nth(8).unwrap() as u32 - '0' as u32) % 2 {
            0 => Gender::Female,
            _ => Gender::Male,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::chi::{Chi, Gender};

    #[test]
    fn valid_date_of_birth() {
        let x: &'static str = Chi::from("1811431232");
        assert_eq!(
            x.date_of_birth(23),
            NaiveDate::from_ymd_opt(1943, 11, 18).unwrap()
        );
        let y: &'static str = Chi::from("1304236366");
        assert_eq!(
            y.date_of_birth(23),
            NaiveDate::from_ymd_opt(2023, 4, 13).unwrap()
        );
        let z: &'static str = Chi::from("1304496368");
        assert_eq!(
            z.date_of_birth(23),
            NaiveDate::from_ymd_opt(1949, 4, 13).unwrap()
        );
    }

    #[test]
    fn gender() {
        let x: &'static str = Chi::from("1811431232");
        assert_eq!(x.gender(), Gender::Male);
        let y: &'static str = Chi::from("1304236366");
        assert_eq!(y.gender(), Gender::Female);
        let z: &'static str = Chi::from("1304496368");
        assert_eq!(z.gender(), Gender::Female);
    }

    #[test]
    #[should_panic]
    fn invalid_date_of_birth() {
        let x: &'static str = Chi::from("1009701234");
        assert_eq!(
            x.date_of_birth(23),
            NaiveDate::from_ymd_opt(1943, 11, 18).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn wrong_length() {
        let _x: &'static str = Chi::from("100970123");
    }
}
