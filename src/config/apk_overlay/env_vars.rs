use std::collections::HashMap;

pub trait EnvVars {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String>;
}
