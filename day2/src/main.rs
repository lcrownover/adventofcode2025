use anyhow::Result;
use std::fs;

#[allow(dead_code)]
fn pt1() -> Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let input = contents.split_whitespace().next().unwrap();

    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut invalids: Vec<u64> = vec![];
    for r in ranges {
        let start: u64 = r.split('-').nth(0).unwrap().parse()?;
        let end: u64 = r.split('-').nth(1).unwrap().parse()?;
        for i in start..=end {
            let istr: String = i.to_string();
            if istr.len() % 2 != 0 {
                continue;
            }
            let isplt = istr.split_at(istr.len() / 2);
            if isplt.0 == isplt.1 {
                invalids.push(i);
            }
        }
    }
    println!("{}", invalids.iter().sum::<u64>());
    Ok(())
}

/// Parses a given string 's' into a vec of strings where each element
/// is of length 'length'.
///
/// # Examples
///
/// ```
/// assert_eq!(parse_into_groups("111", 1), vec!["1", "1", "1"]);
/// assert_eq!(parse_into_groups("111", 2), vec!["11", "1"]);
/// assert_eq!(parse_into_groups("111", 3), vec!["111"]);
/// ```
///
fn parse_into_groups(s: &str, length: usize) -> Vec<String> {
    let mut groups: Vec<String> = vec![];
    let mut remain = s;
    loop {
        if length > remain.len() {
            if !remain.is_empty() {
                groups.push(remain.to_owned());
            }
            break;
        }
        let splt = remain.split_at(length);
        groups.push(splt.0.to_owned());
        remain = splt.1;
    }
    groups
}

fn pt2() -> Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let input = contents.split_whitespace().next().unwrap();

    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut invalids: Vec<u64> = vec![];
    for r in ranges {
        let start: u64 = r.split('-').nth(0).unwrap().parse()?;
        let end: u64 = r.split('-').nth(1).unwrap().parse()?;
        for i in start..=end {
            let istr: String = i.to_string();
            for div in 1..=istr.len() {
                let groups = parse_into_groups(&istr, div);

                if groups.is_empty() || groups.len() == 1 {
                    continue;
                }
                let first = groups[0].clone();
                if groups.iter().all(|item| *item == first) {
                    let num: u64 = groups.join("").parse()?;
                    if !invalids.contains(&num) {
                        invalids.push(num);
                    }
                };
            }
        }
    }
    println!("{}", invalids.iter().sum::<u64>());
    Ok(())
}

fn main() -> Result<()> {
    // pt1()?;
    pt2()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_into_groups() {
        assert_eq!(parse_into_groups("11", 1), vec!["1", "1"]);
        assert_eq!(parse_into_groups("11", 2), vec!["11"]);
        assert_eq!(parse_into_groups("111", 1), vec!["1", "1", "1"]);
        assert_eq!(parse_into_groups("111", 2), vec!["11", "1"]);
        assert_eq!(parse_into_groups("111", 3), vec!["111"]);
        assert_eq!(
            parse_into_groups("1188511885", 1),
            vec!["1", "1", "8", "8", "5", "1", "1", "8", "8", "5"]
        );
        assert_eq!(parse_into_groups("1188511885", 10), vec!["1188511885"]);
        assert_eq!(parse_into_groups("111", 2), vec!["11", "1"]);
        assert_eq!(parse_into_groups("111", 3), vec!["111"]);
    }
}
