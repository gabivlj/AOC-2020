use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_split_input<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut buffer = String::with_capacity(100);
    File::open(path)
        .expect("it to exist")
        .read_to_string(&mut buffer)
        .expect("wow didn't work");
    buffer.split('\n').map(|str| str.to_string()).collect()
}

pub fn get_advent_input_01() -> Vec<i64> {
    file_split_input("./input_01.txt")
        .iter()
        .map(|str| str.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug)]
pub struct Password {
    pub min_times: usize,
    pub max_times: usize,
    pub character: char,
    pub string: String,
}

impl From<&String> for Password {
    fn from(string: &String) -> Self {
        let mut iterator = string.split('-');
        let min = iterator.next().unwrap().parse::<usize>().unwrap();
        let rest = iterator.next().unwrap();
        let mut iterator = rest.split(' ');
        let max = iterator.next().unwrap().parse::<usize>().unwrap();
        let character = iterator
            .next()
            .unwrap()
            .split(':')
            .next()
            .unwrap()
            .chars()
            .next();
        let password = iterator.next().unwrap();
        Self {
            string: password.to_string(),
            character: character.unwrap(),
            min_times: min,
            max_times: max,
        }
    }
}

pub fn get_advent_input_02() -> Vec<Password> {
    file_split_input("./input_02.txt")
        .iter()
        .map(|string| Password::from(string))
        .collect()
}
