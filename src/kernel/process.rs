use alloc::{collections::BTreeMap, string::String};

#[derive(Debug, Clone)]
pub struct Process {
    env: BTreeMap<String, String>,
}
