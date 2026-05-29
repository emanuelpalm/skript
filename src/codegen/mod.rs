/*
use crate::ast::{BinaryOperator, BinaryOperatorCode, Node};
use crate::rvm::{Code, Instr};
use crate::rvm::opcode::*;

pub fn generate(tree: Node) -> Code {
    let mut output = Vec::new();
    generate_node(&tree, &mut output);
    output.push(Instr::new(OP_HALT as u32));
    Code::new(output)
}

fn generate_node(node: &Node, output: &mut Vec<Instr>) {
    match node {
        Node::BinaryOperator(operator) => generate_binary_operator(operator, output),
        Node::Value(value) => generate_value(*value, output),
    }
}

fn generate_binary_operator(binop: &BinaryOperator, output: &mut Vec<Instr>) {
    generate_node(binop.right(), output);
    generate_node(binop.left(), output);
    match binop.code() {
        BinaryOperatorCode::Add => output.push(opcode::ADD),
        BinaryOperatorCode::Sub => output.push(opcode::SUB),
        BinaryOperatorCode::Mul => output.push(opcode::MUL),
        BinaryOperatorCode::Div => output.push(opcode::DIV),
    }
}

fn generate_value(value: f64, output: &mut Vec<Instr>) {
    if value >= i8::MIN as f64 && value <= i8::MAX as f64 && value.fract() == 0.0 {
        output.push(opcode::SET_S_I8);
        output.push(value as i8 as u8);
    } else {
        output.push(opcode::SET_S_F64);
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
        assert_eq!(code, Code::new([
            opcode::SET_S_I8,
            2,
            opcode::SET_S_I8,
            1,
            opcode::ADD,
            opcode::HALT,
        ]));
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
        ));

        let code = generate(ast);
        assert_eq!(code, Code::new([
            opcode::SET_S_I8,
            5,
            opcode::SET_S_I8,
            4,
            opcode::MUL,
            opcode::SET_S_I8,
            100,
            opcode::ADD,
            opcode::HALT,
        ]));
    }


    #[test]
    fn generates_nested_binary_operators_correctly() {
        // (((5 + 4) - 3) * 2) / 1
        let ast = Node::BinaryOperator(BinaryOperator::new(
            BinaryOperatorCode::Div,
            Node::BinaryOperator(BinaryOperator::new(
                BinaryOperatorCode::Mul,
                Node::BinaryOperator(BinaryOperator::new(
                    BinaryOperatorCode::Sub,
                    Node::BinaryOperator(BinaryOperator::new(
                        BinaryOperatorCode::Add,
                        Node::Value(5.0).into(),
                        Node::Value(4.0).into()
                    )).into(),
                    Node::Value(3.0).into()
                )).into(),
                Node::Value(2.0).into()
            )).into(),
            Node::Value(1.0).into(),
        ));

        let code = generate(ast);
        assert_eq!(code, Code::new([
            opcode::SET_S_I8,
            1,
            opcode::SET_S_I8,
            2,
            opcode::SET_S_I8,
            3,
            opcode::SET_S_I8,
            4,
            opcode::SET_S_I8,
            5,
            opcode::ADD,
            opcode::SUB,
            opcode::MUL,
            opcode::DIV,
            opcode::HALT,
        ]));
    }
}*/