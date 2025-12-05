use crate::utils::file_utils;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct Instruction {
    letter: char,
    number: i16,
}

impl TryFrom<String> for Instruction {
    type Error = String;

    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty".to_string());
        }

        let letter = s.chars().next().ok_or("instruction: no letter.")?;
        let number = s[1..]
            .parse::<i16>()
            .map_err(|_| "instruction: bad number.")?;

        Ok(Instruction { letter, number })
    }
}

pub fn run() {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let file_path = base_path.join("input.txt");
    let mut rotation = 50;
    let mut zero_count = 0;

    if let Ok(lines) = file_utils::read_file(&file_path) {
        for line in lines {
            if let Ok(ins) = line {
                match serde_json::from_str::<Instruction>(&format!("\"{}\"", ins)) {
                    Ok(val) => {
                        zero_count += (rotation + val.number).div_euclid(100);

                        match val.letter {
                            'L' => rotation -= val.number,
                            'R' => rotation += val.number,
                            _ => {}
                        }

                        rotation = rotation.rem_euclid(100);

                        if rotation == 0 {
                            zero_count += 1;
                        }
                    }
                    Err(e) => println!("Error parsing line '{}': {}", ins, e),
                }
            }
        }
    }

    println!("password: {}", zero_count);
}
