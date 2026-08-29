pub trait CharVecToString {
    fn to_string_if_valid_f64(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

impl CharVecToString for Vec<char> {
    fn to_string(&self) -> String {
        self.into_iter().collect()
    }

    fn to_string_if_valid_f64(&self) -> Option<String> {
        let text: String = self.to_string();
        if text.parse::<f64>().is_err() {
            return None;
        }

        return Some(text);
    }
}

pub trait CharIsLetter {
    fn is_letter(&self) -> bool;
}

impl CharIsLetter for char {
    fn is_letter(&self) -> bool {
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'].contains(self)
    }
}


/*
*   DETECT EXPRESSIONS ON THE LEFT SIDE OF A STRING
*/

pub fn detect_expression_left(input: String) -> Option<String> {
    let mut input_chars = input.chars();
    let mut first_char = input_chars.next().unwrap();
    let mut left_padding = "".to_string();

    while first_char == ' ' {
        left_padding.push_str(" ");
        first_char = input_chars.next().unwrap();
    }

    let expression = match first_char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' | '-' => number_left(input),
        '(' => braces_left(input),
        '\\' => latex_variable_left(input),
        _ => return None,
    };

    if expression == None {
        return None;
    }
    return Some(left_padding + &expression.unwrap());
}

fn number_left(input: String) -> Option<String> {
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();

    expression.push(input_chars.next().unwrap());

    let current_char_wraped = input_chars.next();
    if current_char_wraped == None {
        return expression.to_string_if_valid_f64();
    }
    let mut current_char = current_char_wraped.unwrap();

    while vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&current_char) {
        expression.push(current_char);
        let current_char_wraped = input_chars.next();
        if current_char_wraped == None {
            break;
        }
        current_char = current_char_wraped.unwrap();
    }

    return expression.to_string_if_valid_f64();
}

fn braces_left(input: String) -> Option<String> {
    let mut opened_braces = 1;
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();
    expression.push(input_chars.next().unwrap());

    let mut current_char: char;

    while opened_braces > 0 {
        let current_char_wraped =  input_chars.next();
        if current_char_wraped == None {
            return None;
        }
        current_char = current_char_wraped.unwrap();

        match current_char {
            '(' => opened_braces += 1,
            ')' => opened_braces -= 1,
            _ => (),
        };
        expression.push(current_char);
    }

    return Some(expression.into_iter().collect());
}

fn latex_variable_left(input: String) -> Option<String> {
    let mut expression: String = "\\".to_string();

    let mut trimmed_input = input.clone();
    trimmed_input = trimmed_input.strip_prefix("\\").unwrap().to_string();

    let name = latex_variable_left_name(trimmed_input.clone());
    if name.is_none() {
        return None;
    }
    let name = &name.unwrap();

    trimmed_input = trimmed_input.strip_prefix(name).unwrap().to_string();
    expression.push_str(name);

    let current_char_wraped = trimmed_input.chars().next();
    if current_char_wraped.is_none() {
        return Some(expression);
    }
    let current_char = current_char_wraped.unwrap();
    
    if !(current_char == '_' || current_char == '^') {
        return Some(expression);
    }

    let mut detected_subscript: bool = false;
    if current_char == '_' {
        detected_subscript = true;
    }
    expression.push(current_char);
    trimmed_input = trimmed_input.strip_prefix(current_char).unwrap().to_string();

    let script = latex_variable_left_script(trimmed_input.clone());
    if script.is_none() {
        return None;
    }
    let script = &script.unwrap();

    trimmed_input = trimmed_input.strip_prefix(script).unwrap().to_string();
    expression.push_str(script);

    let current_char_wraped = trimmed_input.chars().next();
    if current_char_wraped.is_none() {
        return Some(expression);
    }
    let current_char = current_char_wraped.unwrap();

    if !((detected_subscript && current_char == '^') || (!detected_subscript && current_char == '_')) {
        return Some(expression);
    }
    expression.push(current_char);
    trimmed_input = trimmed_input.strip_prefix(current_char).unwrap().to_string();

    let script = latex_variable_left_script(trimmed_input.clone());
    if script.is_none() {
        return None;
    }
    let script = &script.unwrap();

    trimmed_input = trimmed_input.strip_prefix(script).unwrap().to_string();
    expression.push_str(script);

    return Some(expression);
}

fn latex_variable_left_name(input: String) -> Option<String> {
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();

    let current_char_wraped = input_chars.next();
    if current_char_wraped == None {
        return None;
    }
    let mut current_char = current_char_wraped.unwrap();

    while current_char.is_letter() {
        expression.push(current_char);
        let current_char_wraped = input_chars.next();
        if current_char_wraped == None {
            return Some(expression.to_string());
        }
        current_char = current_char_wraped.unwrap();
    }

    return Some(expression.to_string());
}

fn latex_variable_left_script(input: String) -> Option<String> {
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();

    let current_char_wraped = input_chars.next();
    if current_char_wraped.is_none() || current_char_wraped.unwrap() != '{' {
        return None;
    }
    expression.push('{');

    let current_char_wraped = input_chars.next();
    if current_char_wraped == None {
        return None;
    }
    let mut current_char = current_char_wraped.unwrap();

    while current_char.is_letter() || current_char.is_digit(10) {
        expression.push(current_char);
        let current_char_wraped = input_chars.next();
        if current_char_wraped == None {
            return None;
        }
        current_char = current_char_wraped.unwrap();
    }

    if current_char != '}' {
        return None;
    }
    expression.push(current_char);

    return Some(expression.to_string());
}


/*
*   DETECT EXPRESSIONS ON THE RIGHT SIDE OF A STRING
*/


pub fn detect_expression_right(input: String) -> Option<String> {
    return Some("".to_string());
}

#[cfg(test)]
mod char_vec_to_string {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(vec!['-', '1'].to_string(), "-1".to_string());
        assert_eq!(vec!['1'].to_string(), "1".to_string());
        assert_eq!(vec!['1', '.', '0'].to_string(), "1.0".to_string());
        assert_eq!(vec!['.', '5'].to_string(),".5".to_string());
        assert_eq!(Vec::<char>::new().to_string(), "".to_string());
        assert_eq!(vec!['a', 'b', 'c'].to_string(), "abc".to_string());
        assert_eq!(vec![' ', ' ', ' '].to_string(), "   ".to_string());
    }

    #[test]
    fn test_to_string_if_valid_f64() {
        assert_eq!(vec!['-', '1'].to_string_if_valid_f64(), Some("-1".to_string()));
        assert_eq!(vec!['1'].to_string_if_valid_f64(), Some("1".to_string()));
        assert_eq!(vec!['1', '.', '0'].to_string_if_valid_f64(), Some("1.0".to_string()));
        assert_eq!(vec!['.', '5'].to_string_if_valid_f64(),Some(".5".to_string()));
        assert_eq!(vec!['-', '1', '.', '.', '2'].to_string_if_valid_f64(), None);
        assert_eq!(vec!['a', '1'].to_string_if_valid_f64(), None);
        assert_eq!(vec!['-', '-', '1'].to_string_if_valid_f64(), None);
    }
}

#[cfg(test)]
mod char_is_letter_tests {
    use super::*;

    #[test]
    fn test_is_letter() {
        assert!(!'1'.is_letter());
        assert!(!'.'.is_letter());
        assert!(!'°'.is_letter());
        assert!(!')'.is_letter());
        assert!('a'.is_letter());
        assert!('A'.is_letter());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }

    #[test]
    fn test_number_left() {
        assert_eq!(number_left("10.0".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("1".to_string()), Some("1".to_string()));
        assert_eq!(number_left(".10".to_string()), Some(".10".to_string()));
        assert_eq!(number_left("0.10".to_string()), Some("0.10".to_string()));
        assert_eq!(number_left("10.0(sdfssd)".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0(10)".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("-10.0".to_string()), Some("-10.0".to_string()));
        assert_eq!(number_left("10.0 5".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0abd".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0+".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0-".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0*".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0/".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("10.0\\".to_string()), Some("10.0".to_string()));
        assert_eq!(number_left("1.0.0".to_string()), None);
    }

    #[test]
    fn test_braces_left() {
        assert_eq!(braces_left("(10 * 5)".to_string()), Some("(10 * 5)".to_string()));
        assert_eq!(braces_left("(10 * 5) + 3".to_string()), Some("(10 * 5)".to_string()));
        assert_eq!(braces_left("(10 * 5)\\latex".to_string()), Some("(10 * 5)".to_string()));
        assert_eq!(braces_left("(10 * (5))".to_string()), Some("(10 * (5))".to_string()));
        assert_eq!(braces_left("(10 * (3 + 5 -10.5)) + (8*3)".to_string()), Some("(10 * (3 + 5 -10.5))".to_string()));
        assert_eq!(braces_left("(1 + 3 + (5) ".to_string()), None);
        assert_eq!(braces_left("(".to_string()), None);
    }

    #[test]
    fn test_latex_variable_left() {
        assert_eq!(latex_variable_left("\\abcd".to_string()), Some("\\abcd".to_string()));
        assert_eq!(latex_variable_left("\\abcd_{1a}^{2b}".to_string()), Some("\\abcd_{1a}^{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{1a}_{2b}".to_string()), Some("\\abcd^{1a}_{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd_{1a}".to_string()), Some("\\abcd_{1a}".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{2b}".to_string()), Some("\\abcd^{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd ad".to_string()), Some("\\abcd".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{1a}_{2b} abcd".to_string()), Some("\\abcd^{1a}_{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd_{1a} abcd".to_string()), Some("\\abcd_{1a}".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{2b} abcd".to_string()), Some("\\abcd^{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd01".to_string()), Some("\\abcd".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{1a}_{2b}abcd".to_string()), Some("\\abcd^{1a}_{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd_{1a}abcd".to_string()), Some("\\abcd_{1a}".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{2b}abcd".to_string()), Some("\\abcd^{2b}".to_string()));
        assert_eq!(latex_variable_left("\\abcd^{.2b}_{01a}".to_string()), None);
        assert_eq!(latex_variable_left("\\abcd^{2b}_{01.a}".to_string()), None);
        assert_eq!(latex_variable_left("\\abcd01_{1a}^{2b}".to_string()), Some("\\abcd".to_string()));
    }

    #[test]
    fn test_latex_variable_left_name() {
        assert_eq!(latex_variable_left_name("abcd".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("ABCD".to_string()), Some("ABCD".to_string()));
        assert_eq!(latex_variable_left_name("abcd abcd".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("abcd_{xyz}".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("abcd^{xyz}".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("abcd0123".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("abcd.".to_string()), Some("abcd".to_string()));
        assert_eq!(latex_variable_left_name("".to_string()), None);
    }

    #[test]
    fn test_latex_variable_left_script() {
        assert_eq!(latex_variable_left_script("{abc}".to_string()), Some("{abc}".to_string()));
        assert_eq!(latex_variable_left_script("{123}".to_string()), Some("{123}".to_string()));
        assert_eq!(latex_variable_left_script("{a1b2c3}".to_string()), Some("{a1b2c3}".to_string()));
        assert_eq!(latex_variable_left_script("{a1b2c3} djhflshdq".to_string()), Some("{a1b2c3}".to_string()));
        assert_eq!(latex_variable_left_script("{a1b2c3}djhflshdq".to_string()), Some("{a1b2c3}".to_string()));
        assert_eq!(latex_variable_left_script("{a1b2c3".to_string()), None);
        assert_eq!(latex_variable_left_script("a{1b2c3}".to_string()), None);
        assert_eq!(latex_variable_left_script("a1b2c3djhflshdq".to_string()), None);
        assert_eq!(latex_variable_left_script("{a1b2c3.}".to_string()), None);
    }

}
