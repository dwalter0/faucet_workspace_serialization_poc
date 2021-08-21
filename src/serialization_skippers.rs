use std::{collections::HashMap};
use num_traits::identities::Zero;

pub fn skip_serializing_if_empty_str(input: &str) -> bool{
    input == ""
}

pub fn skip_serializing_if_empty_vec<T>(input: &Vec<T>) -> bool{
    input.is_empty()
}

pub fn skip_serializing_if_empty_hash<T,U>(input: &HashMap<T,U>) -> bool{
    input.is_empty()
}

pub fn skip_serializing_if_empty_int<T: Zero>(input: &T) -> bool{
    input.is_zero()
}