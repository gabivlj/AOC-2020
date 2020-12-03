const TREE: char = '#';
const SPACE: char = '.';

pub fn advent_of_code_03(lines: &Vec<Vec<char>>, slope_patt: [usize; 2]) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut count_trees = 0;
    while i < lines.len() {
        if lines[i][j] == TREE {
            count_trees += 1;
        }
        i += slope_patt[0];
        j += slope_patt[1];
        j %= lines[0].len();
    }
    count_trees
}

pub fn advent_of_code_03_ext(lines: &Vec<Vec<char>>) -> usize {
    let slops = [
        // DOWN, RIGHT
        [1_usize, 1],
        [1, 3],
        [1, 5],
        [1, 7],
        [2, 1],
    ];
    let mut slops_sum: [usize; 5] = [0; 5];
    for (idx, slop) in slops.iter().enumerate() {
        slops_sum[idx] = advent_of_code_03(&lines, *slop);
    }
    println!("{:?}", slops_sum);
    slops_sum.iter().fold(1, |acc, &x| acc * x)
}
