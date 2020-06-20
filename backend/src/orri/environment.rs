use std::env;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;


pub type Environment = HashMap<String, String>;


pub fn get_environment() -> Environment {
    env::vars().collect()
}

pub fn lookup<T>(environment: &Environment, key: &'static str) -> Result<T, Error>
    where T: FromStr,
          T::Err: Display {

    environment.get(key)
        .ok_or(Error::KeyNotFound(key))
        .and_then(|string_value| {
            string_value
                .parse::<T>()
                .map_err(|err| Error::Parse{
                    key,
                    details: err.to_string(),
                })
        })
}


#[derive(Debug)]
pub enum Error {
    KeyNotFound(&'static str),
    Parse {
        key: &'static str,
        details: String,
    },
}
