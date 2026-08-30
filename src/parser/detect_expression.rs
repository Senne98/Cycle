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
    let mut first_char = input_chars.next();
    let mut left_padding = "".to_string();

    while first_char == Some(' ') {
        left_padding.push_str(" ");
        first_char = input_chars.next();
    }

    let Some(first_char) = first_char else {
        return Some(left_padding);
    };

    let unpadded_input = input.strip_prefix(&left_padding).unwrap().to_string();

    let expression = match first_char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' | '-' => number_left(unpadded_input),
        '(' => braces_left(unpadded_input),
        '\\' => latex_variable_left(unpadded_input),
        _ => check_operators_left(unpadded_input),
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

    expression.push_str(script);

    return Some(expression);
}

fn latex_variable_left_name(input: String) -> Option<String> { 
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();

    let current_char_wraped = input_chars.next();
    if current_char_wraped == None || !current_char_wraped.unwrap().is_letter() {
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

//checks for operators such as sqrt, or single char variable
fn check_operators_left(input: String) -> Option<String> {
    let mut input_chars = input.chars();

    let current_char_wraped = input_chars.next();
    if current_char_wraped.is_none() {
        return None;
    }

    let first_char = current_char_wraped.unwrap();
    if !first_char.is_letter() {
        return None;
    }

    let current_char_wraped = input_chars.next();
    if current_char_wraped.is_none() || current_char_wraped.unwrap() == ' ' {
        return Some(first_char.to_string());
    }
    
    if input.starts_with("sqrt(") {
        let expression = braces_left(input.strip_prefix("sqrt").unwrap().to_string());
        if expression.is_none() {
            return None;
        }
        let mut result = "sqrt".to_string();
        result.push_str(&expression.unwrap());
        return Some(result);
    }

    return None;
}

/*
*   DETECT EXPRESSIONS ON THE RIGHT SIDE OF A STRING
*/


pub fn detect_expression_right(input: String) -> Option<String> {
    let mut input_chars = input.chars();
    let mut first_char = input_chars.next_back();
    let mut right_padding = "".to_string();

    while first_char == Some(' ') {
        right_padding.push_str(" ");
        first_char = input_chars.next_back();
    }

    if first_char.is_none() {
        return None;
    };

    let unpadded_input = input.strip_suffix(&right_padding).unwrap().to_string();

    let normalized = unpadded_input.replace("- ", "\u{0}");
    let mut iter = normalized.split(|c| {c == '\u{0}' || c == '+' || c == '*' || c == '/' || c == '^' || c == '('});

    let mut last = iter.next_back();
    if last.is_none() {
        return None;
    }
    let mut expression: String = last.unwrap().to_string();
    let test_expr = detect_expression_left(expression.clone());

    if !test_expr.is_none() && test_expr.unwrap() == expression {
        return Some(expression + &right_padding);
    }

    last = iter.next_back();
        
    while last != None {
        let old_expression = expression.clone();
        expression = last.unwrap().to_string();

        let operator = unpadded_input.clone().strip_suffix(&old_expression.clone()).unwrap().chars().last().unwrap();
        if operator == ' ' {
            expression.push_str("- ");
        } else {
            expression.push(operator);
        }

        expression.push_str(&old_expression);

        let test_expr = detect_expression_left(expression.clone());
        if !test_expr.is_none() && test_expr.unwrap() == expression {
            return Some(expression + &right_padding);
        }

        last = iter.next_back();
    }

    return None;
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
    fn test_detect_expression_left() {
        assert_eq!(detect_expression_left(" 10.0".to_string()), Some(" 10.0".to_string()));
        assert_eq!(detect_expression_left("-10.0".to_string()), Some("-10.0".to_string()));
        assert_eq!(detect_expression_left("(5 * 10 + 3)".to_string()), Some("(5 * 10 + 3)".to_string()));
        assert_eq!(detect_expression_left("\\test_{5}".to_string()), Some("\\test_{5}".to_string()));

        assert_eq!(detect_expression_left("10.0 + (5 * \\test)".to_string()), Some("10.0".to_string()));
        assert_eq!(detect_expression_left("-10.0 * sqrt(5)".to_string()), Some("-10.0".to_string()));
        assert_eq!(detect_expression_left("(5 * 10 + 3) - 1.3".to_string()), Some("(5 * 10 + 3)".to_string()));
        assert_eq!(detect_expression_left("\\test_{5} + 2".to_string()), Some("\\test_{5}".to_string()));

        assert_eq!(detect_expression_left("1.0. + 2".to_string()), None);
        assert_eq!(detect_expression_left("\\01 + (5 * 10)".to_string()), None);
        assert_eq!(detect_expression_left("sqrt(10 + (1 -3)".to_string()), None);
        assert_eq!(detect_expression_left(". + a + 10".to_string()), None);
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
        assert_eq!(latex_variable_left_name("01a".to_string()), None);
        assert_eq!(latex_variable_left_name(".a".to_string()), None);
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

    #[test]
    fn test_check_operators_left() {
        // single char variable
        assert_eq!(check_operators_left("a".to_string()), Some("a".to_string()));
        assert_eq!(check_operators_left("A".to_string()), Some("A".to_string()));
        assert_eq!(check_operators_left("a ".to_string()), Some("a".to_string()));
        assert_eq!(check_operators_left("A ".to_string()), Some("A".to_string()));
        assert_eq!(check_operators_left("a abc".to_string()), Some("a".to_string()));
        assert_eq!(check_operators_left("A abc".to_string()), Some("A".to_string()));
        assert_eq!(check_operators_left(".aaz ".to_string()), None);
        assert_eq!(check_operators_left("011245a ".to_string()), None);

        //sqrt
        assert_eq!(check_operators_left("sqrt()".to_string()), Some("sqrt()".to_string()));
        assert_eq!(check_operators_left("sqrt(10 * 5)".to_string()), Some("sqrt(10 * 5)".to_string()));
        assert_eq!(check_operators_left("sqrt(5)abcd ".to_string()), Some("sqrt(5)".to_string()));
        assert_eq!(check_operators_left("sqrt(( 10)".to_string()), None);
        assert_eq!(check_operators_left("sqrt".to_string()), None);
        
        //general false
        assert_eq!(check_operators_left("test".to_string()), None);
    }

    #[test]
    fn test_detect_expression_right() {
        //assert_eq!(detect_expression_right("(10 * 5)".to_string()), Some("(10 * 5)".to_string()));
        assert_eq!(detect_expression_right("(10 * 5) + 3".to_string()), Some(" 3".to_string()));
        assert_eq!(detect_expression_right("(10 * 5) + \\latex ".to_string()), Some(" \\latex ".to_string()));
        assert_eq!(detect_expression_right("(10 * (5))".to_string()), Some("(10 * (5))".to_string()));
        assert_eq!(detect_expression_right("(10 * (3 + 5 -10.5)) + (8*3)".to_string()), Some(" (8*3)".to_string()));
        assert_eq!(detect_expression_right("1+ sqrt(5)".to_string()), Some(" sqrt(5)".to_string()));
        assert_eq!(detect_expression_right("(10.0^(2".to_string()), Some("2".to_string()));
        assert_eq!(detect_expression_right("(".to_string()), Some("".to_string()));

    }
}
