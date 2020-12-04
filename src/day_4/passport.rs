use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PassportInfo {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(u32),
}

impl PassportInfo {
    fn new(single_info: &str) -> PassportInfo {
        let info_type = &single_info[0..3];
        let info_value = &single_info[4..];

        let error_info = format!("Error for Info: {:?}", single_info);

        match info_type {
            "byr" => PassportInfo::BirthYear(info_value.parse().expect(error_info.as_str())),
            "iyr" => PassportInfo::IssueYear(info_value.parse().expect(error_info.as_str())),
            "eyr" => PassportInfo::ExpirationYear(info_value.parse().expect(error_info.as_str())),
            "hgt" => PassportInfo::Height(String::from(info_value)),
            "hcl" => PassportInfo::HairColor(String::from(info_value)),
            "ecl" => PassportInfo::EyeColor(String::from(info_value)),
            "pid" => PassportInfo::PassportId(String::from(info_value)),
            "cid" => PassportInfo::CountryId(info_value.parse().expect(error_info.as_str())),
            _ => panic!("Unknown PassportInfo Type: {:?}", info_type),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            PassportInfo::BirthYear(year) => (1920..2003).contains(year),
            PassportInfo::IssueYear(year) => (2010..2021).contains(year),
            PassportInfo::ExpirationYear(year) => (2020..2031).contains(year),
            PassportInfo::Height(info) => {
                if info.contains("cm") {
                    let value: u32 = info.replace("cm", "").parse().unwrap_or(0);

                    (150..194).contains(&value)
                } else if info.contains("in") {
                    let value: u32 = info.replace("in", "").parse().unwrap_or(0);

                    (59..77).contains(&value)
                } else {
                    false
                }
            }
            PassportInfo::HairColor(info) => {
                let size_valid = info.len() == 7;
                let valid_start = info.starts_with('#');
                let valid_rest = info[1..].chars().all(|c| c.is_digit(16));

                size_valid && valid_start && valid_rest
            }
            PassportInfo::EyeColor(info) => {
                info == "amb"
                    || info == "blu"
                    || info == "brn"
                    || info == "gry"
                    || info == "grn"
                    || info == "hzl"
                    || info == "oth"
            }
            PassportInfo::PassportId(info) => {
                let size_valid = info.len() == 9;
                let characters_valid = info.chars().all(|c| c.is_digit(10));

                size_valid && characters_valid
            }
            PassportInfo::CountryId(_) => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    info: HashSet<PassportInfo>,
}

impl Passport {
    pub fn new(all_info: &str) -> Passport {
        Passport {
            info: all_info
                .split(' ')
                .map(|split_value| PassportInfo::new(split_value))
                .collect(),
        }
    }

    pub fn contains_required_fields(&self) -> bool {
        let mut has_birth_year = false;
        let mut has_issue_year = false;
        let mut has_expiration_year = false;
        let mut has_height = false;
        let mut has_hair_color = false;
        let mut has_eye_color = false;
        let mut has_passport_id = false;

        // PassportInfo::CountryId is optional.
        for passport_info in &self.info {
            match passport_info {
                PassportInfo::BirthYear(_) => has_birth_year = true,
                PassportInfo::IssueYear(_) => has_issue_year = true,
                PassportInfo::ExpirationYear(_) => has_expiration_year = true,
                PassportInfo::Height(_) => has_height = true,
                PassportInfo::HairColor(_) => has_hair_color = true,
                PassportInfo::EyeColor(_) => has_eye_color = true,
                PassportInfo::PassportId(_) => has_passport_id = true,
                PassportInfo::CountryId(_) => {}
            };
        }

        has_birth_year
            && has_issue_year
            && has_expiration_year
            && has_height
            && has_hair_color
            && has_eye_color
            && has_passport_id
    }

    pub fn contains_valid_info(&self) -> bool {
        self.info
            .iter()
            .all(|passport_info| passport_info.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EYE_COLOR: &str = "ecl:gry";
    const BIRTH_YEAR: &str = "byr:1937";

    const VALID_HEIGHT_1: &str = "150cm";
    const VALID_HEIGHT_2: &str = "76in";
    const INVALID_HEIGHT_1: &str = "179";
    const INVALID_HEIGHT_2: &str = "in";

    const VALID_BIRTH_YEAR: u32 = 1920;
    const INVALID_BIRTH_YEAR: u32 = 2003;

    const VALID_HAIR_COLOR: &str = "#123abc";
    const INVALID_HAIR_COLOR_1: &str = "#123abz";
    const INVALID_HAIR_COLOR_2: &str = "123abc";

    const VALID_EYE_COLOR: &str = "gry";
    const INVALID_EYE_COLOR: &str = "wat";

    const VALID_PASSPORT_ID: &str = "000000001";
    const INVALID_PASSPORT_ID_1: &str = "#00000001";
    const INVALID_PASSPORT_ID_2: &str = "0000000001";

    const VALID_PASSPORT_INFO: &str =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    const VALID_PASSPORT_INFO_MISSING_CID: &str =
        "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
    const INVALID_PASSPORT_INFO: &str =
        "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";
    const INVALID_PASSPORT_INFO_2: &str =
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";

    #[test]
    fn test_passport_info_new() {
        let result_1 = PassportInfo::new(EYE_COLOR);
        let result_2 = PassportInfo::new(BIRTH_YEAR);
        let expected_1 = PassportInfo::EyeColor(String::from("gry"));
        let expected_2 = PassportInfo::BirthYear(1937);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_passport_info_is_valid() {
        let valid_height_1 = PassportInfo::Height(String::from(VALID_HEIGHT_1));
        let valid_height_2 = PassportInfo::Height(String::from(VALID_HEIGHT_2));
        let invalid_height_1 = PassportInfo::Height(String::from(INVALID_HEIGHT_1));
        let invalid_height_2 = PassportInfo::Height(String::from(INVALID_HEIGHT_2));

        let valid_birth_year = PassportInfo::BirthYear(VALID_BIRTH_YEAR);
        let invalid_birth_year = PassportInfo::BirthYear(INVALID_BIRTH_YEAR);

        let valid_hair_color = PassportInfo::HairColor(String::from(VALID_HAIR_COLOR));
        let invalid_hair_color_1 = PassportInfo::HairColor(String::from(INVALID_HAIR_COLOR_1));
        let invalid_hair_color_2 = PassportInfo::HairColor(String::from(INVALID_HAIR_COLOR_2));

        let valid_eye_color = PassportInfo::EyeColor(String::from(VALID_EYE_COLOR));
        let invalid_eye_color = PassportInfo::EyeColor(String::from(INVALID_EYE_COLOR));

        let valid_passport = PassportInfo::PassportId(String::from(VALID_PASSPORT_ID));
        let invalid_passport_1 = PassportInfo::PassportId(String::from(INVALID_PASSPORT_ID_1));
        let invalid_passport_2 = PassportInfo::PassportId(String::from(INVALID_PASSPORT_ID_2));

        assert!(valid_height_1.is_valid());
        assert!(valid_height_2.is_valid());
        assert!(!invalid_height_1.is_valid());
        assert!(!invalid_height_2.is_valid());

        assert!(valid_birth_year.is_valid());
        assert!(!invalid_birth_year.is_valid());

        assert!(valid_hair_color.is_valid());
        assert!(!invalid_hair_color_1.is_valid());
        assert!(!invalid_hair_color_2.is_valid());

        assert!(valid_eye_color.is_valid());
        assert!(!invalid_eye_color.is_valid());

        assert!(valid_passport.is_valid());
        assert!(!invalid_passport_1.is_valid());
        assert!(!invalid_passport_2.is_valid());
    }

    #[test]
    fn test_passport_new() {
        let result = Passport::new(VALID_PASSPORT_INFO);

        let expected = Passport {
            info: [
                PassportInfo::EyeColor(String::from("gry")),
                PassportInfo::PassportId(String::from("860033327")),
                PassportInfo::ExpirationYear(2020),
                PassportInfo::HairColor(String::from("#fffffd")),
                PassportInfo::BirthYear(1937),
                PassportInfo::IssueYear(2017),
                PassportInfo::CountryId(147),
                PassportInfo::Height(String::from("183cm")),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_passport_contains_required_fields() {
        let passport_1 = Passport::new(VALID_PASSPORT_INFO);
        let passport_2 = Passport::new(VALID_PASSPORT_INFO_MISSING_CID);
        let passport_3 = Passport::new(INVALID_PASSPORT_INFO);
        let passport_4 = Passport::new(INVALID_PASSPORT_INFO_2);

        assert!(passport_1.contains_required_fields());
        assert!(passport_2.contains_required_fields());
        assert!(!passport_3.contains_required_fields());
        assert!(!passport_4.contains_required_fields());
    }
}
