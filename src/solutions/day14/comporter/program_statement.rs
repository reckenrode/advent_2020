use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, space0},
    combinator::{eof, map, map_res},
    sequence::{delimited, pair, preceded, terminated},
    Finish,
};

pub enum ProgramStatement<'a> {
    Instruction(usize, u64),
    Mask(&'a str),
}

impl<'a> ProgramStatement<'a> {
    pub fn parse(line: &'a str) -> Result<ProgramStatement> {
        fn mask_statement<'a>() -> impl FnMut(&'a str) -> nom::IResult<&'a str, &'a str> {
            let mask_def = tag("mask");
            let mask_content = take_while1(|s| s == 'X' || s == '1' || s == '0');
            terminated(
                preceded(
                    mask_def,
                    preceded(delimited(space0, char('='), space0), mask_content),
                ),
                eof,
            )
        }
        fn mem_statement<'a>() -> impl FnMut(&'a str) -> nom::IResult<&'a str, (usize, u64)> {
            let mem_ref = delimited(
                char('['),
                map_res(digit1, |s: &str| s.parse::<usize>()),
                char(']'),
            );
            let mem_contents = map_res(digit1, |s: &str| s.parse::<u64>());
            terminated(
                pair(
                    preceded(tag("mem"), mem_ref),
                    preceded(delimited(space0, char('='), space0), mem_contents),
                ),
                eof,
            )
        }
        let mut parser = alt((
            map(mem_statement(), |(addr, value)| {
                ProgramStatement::Instruction(addr, value)
            }),
            map(mask_statement(), ProgramStatement::Mask),
        ));
        let (_, result) = parser(line).finish().map_err(|e| anyhow!("{}", e))?;
        Ok(result)
    }
}
