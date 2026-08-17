/*
*   This code is just a placeholder api. The plan is to place everything
*   related to constants in this file eventualy. Originaly I placed it in
*   the UI code. Since I don't want to redo it now I'll do it later.
*
*   For now this is just API for new code to call so I at least don't 
*   need to rewrite that in the future.
*/

use std::sync::{LazyLock, Mutex};

use indexmap::map::IndexMap;

pub trait ConstantsMap {
    fn get_value(&self, latex: &str) -> Option<f64>;
}

impl ConstantsMap for IndexMap<String, (String, String, f64)> {
    fn get_value(&self, key: &str) -> Option<f64> {
        if !self.contains_key(key) { 
            return None; 
        }

        let (_, _, value) = self.get(key).expect(&format!("self should contain {}", key));
        return Some(*value);
    }
}

static DEFAULT_CONSTANTS: LazyLock<Mutex<IndexMap<String, (String, String, f64)>>> = LazyLock::new(|| {Mutex::new(IndexMap::new())}); //<latex, (name, display, value)>
static CUSTOM_CONSTANTS: LazyLock<Mutex<IndexMap<String, (String, String, f64)>>> = LazyLock::new(|| {Mutex::new(IndexMap::new())}); //<latex, (name, display, value)>

pub fn get_value(latex: &str) -> Option<f64> {
    let default_constants = DEFAULT_CONSTANTS.lock().unwrap();
    if default_constants.contains_key(latex) {
        return default_constants.get_value(latex);
    }

    return CUSTOM_CONSTANTS.lock().unwrap().get_value(latex);
}

#[cfg(test)]
mod tests {
    use super::*;

    // ConstantsMap

    #[test]
    fn test_constant_map_get_value() {
        let mut map: IndexMap<String, (String, String, f64)> = IndexMap::new();
        map.insert("\\test".to_string(), ("test".to_string(), "t".to_string(), 9.8));


        assert_eq!(map.get_value("\\test"), Some(9.8));
        assert_eq!(map.get_value("\\none"), None);
    }

    // static functions

    #[test]
    fn test_get_value() {
        DEFAULT_CONSTANTS.lock().unwrap().insert("\\value".to_string(), ("value".to_string(), "v".to_string(), 11.09));
        CUSTOM_CONSTANTS.lock().unwrap().insert("\\custom".to_string(), ("custom".to_string(), "c".to_string(), -7.897));

        assert_eq!(get_value("\\value"), Some(11.09));
        assert_eq!(get_value("\\custom"), Some(-7.897));
        assert_eq!(get_value("\\test"), None);
    }
}
