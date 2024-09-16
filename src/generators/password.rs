use std::ops::RangeInclusive;

use super::Generator;
use crate::{util, Cli};
use clap::Args;

#[derive(Args, Clone)]
#[command(alias = "pw")]
pub(crate) struct Password {
    /// The length of the generated password
    #[arg(long, short = 'n', default_value_t = 16)]
    len: usize,

    /// Include lower case characters
    #[arg(long, short = 'L')]
    lower: bool,

    /// Include upper case characters
    #[arg(long, short = 'U')]
    upper: bool,

    /// Include digit characters
    #[arg(long, short = 'D', alias = "numbers", short_alias = 'N')]
    digits: bool,

    /// Include special characters
    #[arg(long, short = 'S')]
    special: bool,
}

impl Generator for Password {
    type Params = Cli;

    fn generate(&self, _: Self::Params) -> anyhow::Result<String> {
        let mut chars = vec![];

        if self.lower {
            append_lowercase(&mut chars);
        }

        if self.upper {
            append_uppsrcase(&mut chars);
        }

        if self.digits {
            append_digits(&mut chars);
        }

        if self.special {
            append_special(&mut chars);
        }

        if chars.is_empty() {
            append_lowercase(&mut chars);
            append_uppsrcase(&mut chars);
            append_digits(&mut chars);
        }

        Ok(util::random_string(self.len, &chars))
    }
}

fn append_range(v: &mut Vec<char>, r: RangeInclusive<char>) {
    let (n, _) = r.size_hint();
    v.reserve(n);
    r.into_iter().for_each(|c| v.push(c));
}

fn append_lowercase(chars: &mut Vec<char>) {
    append_range(chars, 'a'..='z');
}

fn append_uppsrcase(chars: &mut Vec<char>) {
    append_range(chars, 'A'..='Z');
}

fn append_digits(chars: &mut Vec<char>) {
    append_range(chars, '0'..='9');
}

fn append_special(chars: &mut Vec<char>) {
    append_range(chars, '!'..='/');
    append_range(chars, ':'..='@');
    append_range(chars, '^'..='_');
    append_range(chars, '{'..='~');
}
