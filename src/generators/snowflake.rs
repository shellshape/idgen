use super::Generator;
use crate::Cli;
use anyhow::Result;
use clap::Args;

#[derive(Args, Clone)]
pub(crate) struct Snowflake {
    /// The Machine ID
    #[arg(long, short, default_value_t = 0)]
    machine: i32,
    /// The Node ID
    #[arg(long, short, default_value_t = 0)]
    node: i32,
}

impl Generator for Snowflake {
    type Params = Cli;

    fn generate(&self, _: Self::Params) -> Result<String> {
        Ok(
            snowflake::SnowflakeIdGenerator::new(self.machine, self.node)
                .generate()
                .to_string(),
        )
    }
}
