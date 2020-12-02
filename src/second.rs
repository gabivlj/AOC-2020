use crate::file::Password;

pub fn advent_of_code_02(passwords: Vec<Password>) -> u64 {
    let mut correct_passwords = 0;

    for password in passwords {
        let mut count_characters = 0;
        let character_target = password.character;
        for character in password.string.chars() {
            if character_target == character {
                count_characters += 1;
            }
            if count_characters > password.max_times {
                break;
            }
        }
        if count_characters <= password.max_times && password.min_times <= count_characters {
            correct_passwords += 1;
        }
    }

    correct_passwords
}

pub fn advent_of_code_02_ext(passwords: Vec<Password>) -> u64 {
    let mut correct_passwords = 0;
    for password in passwords {
        let character_target = password.character;
        let p = password.string.chars();
        let mut p = p.skip(password.min_times - 1);
        let mut sum = if p.next().unwrap() == character_target {
            1
        } else {
            0
        };
        let diff = password.max_times - password.min_times - 1;
        sum = sum
            + if p.skip(diff).next().unwrap() == character_target {
                1
            } else {
                0
            };
        if sum == 1 {
            correct_passwords += 1;
        }
    }

    correct_passwords
}
