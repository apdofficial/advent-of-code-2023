use itertools::Itertools;

fn hash(data: &str) -> usize {
    data.chars().fold(0, |mut acc, c|{
        acc += c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

pub fn part1(input: &str) -> i64 {
    input.replace("\n", "").split(',').map(hash).sum::<usize>() as i64
}

pub fn part2(input: &str) -> i64 {
    let binding = input.replace("\n", "");
    let operations = binding
        .split(',')
        .filter_map(|x|{
            if x.contains("=") {
                if let Some((label, val)) = x.split_once("="){
                    return Some((label, val.parse::<u64>().ok()));
                }
            }
            if x.contains("-") {
                if let Some((label, _)) = x.split_once("-"){
                    return Some((label, None));
                }
            }
            None
        })
        .collect_vec();
    let mut boxes: Vec<Vec<(String, u64)>> = (0..256).map(|_| vec![]).collect();
    for operation in operations {
        match operation {
            (label, Some(focal_length)) => { // (=) operation
                if let Some((pos, _)) = boxes[hash(label)].iter().find_position(|(l, _)| l == label) {
                    boxes[hash(label)][pos] = (label.into(), focal_length);
                } else {
                    boxes[hash(label)].push((label.into(), focal_length));
                }
            },
            (label, None) => { // (-) operation
                if let Some((pos, _)) = boxes[hash(label)].iter().find_position(|(l, _)| l == label) {
                    boxes[hash(label)].remove(pos);
                }
            },
        }
    }
    boxes.iter().enumerate().flat_map(|(box_nr, bx)|
        bx.into_iter().enumerate().map(|(slot_nr, (_, fcl_l))| {
                (box_nr + 1) * (slot_nr + 1) * fcl_l.clone() as usize
        }).collect_vec()
    ).sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day15;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1() { assert_eq!(day15::part1(INPUT), 1320); }

    #[test]
    fn part2() { assert_eq!(day15::part2(INPUT), 145); }
}