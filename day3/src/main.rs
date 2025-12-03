use std::cmp::Ordering;
use std::fs;

/// Process the bank unto a new vec with length 12, where
/// the resulting vec is the greatest number that can be made from the elements
/// from the bank by "smooshing" them together.
///
/// Example:
///
/// ```
/// assert_eq!(reduce_bank(vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]), &[9,8,7,6,5,4,3,2,1,1,1,1])
/// assert_eq!(reduce_bank(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]), &[8,8,8,9,1,1,1,1,2,1,1,1])
/// ```
fn reduce_bank(bank: Vec<u64>) -> Vec<u64> {
    let mut v = bank.clone();
    let mut cur = 0;
    while v.len() > 12 {
        if cur == v.len() - 1 {
            v.remove(cur);
            cur = 0;
            continue;
        }
        let (i, j) = (v[cur], v[cur + 1]);
        match i.cmp(&j) {
            Ordering::Less => {
                v.remove(cur);
                cur = 0;
            }
            Ordering::Greater | Ordering::Equal => {
                cur += 1;
            }
        }
    }
    v
}

#[allow(dead_code)]
fn pt1() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    println!("{:?}", lines);

    let mut batts: Vec<u32> = vec![];

    for line in lines {
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        'top: for top in (0..=9).rev() {
            'bank: for (idx, n) in bank.iter().enumerate() {
                if *n == top {
                    if idx + 1 == bank.len() {
                        break 'bank;
                    }
                    let remainder = &bank[idx + 1..];
                    let highest_remainder = remainder.iter().max().unwrap();
                    let batt = format!("{top}{highest_remainder}").parse::<u32>().unwrap();
                    batts.push(batt);
                    break 'top;
                }
            }
        }
    }

    println!("{}", batts.iter().sum::<u32>())
}

fn pt2() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut batts: Vec<u64> = vec![];

    for line in lines {
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).unwrap().into())
            .collect::<Vec<u64>>();

        let batt = reduce_bank(bank)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        batts.push(batt);
    }
    println!("{}", batts.iter().sum::<u64>())
}

fn main() {
    // pt1();
    pt2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_bank() {
        assert_eq!(
            reduce_bank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            &[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]
        );
        assert_eq!(
            reduce_bank(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            &[8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
        assert_eq!(
            reduce_bank(vec![
                9, 8, 6, 6, 9, 6, 6, 8, 6, 9, 3, 8, 9, 6, 7, 5, 6, 7, 5, 5, 9, 6, 5, 9, 9, 9, 7, 9,
                3, 5, 6, 9, 7, 9, 5, 8, 6, 5, 8, 9, 9, 8, 6, 9, 6, 9, 8, 5, 5, 8, 5, 9, 8, 9, 9, 7,
                7, 5, 9, 8, 8, 6, 8, 5, 9, 9, 2, 8, 5, 8, 9, 5, 7, 9, 9, 6, 7, 5, 8, 4, 9, 9, 6, 6,
                8, 9, 6, 9, 5, 4, 7, 8, 7, 8, 9
            ]),
            &[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            reduce_bank(vec![1, 9, 2, 3, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
            &[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        )
    }
}
