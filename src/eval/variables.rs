use std::collections::HashMap;

use super::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Variables(pub HashMap<String, Value>);
