use crate::simple_generator;

simple_generator!(Xid, Ok(xid::new().to_string()));
