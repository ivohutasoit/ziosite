use std::{collections::HashMap, error::Error, fmt::Debug};

pub trait Plugin: Debug {
    fn execute(&self, params: &mut HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>>;
}
