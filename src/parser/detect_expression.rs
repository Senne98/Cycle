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
        '\\' => variable_left(input),
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
    let mut current_char = input_chars.next().unwrap();

    while vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&current_char) {
        expression.push(current_char);
        current_char = input_chars.next().unwrap();
    }

    let full_expression: String = expression.into_iter().collect();
    if full_expression.parse::<f64>().is_err() {
        return None;
    }

    return Some(full_expression);
}

fn braces_left(input: String) -> Option<String> {
    let mut opened_braces = 1;
    let mut input_chars = input.chars();
    let mut expression: Vec<char> = Vec::new();
    expression.push(input_chars.next().unwrap());

    let mut current_char: char;

    while opened_braces > 1 {
        current_char = input_chars.next().unwrap();
        match current_char {
            '(' => opened_braces += 1,
            ')' => opened_braces -= 1,
            _ => break,
        };
        expression.push(input_chars.next().unwrap());
    }

    return Some(expression.into_iter().collect());
}

fn variable_left(input: String) -> Option<String> {
    return Some("".to_string());
}

/*
*   DETECT EXPRESSIONS ON THE RIGHT SIDE OF A STRING
*/


pub fn detect_expression_right(input: String) -> Option<String> {
    return Some("".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }

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
}
