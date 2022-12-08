use itertools::Itertools;

use super::{utils, Day};

pub struct Day8 {}

fn is_visible((i, j): (usize, usize), matrix: &Vec<Vec<u32>>) -> bool {
    let value = matrix.get(i).and_then(|x| x.get(j)).unwrap();
    let row = matrix
        .get(i)
        .map(|x| x.split_at(j))
        .map(|(b, a)| {
            b.iter().max().map(|x| x < value).unwrap_or(true)
                || a.iter().skip(1).max().map(|x| x < value).unwrap_or(true)
        })
        .unwrap_or_default();

    let col = Some(
        matrix
            .iter()
            .filter_map(|r| r.get(j))
            .collect_vec()
            .split_at(i),
    )
    .map(|(b, a)| {
        b.iter().max().map(|x| x < &value).unwrap_or(true)
            || a.iter().skip(1).max().map(|x| x < &value).unwrap_or(true)
    })
    .unwrap_or_default();

    row || col
}

fn scenic_score2_helper(
    (i, j): (usize, usize),
    matrix: &Vec<Vec<u32>>,
    (dx, dy): (isize, isize),
) -> u32 {
    let size = matrix.len() as isize;
    let value = &matrix
        .get(i as usize)
        .and_then(|x| x.get(j as usize))
        .unwrap();

    let mut x = j;
    let mut y = i;

    let mut score = 0;
    loop {
        let nx = x as isize + dx;
        if nx < 0 || nx == size {
            break;
        }
        let ny = y as isize + dy;
        if ny < 0 || ny == size {
            break;
        }
        x = nx as usize;
        y = ny as usize;
        score += 1;

        let current_value = &matrix.get(y).and_then(|r| r.get(x)).unwrap();
        if current_value >= value {
            break;
        }
    }
    score
}

fn scenic_score2((i, j): (usize, usize), matrix: &Vec<Vec<u32>>) -> u32 {
    let mut score = 1;

    score *= scenic_score2_helper((i, j), matrix, (0, 1));
    score *= scenic_score2_helper((i, j), matrix, (0, -1));
    score *= scenic_score2_helper((i, j), matrix, (1, 0));
    score *= scenic_score2_helper((i, j), matrix, (-1, 0));

    score
}

impl Day<u32> for Day8 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(8)?;

        let matrix: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let mut res = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix.get(0).unwrap().len() {
                res = std::cmp::max(res, scenic_score2((i, j), &matrix));
            }
        }
        Ok(res)
    }
}
