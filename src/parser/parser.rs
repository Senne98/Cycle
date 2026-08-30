use crate::parser::parser_nodes::{Node};
use crate::parser::detect_expression::{detect_expression_left, detect_expression_right};

struct MathTree<'a> {
    root: &'a dyn Node,
    input: Option<String>,
}

impl<'a> MathTree<'a> {

    pub fn new(node: &'a dyn Node) -> Self {
        Self {
            root: node,
            input: None,
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = Some(standardize_expression(input));
        if !self.input.is_none() {
            self.input = add_braces(self.input.clone().unwrap());
        }
    }
}

fn standardize_expression(input: String) -> String {
    let input = input.replace("**", "^");
    let input = input.replace("\\cdot", "*");
    return input;
}

fn add_braces(input: String) -> Option<String> {
    let Some(result) = add_braces_for_symbol(input, &['^']) else {
        return None;
    };
    let Some(result) = add_braces_for_symbol(result, &['*', '/']) else {
        return None;
    };
    let normalized = result.replace("- ", "\u{0}");
    let result = add_braces_for_symbol(normalized, &['+', '\u{0}']);
    return result;
}

fn add_braces_for_symbol(input: String, symbols: &[char]) -> Option<String> {
    let Some((left_temp, right_temp)) = input.split_once(symbols) else {
        return Some(input);
    };

    let mut left = left_temp.clone().to_string();
    let mut right = right_temp.clone().to_string();
    let mut symbol = input.strip_prefix(&left).unwrap().chars().next().unwrap();

    println!("left = {:?}", left);
    println!("right = {:?}", right);

    loop {
        let Some(left_expression) = detect_expression_right(left.to_string()) else {
            return None;
        };

        println!("l_ex = {:?}", left_expression);

        left = left.strip_suffix(&left_expression).unwrap().to_string();
        left.push_str("(");
        left.push_str(&left_expression);

        left.push(symbol);

        let Some(right_expression) = detect_expression_left(right.to_string()) else {
            return None;
        };

        println!("r_exp = {:?}", right_expression);

        let right_clone = right.clone();
        let right_residue = right_clone.strip_prefix(&right_expression).unwrap();
        right = right_expression.to_string();
        right.push_str(")");
        right.push_str(right_residue);
        
        let Some((left_temp, right_temp)) = right.split_once(symbols) else {
            let mut result = left.clone();
            result.push_str(&right);
            return Some(result);
        };

        symbol = right.strip_prefix(&left_temp).unwrap().chars().next().unwrap();

        left.push_str(left_temp); 
        right = right_temp.clone().to_string();

        println!("left = {:?}", left);
        println!("right = {:?}", right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_braces_for_symbol() {
        assert_eq!(add_braces_for_symbol("10.0^2 + 3".to_string(), &['^']), Some("(10.0^2) + 3".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2 + 3)".to_string(), &['^']), Some("(10.0^(2 + 3))".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^2^3".to_string(), &['^']), Some("((10.0^2)^3)".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2^3)".to_string(), &['^']), Some("(10.0^((2^3)))".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2 + 5^3)".to_string(), &['^']), Some("(10.0^(2 +( 5^3)))".to_string()));
    }
}
