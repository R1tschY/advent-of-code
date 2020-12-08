use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn calc1(input: &str) -> usize {
    let x = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .split("\n\n")
        .map(|entry| {
            entry
                .split_ascii_whitespace()
                .map(|kv| kv.split_at(kv.find(":").unwrap()))
                .map(|(k, v)| (k, &v[1..]))
                .collect::<HashMap<&str, &str>>()
        })
        .filter(|kvs| x.iter().all(|k| kvs.contains_key(k)))
        .count()
}

fn valid_int(s: &str, at_least: u32, at_most: u32) -> Option<u32> {
    let x = s.parse::<u32>().ok()?;
    if x >= at_least && x <= at_most {
        Some(x)
    } else {
        None
    }
}

fn validate(kvs: &HashMap<&str, &str>) -> Option<()> {
    valid_int(kvs.get("byr")?, 1920, 2002)?;
    valid_int(kvs.get("iyr")?, 2010, 2020)?;
    valid_int(kvs.get("eyr")?, 2020, 2030)?;

    let hgt = kvs.get("hgt")?;
    match hgt {
        _ if hgt.ends_with("cm") => valid_int(&hgt[0..hgt.len() - 2], 150, 193)?,
        _ if hgt.ends_with("in") => valid_int(&hgt[0..hgt.len() - 2], 59, 76)?,
        _ => return None,
    };
    kvs.get("hcl").filter(|hcl| HCL_RE.is_match(hcl))?;

    let ecl_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    kvs.get("ecl").filter(|ecl| ecl_values.contains(ecl))?;

    kvs.get("pid").filter(|pid| PID_RE.is_match(pid))?;
    Some(())
}

fn calc2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|entry| {
            entry
                .split_ascii_whitespace()
                .map(|kv| kv.split_at(kv.find(":").unwrap()))
                .map(|(k, v)| (k, &v[1..]))
                .collect::<HashMap<&str, &str>>()
        })
        .filter(|kvs| validate(kvs).is_some())
        .count()
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc04.txt")));
    println!("{:?}", calc2(include_str!("aoc04.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(calc1(input), 2);
    }

    #[test]
    fn check2_1() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(calc2(input), 0);
    }

    #[test]
    fn check2_2() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(calc2(input), 4);
    }
}
