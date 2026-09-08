use crate::parser::parser_nodes::Node;
use crate::parser::parser_nodes::*;

use crate::parser::detect_expression::*;
use crate::parser::detect_expression::CharIsLetter;

use std::boxed::Box;

pub struct MathTree {
    root: Box<dyn Node>,
    input: Option<String>,
    result: Option<f64>,
}

impl MathTree {

    pub fn new(node: Box<dyn Node>) -> Self {
        Self {
            root: node,
            input: None,
            result: None,
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = Some(standardize_expression(input));
        self.input = add_braces(self.input.clone().unwrap());
        if !self.input.is_none() {
            let root = build_tree(self.input.clone().unwrap());
            if root.is_none() {
                self.root = Box::new(NullNode::new());
            } else {
                self.root = root.unwrap();
            }
            self.result = self.root.result();
        }
    }

    pub fn get_result(&self) -> Option<f64> {
        self.result.clone()
    }
}

/*
* Adding braces to preserver order of operations
*/

fn standardize_expression(input: String) -> String {
    let input = input.replace("**", "^");
    let input = input.replace("\\cdot", "*");
    let input = input.replace("- ", "\u{0} ");
    return input;
}

fn add_braces(input: String) -> Option<String> {
    let Some(result) = add_braces_for_symbol(input, &['^']) else { return None; };
    let Some(result) = add_braces_for_symbol(result, &['*', '/']) else { return None; };
    let Some(result) = add_braces_for_symbol(result, &['+', '\u{0}']) else { return None; };
    return Some(result);
}

fn add_braces_for_symbol(input: String, symbols: &[char]) -> Option<String> {
    let Some((left_temp, right_temp)) = input.split_once(symbols) else {
        return Some(input);
    };

    let mut left = left_temp.to_string();
    let mut right = right_temp.to_string();
    let mut symbol = input.strip_prefix(&left).unwrap().chars().next().unwrap();

    loop {
        let Some(left_expression) = detect_expression_right(left.to_string()) else { return None; };

        left = left.strip_suffix(&left_expression).unwrap().to_string();
        left.push_str("(");
        left.push_str(&left_expression);

        left.push(symbol);

        let Some(right_expression) = detect_expression_left(right.to_string()) else { return None; };

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
        right = right_temp.to_string();
    }
}

/*
* Build math tree
*/

fn build_tree(expression: String) -> Option<Box<dyn Node>> {
    if expression == "" {
        return None;
    }

    let expression = remove_outer_braces(expression);
        
    let Some(left_expression) = detect_expression_left(expression.clone()) else { return None; };
    let Some(left_node) = get_node(left_expression.clone()) else { return None; };
    
    let expression = expression.strip_prefix(&left_expression).unwrap().trim();
    let Some(symbol) = expression.chars().next() else { return Some(left_node); };
    
    let right_expression = expression.strip_prefix(symbol).unwrap().to_string();
    let Some(right_node) = get_node(right_expression) else { return None; };

    match symbol {
        '*' => return Some(Box::new(MultiplicationNode::new(left_node, right_node))),
        '/' => return Some(Box::new(DivisionNode::new(left_node, right_node))),
        '+' => return Some(Box::new(AdditionNode::new(left_node, right_node))),
        '\u{0}' => return Some(Box::new(SubtractionNode::new(left_node, right_node))),
        '^' => return Some(Box::new(PowerNode::new(left_node, right_node))),
        _ => return None,
    }
}

fn remove_outer_braces(expression: String) -> String {
    let mut expression = expression.trim().to_string();

    let mut has_outer_braces: bool = true;
    while has_outer_braces {
        if !(expression.starts_with("(") && expression.ends_with(")")) {
            break;
        }

        let mut expr_iter = expression.chars();
        expr_iter.next();
        expr_iter.next_back();

        let mut remove_braces: bool = true;
        let mut braces_counter = 0;
        for c in expr_iter {
            match c {
                '(' => braces_counter += 1,
                ')' => braces_counter -= 1,
                _ => (),
            }
            if braces_counter < 0 {
                remove_braces = false;
            }
        }

        if remove_braces {
            expression = expression.strip_prefix("(").unwrap().strip_suffix(")").unwrap().to_string();
        } else {
            has_outer_braces = false;
        }
    }

    return expression.trim().to_string();
}

fn get_node(expression: String) -> Option<Box<dyn Node>> {
    let expression = expression.trim().to_string();
    let as_f64 = expression.parse::<f64>();

    if !as_f64.is_err() {
        return Some(Box::new(ValueNode::new(as_f64.unwrap())));
    } 
    if expression.starts_with("(") && expression.ends_with(")") {
        return build_tree(expression);
    }
    return get_var_or_single_expression(expression);
}

fn get_var_or_single_expression(expression: String) -> Option<Box<dyn Node>> {
    let Some(first_char) = expression.trim().chars().next() else { return None; };
    
    if first_char == '\\' {
        return  get_variable_node_latex(expression);
    }

    return get_other_node(expression);
}

fn get_other_node(expression: String) -> Option<Box<dyn Node>> {
    let expression = expression.trim().to_string();
    if !expression.chars().next().unwrap().is_letter() {
        return None;
    }

    if expression.len() == 1 {
        return Some(Box::new(VariableNode::new(expression)));
    }

    if !expression.ends_with(")") {
        return None;
    }

    if expression.starts_with("sqrt(") {
        let Some(inside) = detect_expression_left(expression.clone().strip_prefix("sqrt(").unwrap().strip_suffix(")").unwrap().to_string()) else { return None; };
        let Some(inside_node) = get_node(inside) else { return None; };
        return Some(Box::new(SqrtNode::new(inside_node)));
    }
    if expression.starts_with("ln(") {
        let Some(inside) = detect_expression_left(expression.clone().strip_prefix("ln").unwrap().to_string()) else { return None; };
        let Some(inside_node) = get_node(inside) else { return None; };
        return Some(Box::new(LnNode::new(inside_node)));
    }
    if expression.starts_with("log(") {
        let Some(inside) = detect_expression_left(expression.clone().strip_prefix("log").unwrap().to_string()) else { return None; };
        let Some(inside_node) = get_node(inside) else { return None; };
        return Some(Box::new(LogNode::new(inside_node)));
    }
    if expression.starts_with("exp(") {
        let Some(inside) = detect_expression_left(expression.clone().strip_prefix("exp").unwrap().to_string()) else { return None; };
        let Some(inside_node) = get_node(inside) else { return None; };
        return Some(Box::new(PowerNode::new(Box::new(VariableNode::new("e".to_string())), inside_node)));
    }
    return None;
}

fn get_variable_node_latex(expression: String) -> Option<Box<dyn Node>> {
    let expression = expression.trim().to_string();

    let Some(latex) = latex_variable_left(expression.clone()) else { return None; };

    if latex != expression {
        return None;
    }

    return Some(Box::new(VariableNode::new(expression)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standardize_expression() {
        assert_eq!(standardize_expression("5 - 3".to_string()), "5 \u{0} 3".to_string());
        assert_eq!(standardize_expression("5 ** 3".to_string()), "5 ^ 3".to_string());
        assert_eq!(standardize_expression("5 \\cdot 3".to_string()), "5 * 3".to_string());
        assert_eq!(standardize_expression("5 + -3".to_string()), "5 + -3".to_string());
        assert_eq!(standardize_expression("5 - -3".to_string()), "5 \u{0} -3".to_string());
    }
    
    #[test]
    fn test_add_braces() {
        assert_eq!(add_braces("2 + 3".to_string()), Some("(2 + 3)".to_string()));
        assert_eq!(add_braces("2 \u{0} 3".to_string()), Some("(2 \u{0} 3)".to_string()));
        assert_eq!(add_braces("2 * 3".to_string()), Some("(2 * 3)".to_string()));
        assert_eq!(add_braces("2 / 3".to_string()), Some("(2 / 3)".to_string()));
        assert_eq!(add_braces("2 ^ 3".to_string()), Some("(2 ^ 3)".to_string()));
        assert_eq!(add_braces("2^3 + 5 * 8 \u{0} 0.05 / 3.25".to_string()), Some("(((2^3) +( 5 * 8)) \u{0}( 0.05 / 3.25))".to_string()));
        assert_eq!(add_braces("5 *0.3 \u{0} 8^(5-0.3) * exp(5)".to_string()), Some("((5 *0.3) \u{0}(( 8^(5-0.3)) * exp(5)))".to_string()));
        assert_eq!(add_braces("(5 + 3 \u{0} exp(8 * (3 \u{0} 5))) / sqrt(2) + \\latex_{5a} ^-3.9".to_string()).unwrap().replace(" ", ""), "(((((5 + 3) \u{0} exp((8 * ((3 \u{0} 5)))))) / sqrt(2)) + (\\latex_{5a} ^-3.9))".replace(" ", ""));
    }

    #[test]
    fn test_add_braces_for_symbol() {
        assert_eq!(add_braces_for_symbol("10.0^2 + 3".to_string(), &['^']), Some("(10.0^2) + 3".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2 + 3)".to_string(), &['^']), Some("(10.0^(2 + 3))".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^2^3".to_string(), &['^']), Some("((10.0^2)^3)".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2^3)".to_string(), &['^']), Some("(10.0^((2^3)))".to_string()));
        assert_eq!(add_braces_for_symbol("10.0^(2 + 5^3)".to_string(), &['^']), Some("(10.0^(2 +( 5^3)))".to_string()));
    }

    #[test]
    fn test_build_tree() {
        assert_eq!(build_tree("(5+3.9)".to_string()).unwrap().to_string(), AdditionNode::new(
            Box::new(ValueNode::new(5.0)), 
            Box::new(ValueNode::new(3.9))).to_string());
        assert_eq!(build_tree("(5\u{0}3.9)".to_string()).unwrap().to_string(), SubtractionNode::new(
            Box::new(ValueNode::new(5.0)), 
            Box::new(ValueNode::new(3.9))).to_string());
        assert_eq!(build_tree("(5*3.9)".to_string()).unwrap().to_string(), MultiplicationNode::new(
            Box::new(ValueNode::new(5.0)), 
            Box::new(ValueNode::new(3.9))).to_string());
        assert_eq!(build_tree("(5/3.9)".to_string()).unwrap().to_string(), DivisionNode::new(
            Box::new(ValueNode::new(5.0)), 
            Box::new(ValueNode::new(3.9))).to_string());
        assert_eq!(build_tree("(3 \u{0} exp(8 * (3 \u{0} 5)))".to_string()).unwrap().to_string(), SubtractionNode::new(
            Box::new(ValueNode::new(3.0)), 
            Box::new(PowerNode::new(
                Box::new(VariableNode::new("e".to_string())),
                Box::new(MultiplicationNode::new(
                    Box::new(ValueNode::new(8.0)),
                    Box::new(SubtractionNode::new(
                        Box::new(ValueNode::new(3.0)),
                        Box::new(ValueNode::new(5.0))
                    ))
                ))
            )))
            .to_string());
        assert_eq!(build_tree("(\\latex_{5a} ^-3.9)".to_string()).unwrap().to_string(), PowerNode::new(
                Box::new(VariableNode::new("\\latex_{5a}".to_string())),
                Box::new(ValueNode::new(-3.9))
            ).to_string());



        assert_eq!(build_tree("((((5 + (3 \u{0} exp((8 * ((3 \u{0} 5))))))) / sqrt(2)) + (\\latex_{5a} ^-3.9))".to_string()).unwrap().to_string(), AdditionNode::new(
            Box::new(DivisionNode::new(
                Box::new(AdditionNode::new(
                    Box::new(ValueNode::new(5.0)),
                    Box::new(SubtractionNode::new(
                        Box::new(ValueNode::new(3.0)),
                        Box::new(PowerNode::new(
                            Box::new(VariableNode::new("e".to_string())),
                            Box::new(MultiplicationNode::new(
                                Box::new(ValueNode::new(8.0)),
                                Box::new(SubtractionNode::new(
                                    Box::new(ValueNode::new(3.0)),
                                    Box::new(ValueNode::new(5.0))
                                ))
                            ))
                        ))
                    ))
                )),
                Box::new(SqrtNode::new(Box::new(ValueNode::new(2.0)))))),
            Box::new(PowerNode::new(
                Box::new(VariableNode::new("\\latex_{5a}".to_string())),
                Box::new(ValueNode::new(-3.9))
            ))).to_string());
    }

    #[test]
    fn test_remove_outer_braces() {
        assert_eq!(remove_outer_braces("(5 + 10)".to_string()), "5 + 10".to_string());
        assert_eq!(remove_outer_braces("(5 + 10/5 * (3 \u{0} 5))".to_string()), "5 + 10/5 * (3 \u{0} 5)".to_string());
        assert_eq!(remove_outer_braces("((5 + 10))".to_string()), "5 + 10".to_string());
        assert_eq!(remove_outer_braces("(5 + 3) + (10)".to_string()), "(5 + 3) + (10)".to_string());
        assert_eq!(remove_outer_braces("(5 + 3) + 10".to_string()), "(5 + 3) + 10".to_string());
        assert_eq!(remove_outer_braces("(5 )".to_string()), "5".to_string());
        assert_eq!(remove_outer_braces("(3 \u{0} exp(8 * (3 \u{0} 5)))".to_string()), "3 \u{0} exp(8 * (3 \u{0} 5))".to_string());
    }

    #[test]
    fn test_get_node() {
        assert_eq!(get_node("-5.305".to_string()).unwrap().to_string(), ValueNode::new(-5.305).to_string());
        assert_eq!(get_node("(-5.305 * 5) / (3 \u{0} 8.65)".to_string()).unwrap().to_string(), build_tree("(-5.305 * 5) / (3 \u{0} 8.65)".to_string()).unwrap().to_string());
        assert_eq!(get_node("e".to_string()).unwrap().to_string(), get_var_or_single_expression("e".to_string()).unwrap().to_string());
        assert_eq!(get_node("sqrt(-5.305 * 5)".to_string()).unwrap().to_string(), get_var_or_single_expression("sqrt(-5.305 * 5)".to_string()).unwrap().to_string());
        assert_eq!(get_node("\\latex_{5}".to_string()).unwrap().to_string(), get_var_or_single_expression("\\latex_{5}".to_string()).unwrap().to_string());
        assert_eq!(get_node("(8 * (3 \u{0} 5))".to_string()).unwrap().to_string(), MultiplicationNode::new(
            Box::new(ValueNode::new(8.0)),
            Box::new(SubtractionNode::new(
                Box::new(ValueNode::new(3.0)),
                Box::new(ValueNode::new(5.0))
            ))).to_string());

    }

    #[test]
    fn test_get_var_or_single_expression() {
        assert_eq!(get_var_or_single_expression("\\latex_{0}".to_string()).unwrap().to_string(), get_variable_node_latex("\\latex_{0}".to_string()).unwrap().to_string());
        assert_eq!(get_var_or_single_expression("e".to_string()).unwrap().to_string(), get_other_node("e".to_string()).unwrap().to_string());
        assert_eq!(get_var_or_single_expression("sqrt(-5.3)".to_string()).unwrap().to_string(), get_other_node("sqrt(-5.3)".to_string()).unwrap().to_string());
        
        assert!(get_var_or_single_expression("ea".to_string()).is_none());
        assert!(get_var_or_single_expression(". ".to_string()).is_none());
    }

    #[test]
    fn test_get_other_node() {
        assert_eq!(get_other_node(" t".to_string()).unwrap().to_string(), VariableNode::new("t".to_string()).to_string());
        assert_eq!(get_other_node(" sqrt( 5)".to_string()).unwrap().to_string(), SqrtNode::new(Box::new(ValueNode::new(5.0))).to_string());
        assert_eq!(get_other_node(" ln( (5 + 3) )".to_string()).unwrap().to_string(), LnNode::new(Box::new(AdditionNode::new(Box::new(ValueNode::new(5_f64)), Box::new(ValueNode::new(3_f64))))).to_string());
        assert_eq!(get_other_node(" log( 5)".to_string()).unwrap().to_string(), LogNode::new(Box::new(ValueNode::new(5_f64))).to_string());
        assert_eq!(get_other_node(" exp( 5)".to_string()).unwrap().to_string(), PowerNode::new(Box::new(VariableNode::new("e".to_string())), Box::new(ValueNode::new(5_f64))).to_string());
        
        assert!(get_other_node("ab".to_string()).is_none());
        assert!(get_other_node("0".to_string()).is_none());
    }

    #[test]
    fn test_get_variable_node_latex() {
        assert_eq!(get_variable_node_latex(" \\test ".to_string()).unwrap().to_string(), VariableNode::new("\\test".to_string()).to_string());
        assert_eq!(get_variable_node_latex("\\test_{0a}".to_string()).unwrap().to_string(), VariableNode::new("\\test_{0a}".to_string()).to_string());
        assert_eq!(get_variable_node_latex("\\test^{1b}".to_string()).unwrap().to_string(), VariableNode::new("\\test^{1b}".to_string()).to_string());
        assert_eq!(get_variable_node_latex("\\test^{1b}_{0a}".to_string()).unwrap().to_string(), VariableNode::new("\\test^{1b}_{0a}".to_string()).to_string());
        assert_eq!(get_variable_node_latex("\\test_{1b}^{0a}".to_string()).unwrap().to_string(), VariableNode::new("\\test_{1b}^{0a}".to_string()).to_string());
 
        assert!(get_variable_node_latex("\\test a".to_string()).is_none());
        assert!(get_variable_node_latex("\\test_{a.f}".to_string()).is_none());
        assert!(get_variable_node_latex("\\test^".to_string()).is_none());
        assert!(get_variable_node_latex("\\test0".to_string()).is_none());
    }
}
