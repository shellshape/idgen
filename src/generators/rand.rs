use super::Generator;
use crate::{util, Cli};
use anyhow::Result;
use clap::Args;
use rand::{rngs::OsRng, Rng};
use std::ops::RangeInclusive;

#[derive(Args, Clone)]
pub(crate) struct Rand {
    /// The length of the generated string
    #[arg(long, short = 'n', default_value_t = 16)]
    len: usize,

    /// A list of characters to choose from
    #[arg(long, short, conflicts_with = "ranges")]
    list: Option<String>,

    /// Ranges of characters to choose from
    #[arg(long, short, conflicts_with = "list", value_parser = value_parser_ranges)]
    ranges: Option<Vec<RangeInclusive<char>>>,
}

fn value_parser_ranges(value: &str) -> Result<RangeInclusive<char>, String> {
    let (start, end) = value
        .split_once('-')
        .ok_or("range does not contain a delimiter")?;

    let start: char = start
        .trim()
        .parse()
        .map_err(|e| format!("failed parsing range start: {e}"))?;
    let end: char = end
        .trim()
        .parse()
        .map_err(|e| format!("failed parsing range end: {e}"))?;

    Ok(start..=end)
}

impl Generator for Rand {
    type Params = Cli;

    fn generate(&self, _: Self::Params) -> Result<String> {
        let chars_list = self.list.as_ref().map(|s| s.chars().collect());
        let chars_ranges = self.ranges.as_ref().map(|r| chars_from_ranges(r));

        let chars = chars_list
            .or(chars_ranges)
            .unwrap_or_else(|| chars_from_ranges(&['!'..='_', 'a'..='~']));

        Ok(util::random_string(self.len, &chars))
    }
}

fn chars_from_ranges(s: &[RangeInclusive<char>]) -> Vec<char> {
    s.iter().flat_map(|r| r.clone()).collect()
}
