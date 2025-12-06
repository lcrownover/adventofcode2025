use std::{fmt::Display, num::ParseIntError, str::FromStr};

type Ingredient = u64;

#[derive(Clone, Debug, Copy)]
struct Range {
    start: u64,
    end: u64,
}

struct RangeIter {
    current: u64,
    end: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl From<ParseIntError> for ParseRangeError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        let start = split.next().unwrap().parse::<u64>()?;
        let end = split.next().unwrap().parse::<u64>()?;
        Ok(Self { start, end })
    }
}

impl Range {
    #[allow(dead_code, unused_variables)]
    fn iter(&self) -> RangeIter {
        RangeIter {
            current: self.start,
            end: self.end,
        }
    }

    fn contains(&self, other: u64) -> bool {
        other >= self.start && other <= self.end
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        return self.start == other.start && self.end == other.end;
    }
}

impl Eq for Range {}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start == other.start {
            true => self.end.cmp(&other.end),
            false => self.start.cmp(&other.start),
        }
    }
}

impl Iterator for RangeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let v = self.current;
            self.current += 1;
            Some(v)
        }
    }
}

#[derive(Debug)]
struct Fridge {
    contents: Vec<Range>,
}

struct FridgeIter {
    contents: Vec<Range>,
    current_idx: usize,
}

impl Iterator for FridgeIter {
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.contents.len() {
            None
        } else {
            let v = self.contents[self.current_idx];
            self.current_idx += 1;
            Some(v)
        }
    }
}

impl Fridge {
    fn new(contents: &Vec<Range>) -> Self {
        Self {
            contents: contents.clone(),
        }
    }

    fn sort_contents(&self) -> Self {
        let mut sorted_contents = self.contents.clone();
        sorted_contents.sort();
        Fridge {
            contents: sorted_contents,
        }
    }

    fn pack_contents(&self) -> Self {
        let mut packed: Vec<Range> = vec![];
        for r in &self.contents {
            // just push the first element
            if packed.is_empty() {
                packed.push(*r);
                continue;
            }
            // dedupe
            if packed.contains(r) {
                continue;
            }
            // compare and possibly merge with last entry
            let last_idx = packed.len();
            let last = packed[last_idx - 1].clone();
            if last.contains(r.start) {
                // skip the range if a packed range already includes the
                // entire range
                if last.contains(r.end) {
                    continue;
                }
                // or extend the range
                let new_r = Range {
                    start: last.start,
                    end: r.end,
                };
                packed[last_idx - 1] = new_r;
                continue;
            }

            // otherwise append the new range
            packed.push(*r);
        }
        Fridge { contents: packed }
    }

    fn iter(&self) -> FridgeIter {
        FridgeIter {
            contents: self.contents.clone(),
            current_idx: 0,
        }
    }

    fn fresh_ingredients(&self) -> u64 {
        self.iter().fold(0, |mut acc, i| {
            acc += i.len();
            acc
        })
    }

    fn contains(&self, ingredient: Ingredient) -> bool {
        for r in &self.contents {
            if r.contains(ingredient) {
                return true;
            }
        }
        return false;
    }
}

#[allow(dead_code, unused_variables)]
fn pt1(ranges: &Vec<Range>, ingredients: &Vec<u64>) -> anyhow::Result<()> {
    let fridge = Fridge::new(ranges);
    let mut freshies = 0;
    for i in ingredients {
        if fridge.contains(*i) {
            freshies += 1;
            continue;
        }
    }
    println!("{freshies}");
    Ok(())
}

#[allow(dead_code, unused_variables)]
fn pt2(ranges: &Vec<Range>) -> anyhow::Result<()> {
    let fridge = Fridge::new(ranges).sort_contents().pack_contents();
    let freshies = fridge.fresh_ingredients();
    println!("{}", freshies);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("input.txt")?;

    let ranges = contents
        .lines()
        .filter(|l| l.contains("-"))
        .map(|r| r.parse::<Range>().unwrap())
        .collect::<Vec<Range>>();

    let ingredient_nums = contents
        .lines()
        .filter(|l| !l.contains("-") && !l.trim().is_empty())
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    pt1(&ranges, &ingredient_nums)?;
    pt2(&ranges)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_length() {
        let r1 = Range { start: 3, end: 5 };
        let r2 = Range {
            start: 20,
            end: 100,
        };
        let r3 = Range { start: 5, end: 5 };
        assert_eq!(r1.len(), 3);
        assert_eq!(r2.len(), 81);
        assert_eq!(r3.len(), 1);
    }

    #[test]
    fn test_range_iter() {
        let v: u64 = Range { start: 3, end: 5 }.iter().sum();
        assert_eq!(v, 12);
    }

    #[test]
    fn test_new_fridge() {
        let rs = vec![Range { start: 3, end: 5 }, Range { start: 10, end: 12 }];
        let f = Fridge::new(&rs);
        assert_eq!(f.contents[0], Range { start: 3, end: 5 });
        assert_eq!(f.contents[1], Range { start: 10, end: 12 });
    }

    #[test]
    fn test_sorted_fridge() {
        let rs = vec![Range { start: 10, end: 12 }, Range { start: 3, end: 5 }];
        let f = Fridge::new(&rs).sort_contents();
        assert_eq!(f.contents[0], Range { start: 3, end: 5 });
        assert_eq!(f.contents[1], Range { start: 10, end: 12 });
    }

    #[test]
    fn test_packed_fridge_one_big() {
        let rs = vec![
            Range { start: 10, end: 12 },
            Range { start: 3, end: 5 },
            Range { start: 12, end: 12 },
            Range { start: 3, end: 6 },
            Range { start: 1, end: 15 },
        ];
        let f = Fridge::new(&rs).sort_contents().pack_contents();
        assert_eq!(f.contents[0], Range { start: 1, end: 15 });
    }

    #[test]
    fn test_packed_fridge_smaller_multiple() {
        let rs = vec![
            Range { start: 10, end: 12 },
            Range { start: 3, end: 5 },
            Range { start: 12, end: 12 },
            Range { start: 3, end: 6 },
        ];
        let f = Fridge::new(&rs).sort_contents().pack_contents();
        assert_eq!(f.contents[0], Range { start: 3, end: 6 });
        assert_eq!(f.contents[1], Range { start: 10, end: 12 });
    }

    #[test]
    fn test_fridge_fresh_ingredients() {
        let rs = vec![
            Range { start: 10, end: 12 },
            Range { start: 3, end: 5 },
            Range { start: 12, end: 12 },
            Range { start: 3, end: 6 },
        ];
        let f = Fridge::new(&rs).sort_contents().pack_contents();
        assert_eq!(f.fresh_ingredients(), 7);
    }
}
