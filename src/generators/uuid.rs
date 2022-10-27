use crate::simple_generator;

simple_generator!(Uuid, Ok(uuid::Uuid::new_v4().to_string()));
