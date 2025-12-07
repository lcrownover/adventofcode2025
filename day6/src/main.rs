use anyhow::{Result, bail};

#[allow(dead_code, unused_variables)]
fn pt1(contents: &str) -> Result<()> {
    let lines: Vec<String> = contents.lines().map(|l| l.to_owned()).collect();
    let matrix: Vec<Vec<String>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();
    let problem_count: usize = matrix.iter().nth(0).unwrap().len();
    let mut transposed: Vec<Vec<String>> = vec![];
    for idx in 0..problem_count {
        let mut tentry: Vec<String> = vec![];
        for entry in &matrix {
            tentry.push(entry[idx].clone());
        }
        transposed.push(tentry);
    }
    let problem_length = transposed.iter().nth(0).unwrap().len();

    let mut total = 0;
    for problem in &transposed {
        let op = problem[problem_length - 1].clone();
        let nums = &problem[..problem_length - 1];
        let val = match op.as_ref() {
            "*" => nums.iter().fold(1, |mut acc, n| {
                acc *= n.parse::<u64>().unwrap();
                acc
            }),
            "+" => nums.iter().fold(0, |mut acc, n| {
                acc += n.parse::<u64>().unwrap();
                acc
            }),
            _ => bail!("bad op"),
        };
        total += val;
    }
    println!("{}", total);
    Ok(())
}

#[derive(Debug, Clone)]
struct RawProblem {
    lines: Vec<String>,
    op: String,
}

#[allow(dead_code)]
impl Into<Problem> for RawProblem {
    /// Parses a RawProblem into a Problem
    ///
    /// Example:
    ///
    /// RawProblem { lines: ["123", " 45", "  6"], op: "*" }
    /// =>
    /// Problem { numbers: [356, 24, 1], op: "*" }
    ///
    fn into(self) -> Problem {
        let mut numbers: Vec<u64> = vec![];
        let digits = self.lines.iter().nth(0).unwrap().len();
        for i in 0..digits {
            let mut makervec: Vec<String> = vec![];
            for n in &self.lines {
                makervec.push(n.chars().nth(i).unwrap().to_string());
            }
            let num: u64 = makervec.join("").trim().parse().unwrap();
            numbers.push(num);
        }
        Problem {
            numbers: numbers,
            op: self.op,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    op: String,
}

impl Problem {
    fn solve(&self) -> u64 {
        let v: u64 = match self.op.as_ref() {
            "*" => self.numbers.iter().fold(1, |mut acc, n| {
                acc *= n;
                acc
            }),
            "+" => self.numbers.iter().fold(0, |mut acc, n| {
                acc += n;
                acc
            }),
            _ => panic!("bad op"),
        };
        v
    }
}

#[allow(dead_code, unused_variables)]
fn pt2(contents: &str) -> Result<()> {
    let lines: Vec<&str> = contents.lines().collect();
    let lines_chars: Vec<Vec<String>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect();
    // iterate through the first line and create a vec
    // of all the indexes that we'll split the contents on
    let line_length = lines[0].len();
    let num_lines = lines.len();
    let mut split_ons: Vec<usize> = vec![];
    for cursor in 0..line_length {
        let mut good = true;
        for l in &lines {
            let c = l.chars().nth(cursor).unwrap();
            if l.chars().nth(cursor).unwrap() != ' ' {
                good = false
            }
        }
        if good {
            split_ons.push(cursor)
        }
    }
    // get the last problem as well
    split_ons.push(line_length);

    // now that we know the columns that deliniate the problems,
    // we can get a start and end range based on those columns
    let mut problems: Vec<RawProblem> = vec![];

    let mut bottom_range = 0;
    for split_idx in split_ons {
        // this is because the first element needs to be
        // inclusive of 0, but all the rest of them
        // need to skip the character at the last index
        let top_range = split_idx;
        let mut problem_lines: Vec<String> = vec![];
        for line in &lines_chars {
            let line_section = match bottom_range {
                0 => line[bottom_range..top_range].to_vec().join(""),
                _ => line[bottom_range + 1..top_range].to_vec().join(""),
            };
            problem_lines.push(line_section);
        }
        bottom_range = split_idx;

        let op = problem_lines.last().unwrap().trim().to_string();
        problem_lines.remove(problem_lines.len() - 1);
        problems.push(RawProblem {
            lines: problem_lines,
            op: op,
        })
    }

    let mut total = 0;
    for rp in problems {
        let p: Problem = rp.into();
        let val = p.solve();
        total += val;
    }

    println!("{}", total);
    Ok(())
}

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("input.txt")?;
    // pt1(&contents)?;
    pt2(&contents)?;
    Ok(())
}
