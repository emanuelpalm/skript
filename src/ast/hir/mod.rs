use crate::{ast, hir};

struct Accumulator {
    instructions: Vec<hir::Instr>,
    register_counter: u8,
}

impl Accumulator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            register_counter: 0,
        }
    }

    fn allocate(&mut self) -> hir::Register {
        let n = self.register_counter;
        self.register_counter += 1;
        hir::Register(n)
    }

    fn push(&mut self, instr: hir::Instr) {
        self.instructions.push(instr);
    }

    fn unwrap(self) -> Vec<hir::Instr> {
        self.instructions
    }
}

pub fn lower(tree: &ast::Node) -> hir::Block {
    let mut acc = Accumulator::new();
    let source = lower_node(tree, &mut acc);
    acc.push(hir::Instr::Return { source });
    hir::Block::new(acc.unwrap())
}

pub fn lower_node(node: &ast::Node, acc: &mut Accumulator) -> hir::Register {
    match node {
        ast::Node::BinaryOperator(node) => {
            let left = lower_node(node.left(), acc);
            let right = lower_node(node.right(), acc);

            let operator = match node.code() {
                ast::BinaryOperatorCode::Add => hir::BinaryOperatorCode::Add,
                ast::BinaryOperatorCode::Sub => hir::BinaryOperatorCode::Sub,
                ast::BinaryOperatorCode::Mul => hir::BinaryOperatorCode::Mul,
                ast::BinaryOperatorCode::Div => hir::BinaryOperatorCode::Div,
            };

            let target = acc.allocate();

            acc.push(hir::Instr::BinaryOperator { left, right, operator, target });

            target
        }

        ast::Node::Value(value) => {
            let target = acc.allocate();
            acc.push(hir::Instr::Load { value: *value, target });
            target
        }
    }
}
