//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

use crate::parser::constants;

pub trait Node {
    fn result(&self) -> Option<f64>;
    fn to_string(&self) -> String;
}

// NullNode

pub struct NullNode {

}

impl NullNode {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Node for NullNode { 
    fn result(&self) -> Option<f64> {
        return None;
    }

    fn to_string(&self) -> String {
        "Null".to_string()
    }
}

// ValueNode

pub struct ValueNode {
    value: f64,
}

impl ValueNode {
    pub fn new(val: f64) -> ValueNode {
        Self {
            value: val,
        }
    }
}

impl Node for ValueNode {
    fn result(&self) -> Option<f64> {
        return Some(self.value);
    }

    fn to_string(&self) -> String {
        "value: ".to_string() + &self.value.to_string()
    }
}

// VariableNode

pub struct VariableNode {
    value: String,
}

impl VariableNode {
    pub fn new(val: String) -> VariableNode {
        Self {
            value: val,
        }
    }
}

impl Node for VariableNode {
    fn result(&self) -> Option<f64> {
        return constants::get_value_as_f64(&self.value);
    }

    fn to_string(&self) -> String {
        "variable: ".to_string() + &self.value
    }
}

// MultiplicationNode

pub struct MultiplicationNode {
    left_node: Box<dyn Node>,
    right_node: Box<dyn Node>,
}

impl MultiplicationNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> MultiplicationNode {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl Node for MultiplicationNode {
    fn result(&self) -> Option<f64> {
        let left = self.left_node.result();
        let right = self.right_node.result();

        if left == None || right == None {
            return None;
        }

        return Some(left.expect("left_node should be a f64") * right.expect("right_node should be a f64"));
    }

    fn to_string(&self) -> String {
        "multiplication: {".to_string() + &self.left_node.to_string() + " ," + &self.right_node.to_string() + "}"
    }
}


// DivisionNode

pub struct DivisionNode {
    left_node: Box<dyn Node>,
    right_node: Box<dyn Node>,
}

impl DivisionNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> DivisionNode {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl Node for DivisionNode {
    fn result(&self) -> Option<f64> {
        let left = self.left_node.result();
        let right = self.right_node.result();

        if left == None || right == None {
            return None;
        }

        return Some(left.expect("left_node should be a f64") / right.expect("right_node should be a f64"));
    }

    fn to_string(&self) -> String {
        "division: {".to_string() + &self.left_node.to_string() + " ," + &self.right_node.to_string() + "}"
    }
}

// AdditionNode

pub struct AdditionNode {
    left_node: Box<dyn Node>,
    right_node: Box<dyn Node>,
}

impl AdditionNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> AdditionNode {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl Node for AdditionNode {
    fn result(&self) -> Option<f64> {
        let left = self.left_node.result();
        let right = self.right_node.result();

        if left == None || right == None {
            return None;
        }

        return Some(left.expect("left_node should be a f64") + right.expect("right_node should be a f64"));
    }

    fn to_string(&self) -> String {
        "addition: {".to_string() + &self.left_node.to_string() + " ," + &self.right_node.to_string() + "}"
    }
}

// SubtractionNode

pub struct SubtractionNode {
    left_node: Box<dyn Node>,
    right_node: Box<dyn Node>,
}

impl SubtractionNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> SubtractionNode {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl Node for SubtractionNode {
    fn result(&self) -> Option<f64> {
        let left = self.left_node.result();
        let right = self.right_node.result();

        if left == None || right == None {
            return None;
        }

        return Some(left.expect("left_node should be a f64") - right.expect("right_node should be a f64"));
    }

    fn to_string(&self) -> String {
         "subtraction: {".to_string() + &self.left_node.to_string() + " ," + &self.right_node.to_string() + "}"
    }
}

// PowerNode

pub struct PowerNode {
    left_node: Box<dyn Node>,
    right_node: Box<dyn Node>,
}

impl PowerNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> PowerNode {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl Node for PowerNode {
    fn result(&self) -> Option<f64> {
        let left = self.left_node.result();
        let right = self.right_node.result();

        if left == None || right == None {
            return None;
        }

        return Some(left.expect("left_node should be a f64").powf(right.expect("right_node should be a f64")));
    }

    fn to_string(&self) -> String {
         "power: {".to_string() + &self.left_node.to_string() + " ," + &self.right_node.to_string() + "}"
    }
}

// SqrtNode

pub struct SqrtNode {
    node: Box<dyn Node>,
}

impl SqrtNode {
    pub fn new(node: Box<dyn Node>) -> SqrtNode {
        Self {
            node: node,
        }
    }
}

impl Node for SqrtNode {
    fn result(&self) -> Option<f64> {
        let Some(result) = self.node.result() else {
            return None;
        };

        return Some(result.sqrt());
    }

    fn to_string(&self) -> String {
        "sqrt: {".to_string() + &self.node.to_string() + "}"
    }
}

// LnNode

pub struct LnNode {
    node: Box<dyn Node>,
}

impl LnNode {
    pub fn new(node: Box<dyn Node>) -> LnNode {
        Self {
            node: node,
        }
    }
}

impl Node for LnNode {
    fn result(&self) -> Option<f64> {
        let Some(result) = self.node.result() else {
            return None;
        };

        return Some(result.ln());
    }

    fn to_string(&self) -> String {
        "ln: {".to_string() + &self.node.to_string() + "}"
    }
}

// LogNode

pub struct LogNode {
    node: Box<dyn Node>,
}

impl LogNode {
    pub fn new(node: Box<dyn Node>) -> LogNode {
        Self {
            node: node,
        }
    }
}

impl Node for LogNode {
    fn result(&self) -> Option<f64> {
        let Some(result) = self.node.result() else {
            return None;
        };

        return Some(result.log10());
    }

    fn to_string(&self) -> String {
        "log: {".to_string() + &self.node.to_string() + "}"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // ValueNode

    #[test]
    fn test_value_node_new() {
        let value_node = ValueNode::new(10.0);
        assert_eq!(value_node.value, 10.0);
    }

    #[test]
    fn test_value_node_result() {
        let value_node = ValueNode::new(10.01);
        assert_eq!(value_node.result(), Some(10.01));
    }

    // VariableNode

    #[test]
    fn test_variable_node_new() {
        //todo
    }

    #[test]
    fn test_variable_node_result() {
        //todo
    }

    // MultiplicationNode

    #[test]
    fn test_multiplication_node_new() {
        let left = Box::new(ValueNode::new(11.0));
        let right = Box::new(ValueNode::new(5.5));

        let node = MultiplicationNode::new(left, right);

        assert_eq!(node.left_node.result(), Some(11.0));
        assert_eq!(node.right_node.result(), Some(5.5));
    }

    #[test]
    fn test_multiplication_node_result() {
        let left = Box::new(ValueNode::new(11.0));
        let right = Box::new(ValueNode::new(5.5));

        let node = MultiplicationNode::new(left, right);

        assert_eq!(node.result(), Some(60.5));
    }

    // DivisionNode

    #[test]
    fn test_division_node_new() {
        let left = Box::new(ValueNode::new(0.0));
        let right = Box::new(ValueNode::new(57.98));

        let node = DivisionNode::new(left, right);

        assert_eq!(node.left_node.result(), Some(0.0));
        assert_eq!(node.right_node.result(), Some(57.98));
    }

    #[test]
    fn test_division_node_result() {
        let left = Box::new(ValueNode::new(7.5));
        let right = Box::new(ValueNode::new(0.5));

        let node = DivisionNode::new(left, right);

        assert_eq!(node.result(), Some(15.0));
    }

    // AdditionNode

    #[test]
    fn test_addition_node_new() {
        let left = Box::new(ValueNode::new(9.1));
        let right = Box::new(ValueNode::new(-0.3));

        let node = AdditionNode::new(left, right);

        assert_eq!(node.left_node.result(), Some(9.1));
        assert_eq!(node.right_node.result(), Some(-0.3));
    }

    #[test]
    fn test_addition_node_result() {
        let mut left = Box::new(ValueNode::new(5.3));
        let mut right = Box::new(ValueNode::new(0.5));

        let mut node = AdditionNode::new(left, right);

        assert_eq!(node.result(), Some(5.8));

        left = Box::new(ValueNode::new(5.3));
        right = Box::new(ValueNode::new(-0.5));

        node = AdditionNode::new(left, right);

        assert_eq!(node.result(), Some(4.8));

        left = Box::new(ValueNode::new(-5.3));
        right = Box::new(ValueNode::new(0.5));

        node = AdditionNode::new(left, right);

        assert_eq!(node.result(), Some(-4.8));
    }

    // SubtractionNode

    #[test]
    fn test_subtraction_node_new() {
        let left = Box::new(ValueNode::new(10000.1));
        let right = Box::new(ValueNode::new(99.5));

        let node = SubtractionNode::new(left, right);

        assert_eq!(node.left_node.result(), Some(10000.1));
        assert_eq!(node.right_node.result(), Some(99.5));
    }

    #[test]
    fn test_subtraction_node_result() {
        let mut left = Box::new(ValueNode::new(5.3));
        let mut right = Box::new(ValueNode::new(0.5));

        let mut node = SubtractionNode::new(left, right);

        assert_eq!(node.result(), Some(4.8));

        left = Box::new(ValueNode::new(5.3));
        right = Box::new(ValueNode::new(-0.5));

        node = SubtractionNode::new(left, right);

        assert_eq!(node.result(), Some(5.8));

        left = Box::new(ValueNode::new(-5.3));
        right = Box::new(ValueNode::new(0.5));

        node = SubtractionNode::new(left, right);

        assert_eq!(node.result(), Some(-5.8));
    }

}
