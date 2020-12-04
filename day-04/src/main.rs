use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn split_once(s: &str) -> (&str, &str) {
    let mut splitter = s.splitn(2, ':');
    (splitter.next().unwrap(), splitter.next().unwrap())
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    exp_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn valid_1(&self) -> bool {
        let required_ints = [self.birth_year, self.issue_year, self.exp_year];
        let required_strs = [
            self.height.as_ref(),
            self.hair_color.as_ref(),
            self.eye_color.as_ref(),
            self.passport_id.as_ref(),
        ];
        let has_required_ints =
            required_ints.iter().filter_map(|v| *v).count() == required_ints.len();
        let has_required_strs =
            required_strs.iter().filter_map(|v| *v).count() == required_strs.len();

        has_required_ints && has_required_strs
    }

    fn valid(&self) -> bool {
        let required_ints = [self.birth_year, self.issue_year, self.exp_year];
        let required_strs = [
            self.height.as_ref(),
            self.hair_color.as_ref(),
            self.eye_color.as_ref(),
            self.passport_id.as_ref(),
        ];
        let has_required_ints =
            required_ints.iter().filter_map(|v| *v).count() == required_ints.len();
        let has_required_strs =
            required_strs.iter().filter_map(|v| *v).count() == required_strs.len();

        if !(has_required_ints && has_required_strs) {
            return false;
        }

        let birth_year_valid = self.birth_year.unwrap() >= 1920 && self.birth_year.unwrap() <= 2002;
        let issue_year_valid = self.issue_year.unwrap() >= 2010 && self.issue_year.unwrap() <= 2020;
        let exp_year_valid = self.exp_year.unwrap() >= 2020 && self.exp_year.unwrap() <= 2030;

        let detail_valid = self.validate_detail(
            self.height.as_ref().unwrap(),
            self.hair_color.as_ref().unwrap(),
            self.eye_color.as_ref().unwrap(),
            self.passport_id.as_ref().unwrap(),
        );

        has_required_ints
            && has_required_strs
            && birth_year_valid
            && issue_year_valid
            && exp_year_valid
            && detail_valid
    }

    fn validate_detail(
        &self,
        height: &str,
        hair_color: &str,
        eye_color: &str,
        passport_id: &str,
    ) -> bool {
        lazy_static! {
            static ref HEIGHT_INT_RE: Regex = Regex::new(r"\d+").unwrap();
            static ref HEIGHT_RE: Regex = Regex::new(r"\d+(cm|in)").unwrap();
            static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}").unwrap();
            static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        let height_int = HEIGHT_INT_RE
            .captures(height)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let height_valid = HEIGHT_RE.is_match(height)
            && if height.contains("cm") {
                height_int >= 150 && height_int <= 193
            } else {
                height_int >= 59 && height_int <= 76
            };

        let hair_color_valid = HAIR_RE.is_match(hair_color);

        let eye_color_valid = EYE_RE.is_match(eye_color);

        let passport_id_valid = PASSPORT_ID_RE.is_match(passport_id);

        height_valid && hair_color_valid && eye_color_valid && passport_id_valid
    }
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();

        for item in s.split(&[' ', '\n'][..]) {
            match split_once(item) {
                ("byr", byr) => passport.birth_year = byr.parse::<i32>().ok(),
                ("iyr", iyr) => passport.issue_year = iyr.parse::<i32>().ok(),
                ("eyr", eyr) => passport.exp_year = eyr.parse::<i32>().ok(),
                ("hgt", hgt) => passport.height = Some(hgt.to_string()),
                ("hcl", hcl) => passport.hair_color = Some(hcl.to_string()),
                ("ecl", ecl) => passport.eye_color = Some(ecl.to_string()),
                ("pid", pid) => passport.passport_id = Some(pid.to_string()),
                ("cid", cid) => passport.country_id = Some(cid.to_string()),
                _ => {}
            };
        }

        Ok(passport)
    }
}

fn parse_passports(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .map(Passport::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn main() {
    let input = fs::read_to_string("./day-04/input.txt").unwrap();

    let passports = parse_passports(&input);

    println!(
        "Part 1: {}",
        passports.iter().filter(|p| p.valid_1()).count()
    );

    println!("Part 2: {}", passports.iter().filter(|p| p.valid()).count());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample_1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passports = parse_passports(sample_1);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports.iter().filter(|p| p.valid_1()).count(), 2);
    }

    #[test]
    fn test_sample_2() {
        let sample_invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports = parse_passports(sample_invalid);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports.iter().filter(|p| p.valid()).count(), 0);

        let sample_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports = parse_passports(sample_valid);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports.iter().filter(|p| p.valid()).count(), 4);
    }
}
