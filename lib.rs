use std::collections::HashMap;

//create a structure for a dynamically typed dictionary
#[derive(Debug, Clone)]
pub struct Dictionary {
    pub data: HashMap<String, Value>,
}
//create a structure for a dynamically typed value
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Dict(Dictionary),
}
//implement the dictionary structure
impl Dictionary {
    //create a new dictionary
    pub fn new() -> Dictionary {
        Dictionary {
            data: HashMap::new(),
        }
    }
    //add a new value to the dictionary
    pub fn add(&mut self, key: &str, value: Value) {
        self.data.insert(key.to_string(), value);
    }
    //get a value from the dictionary
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
    //get a mutable value from the dictionary
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.data.get_mut(key)
    }
    //remove a value from the dictionary
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
    //check if the dictionary contains a value
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    //get the length of the dictionary
    pub fn len(&self) -> usize {
        self.data.len()
    }
    //check if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    //clear the dictionary
    pub fn clear(&mut self) {
        self.data.clear();
    }
    //get the keys of the dictionary
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().map(|x| x.to_string()).collect()
    }
    //get the values of the dictionary
    pub fn values(&self) -> Vec<Value> {
        self.data.values().map(|x| x.clone()).collect()
    }
}
//implement the value structure
impl Value {
    //get the type of the value
    pub fn get_type(&self) -> String {
        match self {
            Value::Int(_) => "int".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Dict(_) => "dict".to_string(),
        }
    }
    //get the value as an integer
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(x) => Some(*x),
            _ => None,
        }
    }
    //get the value as a float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(x) => Some(*x),
            _ => None,
        }
    }
    //get the value as a string
    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(x) => Some(x.to_string()),
            _ => None,
        }
    }
    //get the value as a boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(x) => Some(*x),
            _ => None,
        }
    }
    //get the value as a list
    pub fn as_list(&self) -> Option<Vec<Value>> {
        match self {
            Value::List(x) => Some(x.to_vec()),
            _ => None,
        }
    }
    //get the value as a dictionary
    pub fn as_dict(&self) -> Option<Dictionary> {
        match self {
            Value::Dict(x) => Some(x.clone()),
            _ => None,
        }
    }
    //create a new integer value
    pub fn new_int(x: i32) -> Value {
        Value::Int(x)
    }
    //create a new float value
    pub fn new_float(x: f64) -> Value {
        Value::Float(x)
    }
    //create a new string value
    pub fn new_string(x: String) -> Value {
        Value::String(x)
    }
    //create a new boolean value
    pub fn new_bool(x: bool) -> Value {
        Value::Bool(x)
    }
    //create a new list value
    pub fn new_list(x: Vec<Value>) -> Value {
        Value::List(x)
    }
    //create a new dictionary value
    pub fn new_dict(x: Dictionary) -> Value {
        Value::Dict(x)
    }
    //get the length of the value
    pub fn len(&self) -> Option<usize> {
        match self {
            Value::List(x) => Some(x.len()),
            Value::Dict(x) => Some(x.len()),
            _ => None,
        }
    }
    //check if the value is empty
    pub fn is_empty(&self) -> Option<bool> {
        match self {
            Value::List(x) => Some(x.is_empty()),
            Value::Dict(x) => Some(x.is_empty()),
            _ => None,
        }
    }
    //clear the value
    pub fn clear(&mut self) {
        match self {
            Value::List(x) => x.clear(),
            Value::Dict(x) => x.clear(),
            _ => (),
        }
    }
    //get the keys of the value
    pub fn keys(&self) -> Option<Vec<String>> {
        match self {
            Value::Dict(x) => Some(x.keys()),
            _ => None,
        }
    }
    //get the values of the value
    pub fn values(&self) -> Option<Vec<Value>> {
        match self {
            Value::Dict(x) => Some(x.values()),
            _ => None,
        }
    }
    //get a value from the value
    pub fn get(&self, key: &str) -> Option<Value> {
        match self {
            Value::Dict(x) => x.get(key).map(|x| x.clone()),
            _ => None,
        }
    }
    //get a mutable value from the value
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        match self {
            Value::Dict(x) => x.get_mut(key),
            _ => None,
        }
    }
    //check if the value contains a value
    pub fn contains(&self, key: &str) -> Option<bool> {
        match self {
            Value::Dict(x) => Some(x.contains(key)),
            _ => None,
        }
    }
    //remove a value from the value
    pub fn remove(&mut self, key: &str) {
        match self {
            Value::Dict(x) => x.remove(key),
            _ => (),
        }
    }
}

macro_rules! dict {
    ($($key:expr => $value:expr),*) => {{
        let mut dict = Dictionary::new();
        $(
            dict.add($key, $value);
        )*
        dict
    }};
}
