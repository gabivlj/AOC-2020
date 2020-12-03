mod file;
mod first;
mod second;
mod third;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::file;
    use crate::{
        first::{advent_of_code_01, advent_of_code_01_ext},
        second::{advent_of_code_02, advent_of_code_02_ext},
        third::{advent_of_code_03, advent_of_code_03_ext},
    };

    #[test]
    fn advent_test_01() {
        let (x, y, res) = advent_of_code_01();
        assert_eq!(x * y, res);
        assert_eq!(x + y, 2020);
        let (x, y, z, res) = advent_of_code_01_ext();
        assert_eq!(x * y * z, res);
        assert_eq!(x + y + z, 2020);
    }

    #[test]
    fn advent_test_02() {
        let result = advent_of_code_02(file::get_advent_input_02());
        assert_eq!(506, result);
        let result = advent_of_code_02_ext(file::get_advent_input_02());
        assert_eq!(443, result);
    }

    #[test]
    fn advent_test_03() {
        let input = file::get_advent_input_03();
        let result = advent_of_code_03(&input, [1, 3]);
        assert_eq!(result, 252);
        let result = advent_of_code_03_ext(&input);
        assert_eq!(result, 2608962048);
    }
}
