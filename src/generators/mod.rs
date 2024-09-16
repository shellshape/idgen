pub mod password;
pub mod rand;
pub mod snowflake;
pub mod uuid;
pub mod xid;

use anyhow::Result;

pub trait Generator {
    type Params;

    fn generate(&self, global_params: Self::Params) -> Result<String>;
}

#[macro_export]
macro_rules! simple_generator {
    ($name:ident, $gen:expr) => {
        #[derive(clap::Args, Clone)]
        pub(crate) struct $name {}

        impl $crate::generators::Generator for $name {
            type Params = $crate::Cli;

            fn generate(&self, _: Self::Params) -> anyhow::Result<String> {
                $gen
            }
        }
    };
}
