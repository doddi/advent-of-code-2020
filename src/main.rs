use std::fs::File;
use std::io::{ self, BufRead };
use regex::Regex;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn main() {
    let map = read_lines();

    let mut count = 0;
    let mut read: u128 = 0;
    let mut line_count: u128 = map.len() as u128;

    while read < line_count-1 {
        let passport: Passport = get_passport(&map, &mut read);
        let valid: bool = passport_is_valid(passport);
        if valid {
            count += 1;
        }

    }
    println!("{}", count);
}

fn passport_is_valid(passport: Passport) -> bool {
    if checkYear(passport.byr.to_string(), 1920, 2002) &&
        checkYear(passport.iyr, 2010, 2020) &&
        checkYear(passport.eyr, 2020, 2030) &&
        checkHeight(passport.hgt) &&
        checkHairColour(passport.hcl) &&
        checkEyeColour(passport.ecl) &&
        checkPassportId(passport.pid) {
        return true;
    }
    return false;
}

fn checkPassportId(passport_id: String) -> bool {
    let re = Regex::new(r"^[0-9]+$").unwrap();
    if re.is_match(passport_id.as_str()) && passport_id.len()==9 {
        return true;
    }

    return false;
}

fn checkEyeColour(eye: String) -> bool {
    let test = eye.as_str();
    return match test {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn checkHairColour(colour: String) -> bool {
    if colour.starts_with("#") && colour.len()==7 {
        let value = colour.replace("#", "");

        let re1 = Regex::new(r"^[a-f0-9]+$").unwrap();
        if re1.is_match(value.as_str()) {
            return true;
        }
        //
        // let re2 = Regex::new(r"^[0-9]+$").unwrap();
        // if re2.is_match(value.as_str()) {
        //     return true;
        // }
    }

    return false;
}

fn checkYear(value: String, min: u32, max: u32) -> bool {
    if value.len() != 4 {
        return false;
    }

    let year = value.parse::<u32>();

    if year.is_ok() {
        let val = year.unwrap();
        if val >= min &&
            val <= max {
            return true;
        }
    }
    return false;
}

fn checkHeight(value: String) -> bool {
    if value.ends_with("cm") {
        let hcm = value.replace("cm", "");
        let height = hcm.parse::<u32>();
        if height.is_ok() {
            let test = height.unwrap();
            if test >= 150 && test <= 193 {
                return true;
            }
        }
    } else if value.ends_with("in") {
        let hin = value.replace("in", "");
        let height = hin.parse::<u32>();
        if height.is_ok() {
            let test = height.unwrap();
            if test >= 59 && test <= 76 {
                return true;
            }
        }
    }
    return false;
}

fn get_passport(map: &Vec<String>, read: &mut u128) -> Passport {
    let mut finished: bool = false;
    let mut context : Vec<&String> = vec![];

    while finished == false {
        let line = map.get(*read as usize).unwrap();
        *read += 1;

        if line.len() == 0 || *read == map.len() as u128 {
            finished = true;
            return parse_passport(context.clone());
        } else {
            context.push(line);
        }
    }

    return init_passport();
}

fn parse_passport(lines: Vec<&String>) -> Passport {
    let mut passport = init_passport();
    for line in lines {
        let items = line.split(" ").collect::<Vec<_>>();

        for item in items {
            let key_value = item.split(":").collect::<Vec<_>>();

            match key_value[0] {
                "byr" => passport.byr = key_value[1].to_string(),
                "iyr" => passport.iyr = key_value[1].to_string(),
                "eyr" => passport.eyr = key_value[1].to_string(),
                "hgt" => passport.hgt = key_value[1].to_string(),
                "hcl" => passport.hcl = key_value[1].to_string(),
                "ecl" => {
                    if !passport.ecl.eq("") {
                        passport.ecl = "".to_string();
                    } else {
                        passport.ecl = key_value[1].to_string()
                    }
                },
                "pid" => passport.pid = key_value[1].to_string(),
                "cid" => passport.cid = key_value[1].to_string(),
                _ => {}
            }
        }
    }

    return passport;
}

fn init_passport()  -> Passport{
    return Passport {
        byr: "".to_string(),
        iyr: "".to_string(),
        eyr: "".to_string(),
        hgt: "".to_string(),
        hcl: "".to_string(),
        ecl: "".to_string(),
        pid: "".to_string(),
        cid: "".to_string()
    };
}


fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
        lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}
