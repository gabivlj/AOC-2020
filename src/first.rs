use crate::file;

pub fn advent_of_code_01() -> (i64, i64, i64) {
    use std::collections::HashMap;
    let input = file::get_advent_input_01();
    let mut map: HashMap<i64, i64> = HashMap::new();
    for number in input {
        if map.contains_key(&number) {
            return (number, (2020 - number), number * (2020 - number));
        }
        map.insert(2020 - number, 0);
    }
    (0, 0, 0)
}

pub fn advent_of_code_01_ext() -> (i64, i64, i64, i64) {
    use std::collections::HashMap;
    let input = file::get_advent_input_01();
    let mut map: HashMap<i64, [usize; 2]> = HashMap::new();
    for (idx, number) in input.iter().enumerate() {
        if map.contains_key(number) {
            let idxs = map[number];
            if idxs.contains(&idx) {
                continue;
            }
            return (
                *number,
                input[idxs[0]],
                input[idxs[1]],
                number * input[idxs[0]] * input[idxs[1]],
            );
        }
        for (idx2, number2) in input.iter().enumerate() {
            if idx2 == idx {
                continue;
            }
            map.insert(2020 - (number2 + number), [idx, idx2]);
        }
    }
    (0, 0, 0, 0)
}
