//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

use crate::tools::std_ext::CharIsLetter;

pub trait ValidateConstants {
    fn is_valid_constant(&self) -> bool;

    fn get_valid_constant_left(&self) -> Option<String>;
}

impl ValidateConstants for String {
    fn is_valid_constant(&self) -> bool {
        let Some(result) = self.get_valid_constant_left() else { return false; };
        return result == self.to_string();
    }

    fn get_valid_constant_left(&self) -> Option<String> {
        let constant = self.trim().to_string();
        let Some(name) = get_valid_name(constant.clone()) else { return None; };
        if name == constant { return Some(name); }

        let mut result = name.clone();
        let mut constant = self.trim().strip_prefix(&name).unwrap();

        let next_char = constant.chars().next().unwrap();
        let mut checked_subscript = false;

        if next_char == '_' {
            checked_subscript = true;
        } else if next_char !='^' {
            return Some(result);
        }

        constant = constant.strip_prefix(next_char).unwrap();

        let Some(script) = get_valid_script(constant.to_string()) else {
            return Some(result);
        };

        result.push(next_char);
        result.push_str(&script);

        constant = constant.strip_prefix(&script).unwrap();
        let Some(next_char) = constant.chars().next() else { return Some(result); };

        if !((next_char == '_' && !checked_subscript) || (next_char =='^' && checked_subscript)) {
            return Some(result);
        }

        constant = constant.strip_prefix(next_char).unwrap();

        let Some(script) = get_valid_script(constant.to_string()) else {
            return Some(result);
        };

        result.push(next_char);
        result.push_str(&script);
        return Some(result);
    }
}

fn get_valid_name(constant: String) -> Option<String> {
    let mut constant_chars = constant.chars();
    let Some(first_char) = constant_chars.next() else { return None; };

    if first_char != '\\' {
        if !first_char.is_letter() { return None; }
        return Some(first_char.to_string());
    }

    let mut result: String = "\\".to_string();
    
    let Some(next_char_temp) = constant_chars.next() else { return None; };
    let mut next_char = next_char_temp;

    while next_char.is_letter() {
        result.push(next_char);
        let Some(next_char_temp) = constant_chars.next() else { return Some(result); };
        next_char = next_char_temp;
    }

    if !vec![' ', '_', '^'].contains(&next_char) { return None; }
    
    return Some(result);
}

fn get_valid_script(constant: String) -> Option<String> {
    let mut constant_chars = constant.chars();
    let Some(first_char) = constant_chars.next() else { return None; };

    if first_char != '{' { return None; }

    let mut result: String = "{".to_string();

    let Some(mut next_char) = constant_chars.next() else { return None; };

    while next_char.is_letter() || next_char.is_digit(10) {
        result.push(next_char);
        let Some(next_char_temp) = constant_chars.next() else { return None; };
        next_char = next_char_temp;
    }

    if next_char != '}' { return None; }
    
    result.push('}');
    return Some(result);
}
