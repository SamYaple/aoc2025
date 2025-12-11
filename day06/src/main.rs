use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::terminated;

enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        terminated(
            alt((
                map(preceded(space0, tag("+")), |_| Self::Addition),
                map(preceded(space0, tag("*")), |_| Self::Multiplication),
            )),
            multispace0,
        )
        .parse(input)
    }
}


struct MathHomework {
    inputs: Vec<Vec<u64>>,
    operators: Vec<Operation>,
    answers: Vec<u64>,
}

impl MathHomework {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, inputs) = many1(terminated(
            many1(preceded(space0, map_res(digit1, str::parse))),
            multispace0,
        ))
        .parse(input)?;

        let (input, operators) = many1(Operation::parse).parse(input)?;
        Ok((
            input,
            Self {
                inputs,
                operators,
                answers: Vec::new(),
            },
        ))
    }

    fn validate(&self) -> anyhow::Result<()> {
        for input in &self.inputs {
            if self.operators.len() != input.len() {
                anyhow::bail! {"input lines were of different lengths, bad input."}
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> anyhow::Result<()> {
        for pos in 0..self.operators.len() {
            let mut tmp = match self.operators[pos] {
                Operation::Addition => 0,
                Operation::Multiplication => 1,
            };

            for input in &self.inputs {
                match self.operators[pos] {
                    Operation::Addition => tmp += input[pos],
                    Operation::Multiplication => tmp *= input[pos],
                }
            }
            self.answers.push(tmp);
        }
        Ok(())
    }
}

// I refuse to update my part1 answer! Ill just rewrite the input...
fn translate_input_for_part2(input: &str) -> anyhow::Result<String> {
    let mut lines: Vec<&str> = input.lines().collect();
    let Some(operators) = lines.pop() else {
        anyhow::bail! {"Our input was empty"};
    };
    let (_, parsed_operators) = many1(Operation::parse).parse(operators).unwrap();

    // create character array of all lines of input except the operators
    let input_buffer: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let num_of_rows = input_buffer.len();
    let num_of_cols = parsed_operators.len();
    let length_of_chars = operators.chars().count();

    // create output buffer for reassembled numbers
    let mut output_buffer: Vec<Vec<String>> = Vec::new();
    for _ in 0..num_of_rows {
        output_buffer.push(Vec::with_capacity(num_of_cols));
    }

    let mut col = 0;
    let mut row = 0;
    for pos in 0..length_of_chars {
        let mut value = String::new();
        for chars in &input_buffer {
            value.push(chars[pos]);
        }
        if value.trim().is_empty() {
            while row < num_of_rows {
                match parsed_operators[col] {
                    Operation::Addition => output_buffer[row].push(" 0 ".to_string()),
                    Operation::Multiplication => output_buffer[row].push(" 1 ".to_string()),
                }
                row += 1;
            }
            col += 1;
            row = 0;
            continue;
        }
        output_buffer[row].push(value);
        row += 1;
    }
    while row < num_of_rows {
        match parsed_operators[col] {
            Operation::Addition => output_buffer[row].push("0".to_string()),
            Operation::Multiplication => output_buffer[row].push("1".to_string()),
        }
        row += 1;
    }
    let columns = output_buffer
        .iter()
        .map(|inner| inner.join(" "))
        .collect::<Vec<String>>()
        .join("\n");
    let translated = format! {"{columns}\n{operators}"};

    Ok(translated)
}

// parse input, return summed answer
fn do_homework(input: &str) -> anyhow::Result<u64> {
    // fully parse input, fail if any chars remain unparsed
    let (_, mut mh) = all_consuming(MathHomework::parse).parse(input).unwrap();

    // perform extra validation on the struct after successful parsing
    mh.validate()?;

    // populate the `answers` field of the struct with the math'd answer
    mh.solve()?;

    // pretty print all the math problems
    println!["{}", &mh];

    // Return the sum of all the answers
    Ok(mh.answers.into_iter().sum())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    let translated_input = &translate_input_for_part2(input)?;

    let part1_ans = do_homework(input)?;
    let part2_ans = do_homework(translated_input)?;

    println!["ans part1: {}", part1_ans];
    println!["ans part2: {}", part2_ans];
    Ok(())
}

// Pretty print the `MathHomework` struct.
// Prints out the math problem and answer:
//
//   day1:
//     123 * 45 * 6 = 33210
//     328 + 64 + 98 = 490
//     51 * 387 * 215 = 4243455
//     64 + 23 + 314 = 401
//
//   day2:
//     1 * 24 * 356 = 8544
//     369 + 248 + 8 = 625
//     32 * 581 * 175 = 3253600
//     623 + 431 + 4 = 1058
//
impl std::fmt::Display for MathHomework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in 0..self.operators.len() {
            // use a peekable iterator to check if we are in the last element for formatting
            let mut it = self.inputs.iter().peekable();
            while let Some(input) = it.next() {
                if it.peek().is_none() {
                    write!(f, "{} = ", input[pos])?;
                } else {
                    write!(f, "{} {} ", input[pos], self.operators[pos])?;
                }
            }
            write!(f, "{}\n", self.answers[pos])?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "+"),
            Self::Multiplication => write!(f, "*"),
        }
    }
}
