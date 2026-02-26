use anyhow::{anyhow, Result};
use serenity::all::{ResolvedOption, ResolvedValue};

pub fn r_option_to_i32(option: &ResolvedOption) -> Result<i32> {
    match &option.value {
        ResolvedValue::Integer(id) => Ok(*id as i32),
        _ => Err(anyhow!("Option is not an integer")),
    }
}
