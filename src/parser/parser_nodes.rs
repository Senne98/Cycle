

pub trait Node {
    fn result(&self) -> f64;
}

pub trait OperationNode {
    fn calculate(&self) -> f64;
}

impl<T: OperationNode> Node for T {
    fn result(&self) -> f64 {
        return self.calculate();
    }
}

// ValueNode

struct ValueNode {
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
    fn result(&self) -> f64 {
        return self.value;
    }
}

// MultiplicationNode

struct MultiplicationNode<'a> {
    left_node: &'a dyn Node,
    right_node: &'a dyn Node,
}

impl<'a> MultiplicationNode<'a> {
    pub fn new(left: &'a dyn Node, right: &'a dyn Node) -> MultiplicationNode<'a> {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl OperationNode for MultiplicationNode<'_> {
    fn calculate(&self) -> f64 {
        return self.left_node.result() * self.right_node.result();
    }
}

// DivisionNode

struct DivisionNode<'a> {
    left_node: &'a dyn Node,
    right_node: &'a dyn Node,
}

impl<'a> DivisionNode<'a> {
    pub fn new(left: &'a dyn Node, right: &'a dyn Node) -> DivisionNode<'a> {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl OperationNode for DivisionNode<'_> {
    fn calculate(&self) -> f64 {
        return self.left_node.result() / self.right_node.result();
    }
}

// AdditionNode

struct AdditionNode<'a> {
    left_node: &'a dyn Node,
    right_node: &'a dyn Node,
}

impl<'a> AdditionNode<'a> {
    pub fn new(left: &'a dyn Node, right: &'a dyn Node) -> AdditionNode<'a> {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl OperationNode for AdditionNode<'_> {
    fn calculate(&self) -> f64 {
        return self.left_node.result() + self.right_node.result();
    }
}

// SubtractionNode

struct SubtractionNode<'a> {
    left_node: &'a dyn Node,
    right_node: &'a dyn Node,
}

impl<'a> SubtractionNode<'a> {
    pub fn new(left: &'a dyn Node, right: &'a dyn Node) -> SubtractionNode<'a> {
        Self {
            left_node: left,
            right_node: right,
        }
    }
}

impl OperationNode for SubtractionNode<'_> {
    fn calculate(&self) -> f64 {
        return self.left_node.result() - self.right_node.result();
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
        assert_eq!(value_node.result(), 10.01);
    }

    // MultiplicationNode

    #[test]
    fn test_multiplication_node_new() {
        let left = ValueNode::new(11.0);
        let right = ValueNode::new(5.5);

        let node = MultiplicationNode::new(&left, &right);

        assert_eq!(node.left_node.result(), 11.0);
        assert_eq!(node.right_node.result(), 5.5);
    }

    #[test]
    fn test_multiplication_node_result() {
        let left = ValueNode::new(11.0);
        let right = ValueNode::new(5.5);

        let node = MultiplicationNode::new(&left, &right);

        assert_eq!(node.result(), 60.5);
    }

    // DivisionNode

    #[test]
    fn test_division_node_new() {
        let left = ValueNode::new(0.0);
        let right = ValueNode::new(57.98);

        let node = DivisionNode::new(&left, &right);

        assert_eq!(node.left_node.result(), 0.0);
        assert_eq!(node.right_node.result(), 57.98);
    }

    #[test]
    fn test_division_node_result() {
        let left = ValueNode::new(7.5);
        let right = ValueNode::new(0.5);

        let node = DivisionNode::new(&left, &right);

        assert_eq!(node.result(), 15.0);
    }

    // AdditionNode

    #[test]
    fn test_addition_node_new() {
        let left = ValueNode::new(9.1);
        let right = ValueNode::new(-0.3);

        let node = AdditionNode::new(&left, &right);

        assert_eq!(node.left_node.result(), 9.1);
        assert_eq!(node.right_node.result(), -0.3);
    }

    #[test]
    fn test_addition_node_result() {
        let mut left = ValueNode::new(5.3);
        let mut right = ValueNode::new(0.5);

        let mut node = AdditionNode::new(&left, &right);

        assert_eq!(node.result(), 5.8);

        left = ValueNode::new(5.3);
        right = ValueNode::new(-0.5);

        node = AdditionNode::new(&left, &right);

        assert_eq!(node.result(), 4.8);

        left = ValueNode::new(-5.3);
        right = ValueNode::new(0.5);

        node = AdditionNode::new(&left, &right);

        assert_eq!(node.result(), -4.8);
    }

    // SubtractionNode

    #[test]
    fn test_subtraction_node_new() {
        let left = ValueNode::new(10000.1);
        let right = ValueNode::new(99.5);

        let node = SubtractionNode::new(&left, &right);

        assert_eq!(node.left_node.result(), 10000.1);
        assert_eq!(node.right_node.result(), 99.5);
    }

    #[test]
    fn test_subtraction_node_result() {
        let mut left = ValueNode::new(5.3);
        let mut right = ValueNode::new(0.5);

        let mut node = SubtractionNode::new(&left, &right);

        assert_eq!(node.result(), 4.8);

        left = ValueNode::new(5.3);
        right = ValueNode::new(-0.5);

        node = SubtractionNode::new(&left, &right);

        assert_eq!(node.result(), 5.8);

        left = ValueNode::new(-5.3);
        right = ValueNode::new(0.5);

        node = SubtractionNode::new(&left, &right);

        assert_eq!(node.result(), -5.8);
    }

}
