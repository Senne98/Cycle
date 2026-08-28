pub trait CharVecAsFloat {
    fn to_string_if_valid_f64(&self) -> Option<String>;
}

impl CharVecAsFloat for Vec<char> {
    fn to_string_if_valid_f64(&self) -> Option<String> {
        let text: String = self.into_iter().collect();
        if text.parse::<f64>().is_err() {
            return None;
        }

        return Some(text);
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

}
