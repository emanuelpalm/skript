use crate::ast::{BinaryOperator, BinaryOperatorCode, Node};
use crate::vm::opcode;

pub fn generate(tree: Node) -> Vec<u8> {
    let mut output = Vec::new();
    generate_node(&tree, &mut output);
    output.push(opcode::HALT);
    output
}

fn generate_node(node: &Node, output: &mut Vec<u8>) {
    match node {
        Node::BinaryOperator(operator) => generate_binary_operator(operator, output),
        Node::Value(value) => generate_value(*value, output),
    }
}

fn generate_binary_operator(binop: &BinaryOperator, output: &mut Vec<u8>) {
    generate_node(binop.right(), output);
    generate_node(binop.left(), output);
    match binop.code() {
        BinaryOperatorCode::Add => output.push(opcode::ADD),
        BinaryOperatorCode::Sub => output.push(opcode::SUB),
        BinaryOperatorCode::Mul => output.push(opcode::MUL),
        BinaryOperatorCode::Div => output.push(opcode::DIV),
    }
}

fn generate_value(value: f64, output: &mut Vec<u8>) {
    if value >= i8::MIN as f64 && value <= i8::MAX as f64 && value.fract() == 0.0 {
        output.push(opcode::PUSH_I8);
        output.push(value as i8 as u8);
    } else {
        output.push(opcode::PUSH_F64);
        value.to_ne_bytes().iter().for_each(|&byte| output.push(byte));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_simple_binary_operator_correctly() {
        // 1 + 2
        let ast = Node::BinaryOperator(BinaryOperator::new(
            BinaryOperatorCode::Add,
            Node::Value(1.0).into(),
            Node::Value(2.0).into(),
        ).into());

        let code = generate(ast);
        assert_eq!(code, vec![
            opcode::PUSH_I8,
            2,
            opcode::PUSH_I8,
            1,
            opcode::ADD,
            opcode::HALT,
        ]);
    }

    #[test]
    fn generates_nested_binary_operator_correctly() {
        // 100 + 4 * 5
        let ast = Node::BinaryOperator(BinaryOperator::new(
            BinaryOperatorCode::Add,
            Node::Value(100.0).into(),
            Node::BinaryOperator(BinaryOperator::new(
                BinaryOperatorCode::Mul,
                Node::Value(4.0).into(),
                Node::Value(5.0).into(),
            )).into()
        ).into());

        let code = generate(ast);
        assert_eq!(code, vec![
            opcode::PUSH_I8,
            5,
            opcode::PUSH_I8,
            4,
            opcode::MUL,
            opcode::PUSH_I8,
            100,
            opcode::ADD,
            opcode::HALT,
        ]);
    }
}