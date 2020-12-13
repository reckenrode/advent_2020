mod multiple_finder;

use anyhow::{
    anyhow,
    Result,
};
use clap::Clap;
use std::{
    error::Error,
    fs::File,
    io::{
        BufRead,
        BufReader,
    }
};

use multiple_finder::NextMultiple;

#[derive(Clap)]
pub struct Solution {
    input: std::path::PathBuf,
}

impl Solution {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open(&self.input)?);
        let (earliest_depature, busses) = Solution::parse(reader)?;
        let (next_bus, departure_time) = busses.into_iter()
            .map(|x| (x, earliest_depature.next_multiple(x)))
            .min_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
            .ok_or(anyhow!("no minimum number was found, which is unexpected"))?;
        let waiting_time = departure_time - earliest_depature;
        println!(
            "You have to wait {} minutes for bus {} to arrive. The multiplier is {}.",
            waiting_time,
            next_bus,
            waiting_time * next_bus
        );
        Ok(())
    }

    fn parse(file: impl BufRead) -> Result<(u32, Vec<u32>)> {
        let mut lines = file.lines();
        let my_time = lines.next()
            .ok_or(anyhow!("input missing first line"))??
            .parse()?;
        let times: Result<Vec<_>, _> = lines.next()
            .ok_or(anyhow!("input missing schedule of times"))??
            .split(",")
            .filter(|x| *x != "x")
            .map(|x| x.parse())
            .collect();
        Ok((my_time, times?))
    }
}
