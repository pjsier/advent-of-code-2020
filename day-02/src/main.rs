use std::fs;
use std::str::FromStr;

struct PasswordPolicy {
    pub character: char,
    pub min: usize,
    pub max: usize,
}

impl PasswordPolicy {
    fn password_passes(&self, password: &str) -> bool {
        let char_count = password.chars().filter(|c| *c == self.character).count();
        char_count >= self.min && char_count <= self.max
    }

    fn password_passes_alt(&self, password: &str) -> bool {
        (password.chars().nth(self.min - 1).unwrap_or(' ') == self.character)
            ^ (password.chars().nth(self.max - 1).unwrap_or(' ') == self.character)
    }
}

impl FromStr for PasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<PasswordPolicy, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();
        if v.len() != 2 {
            return Err(());
        }
        let (min_max, character) = (v[0], v[1]);

        let min_max_v: Vec<&str> = min_max.split('-').collect();
        if min_max_v.len() != 2 {
            return Err(());
        }
        let (min_str, max_str) = (min_max_v[0], min_max_v[1]);
        Ok(PasswordPolicy {
            character: character.chars().next().unwrap(),
            min: min_str.parse::<usize>().unwrap(),
            max: max_str.parse::<usize>().unwrap(),
        })
    }
}

fn split_line(line: String) -> Option<(PasswordPolicy, String)> {
    let split_line: Vec<&str> = line.split(": ").collect();
    if split_line.len() != 2 {
        return None;
    }
    let (policy_str, password_str) = (split_line[0], split_line[1]);
    let policy = PasswordPolicy::from_str(policy_str).unwrap();
    Some((policy, password_str.to_string()))
}

fn password_line_passes(line: String) -> bool {
    if let Some((policy, pw)) = split_line(line) {
        return policy.password_passes(&pw);
    }
    false
}

fn password_line_passes_alt(line: String) -> bool {
    if let Some((policy, pw)) = split_line(line) {
        return policy.password_passes_alt(&pw);
    }
    false
}

fn main() {
    let input = fs::read_to_string("./day-02/input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();

    println!(
        "Part 1 {}",
        lines
            .clone()
            .iter()
            .filter(|l| password_line_passes(l.to_string()))
            .count()
    );
    println!(
        "Part 2 {}",
        lines
            .clone()
            .iter()
            .filter(|l| password_line_passes_alt(l.to_string()))
            .count()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let p = PasswordPolicy::from_str("1-3 a").unwrap();
        assert_eq!(p.min, 1);
        assert_eq!(p.max, 3);
        assert_eq!(p.character, 'a');
    }

    #[test]
    fn test_password_lines_sample() {
        let password_lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let password_match_count = password_lines
            .iter()
            .filter(|l| password_line_passes(l.to_string()))
            .count();
        assert_eq!(password_match_count, 2);
    }

    #[test]
    fn test_password_lines_alt_sample() {
        let password_lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let password_match_count = password_lines
            .iter()
            .filter(|l| password_line_passes_alt(l.to_string()))
            .count();
        assert_eq!(password_match_count, 1);
    }

    #[test]
    fn test_password_alt_check() {
        let password_line = String::from("2-3 h: hhhh");
        assert!(!password_line_passes_alt(password_line));
    }
}
