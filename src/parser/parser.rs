use crate::parser::parser_nodes::{Node};
use crate::parser::detect_expression::{detect_expression_left, detect_expression_right};

struct MathTree<'a> {
    root: &'a dyn Node,
    input: String,
}

impl<'a> MathTree<'a> {

    pub fn new(node: &'a dyn Node) -> Self {
        Self {
            root: node,
            input: "".to_string(),
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = standardize_expression(input);
        self.input = add_braces(self.input.clone());
    }
}

fn standardize_expression(input: String) -> String {
    let input = input.replace("**", "^");
    let input = input.replace("\\cdot", "*");
    return input;
}

fn add_braces(input: String) -> String {
    let result = add_braces_for_symbol(input, '^');
    return result;
}

fn add_braces_for_symbol(input: String, symbol: char) -> String {
    let mut split = input.split(symbol);
    let mut result = "".to_string();

    let mut left = split.next().unwrap();
    let mut right: &str;

    for s in split {
        right = s;
        
        //handle None
        let left_expression = detect_expression_right(left.to_string()).unwrap();
        result.push_str(&left.strip_prefix(&left_expression).unwrap());
        result.push_str("(");
        result.push_str(&left_expression);
        result.push(symbol);

        //handle None
        let right_expression = detect_expression_left(right.to_string()).unwrap();
        result.push_str(&right_expression);
        result.push_str(")");
        right = right.strip_suffix(&right_expression).unwrap();

        left = right;
    }

    result.push_str(left);
    return result;
}
