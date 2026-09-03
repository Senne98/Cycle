use std::sync::{LazyLock, Mutex};
use std::fs;
use std::fs::File;
use std::io::Write;

use indexmap::map::IndexMap;

pub trait ConstantsMap {
    fn get_value(&self, key: &str) -> Option<String>;
    fn get_value_as_f64(&self, key: &str) -> Option<f64>;
}

impl ConstantsMap for IndexMap<String, (String, String, String)> {
    fn get_value(&self, key: &str) -> Option<String> {
        if !self.contains_key(key) { 
            return None; 
        }

        let (_, _, value) = self.get(key).expect(&format!("self should contain {}", key));
        return Some(value.clone());
    }

    fn get_value_as_f64(&self, key: &str) -> Option<f64> {
        if !self.contains_key(key) { 
            return None; 
        }

        let (_, _, value) = self.get(key).expect(&format!("self should contain {}", key));
        return Some(value.parse::<f64>().expect("value should be a f64"));
    }
}

static DEFAULT_CONSTANTS: LazyLock<Mutex<IndexMap<String, (String, String, String)>>> = LazyLock::new(|| {Mutex::new(IndexMap::new())}); //<latex, (name, display, value)>
static CUSTOM_CONSTANTS: LazyLock<Mutex<IndexMap<String, (String, String, String)>>> = LazyLock::new(|| {Mutex::new(IndexMap::new())}); //<latex, (name, display, value)>

pub fn get_value(latex: &str) -> Option<String> {
    let default_constants = DEFAULT_CONSTANTS.lock().unwrap();
    if default_constants.contains_key(latex) {
        return default_constants.get_value(latex);
    }

    return CUSTOM_CONSTANTS.lock().unwrap().get_value(latex);
}

pub fn get_value_as_f64(latex: &str) -> Option<f64> {
    let default_constants = DEFAULT_CONSTANTS.lock().unwrap();
    if default_constants.contains_key(latex) {
        return default_constants.get_value_as_f64(latex);
    }

    return CUSTOM_CONSTANTS.lock().unwrap().get_value_as_f64(latex);
}

pub fn is_constant(latex: &str) -> bool {
    if DEFAULT_CONSTANTS.lock().unwrap().contains_key(latex) {
        return true;
    }
    return CUSTOM_CONSTANTS.lock().unwrap().contains_key(latex);
}

pub fn get_custom_constants() -> Vec<(String, String, String, String)> {
    CUSTOM_CONSTANTS.lock().unwrap().iter()
            .map(|(latex, (name, display, value))| (latex.clone(), name.clone(), display.clone(), value.clone()))
            .collect()
}

pub fn get_default_constants() -> Vec<(String, String, String, String)> {
    DEFAULT_CONSTANTS.lock().unwrap().iter()
            .map(|(latex, (name, display, value))| (latex.clone(), name.clone(), display.clone(), value.clone()))
            .collect()
}


pub fn remove_custom_constant(latex: String) {
    CUSTOM_CONSTANTS.lock().unwrap().shift_remove(&latex);
    save_custom_constants();
}

// Load constants from disk

const DEFAULT_CONSTANTS_FILE: &str = "rescources/default_constants.csv";
const CUSTOM_CONSTANTS_FILE: &str = "custom_constants.csv";

pub fn load_constants() {
    load_default_constants();
    load_custom_constants();
}

pub fn add_custom_constant(latex: &str, name: &str, display: &str, value: &str) {
    // VALIDATE VALUES
    // ADD TESTCASE
    CUSTOM_CONSTANTS.lock().unwrap().insert(latex.to_string(), (name.to_string(), display.to_string(), value.to_string()));
    save_custom_constants();
}

fn load_default_constants() {
    let cst_file = fs::read_to_string(DEFAULT_CONSTANTS_FILE).unwrap_or_else(|_| panic!("Can't read file {DEFAULT_CONSTANTS_FILE}"));
    let mut constants = cst_file.lines();

    let mut default_constants = DEFAULT_CONSTANTS.lock().unwrap();

    constants.next();

    for constant in constants {
        let mut parts = constant.split(",");
        let latex = parts.next().unwrap().to_string();
        let name = parts.next().unwrap().to_string();
        let display = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();

        default_constants.insert(latex.clone(), (name.clone(), display.clone(), value.clone()));
    }
}

fn load_custom_constants() {
    let cst_file = fs::read_to_string(CUSTOM_CONSTANTS_FILE).expect(&format!("Can't create file {CUSTOM_CONSTANTS_FILE}"));
    let constants = cst_file.lines();

    let mut custom_constants = CUSTOM_CONSTANTS.lock().unwrap();

    for constant in constants {
        let mut parts = constant.split(",");
        let latex = parts.next().unwrap().to_string();
        let name = parts.next().unwrap().to_string();
        let display = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();

        custom_constants.insert(latex.clone(), (name.clone(), display.clone(), value.clone()));
    }
}

fn save_custom_constants() {
    let mut file_content = "".to_string();
    let custom_constants = CUSTOM_CONSTANTS.lock().unwrap();
    let latex_symbols = custom_constants.keys();

    for latex in latex_symbols {
        let (name, display, value) = custom_constants.get(latex).unwrap();
        let latex = latex.clone();

        file_content.push_str(&format!("{},{},{},{}\n", latex, name, display, value));
    }

    let _ = file_content.trim_end_matches("\n");

    let mut file = File::create(CUSTOM_CONSTANTS_FILE).expect(&format!("Can't create file {CUSTOM_CONSTANTS_FILE}"));
    let _ = file.write_all(file_content.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    // ConstantsMap

    #[test]
    fn test_constant_map_get_value() {
        let mut map: IndexMap<String, (String, String, String)> = IndexMap::new();
        map.insert("\\test".to_string(), ("test".to_string(), "t".to_string(), "9.8".to_string()));


        assert_eq!(map.get_value("\\test"), Some("9.8".to_string()));
        assert_eq!(map.get_value("\\none"), None);
    }

    #[test]
    fn test_constant_map_get_as_f64_value() {
        let mut map: IndexMap<String, (String, String, String)> = IndexMap::new();
        map.insert("\\test".to_string(), ("test".to_string(), "t".to_string(), "9.8".to_string()));


        assert_eq!(map.get_value_as_f64("\\test"), Some(9.8));
        assert_eq!(map.get_value_as_f64("\\none"), None);
    }

    // static functions
    
    #[test]
    fn test_get_value() {
        DEFAULT_CONSTANTS.lock().unwrap().insert("\\value".to_string(), ("value".to_string(), "v".to_string(), "11.09".to_string()));
        CUSTOM_CONSTANTS.lock().unwrap().insert("\\custom".to_string(), ("custom".to_string(), "c".to_string(), "-7.897".to_string()));

        assert_eq!(get_value("\\value"), Some("11.09".to_string()));
        assert_eq!(get_value("\\custom"), Some("-7.897".to_string()));
        assert_eq!(get_value("\\test"), None);
    }

    #[test]
    fn test_get_value_as_f64() {
        DEFAULT_CONSTANTS.lock().unwrap().insert("\\value".to_string(), ("value".to_string(), "v".to_string(), "11.09".to_string()));
        CUSTOM_CONSTANTS.lock().unwrap().insert("\\custom".to_string(), ("custom".to_string(), "c".to_string(), "-7.897".to_string()));

        assert_eq!(get_value_as_f64("\\value"), Some(11.09));
        assert_eq!(get_value_as_f64("\\custom"), Some(-7.897));
        assert_eq!(get_value_as_f64("\\test"), None);
    }

    #[test]
    fn test_load_default_constants() {
        load_default_constants();

        assert_eq!(get_value_as_f64("e"), Some(2.718281828459045235360287471352));
        assert_eq!(get_value_as_f64("h"), Some(6.62607015e-34));
        assert_eq!(get_value_as_f64("\\hbar"), Some(1.054571817e-34));
    }
}
