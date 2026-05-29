use super::accumulator::*;
use crate::{ast, hir};

pub fn lower(tree: &ast::Node) -> hir::Block {
    let mut acc = Accumulator::new();
    let source = lower_node(tree, &mut acc);
    acc.push(hir::Instr::Return { source });
    hir::Block::new(acc.unwrap())
}

pub fn lower_node(node: &ast::Node, acc: &mut Accumulator) -> hir::Register {
    match node {
        ast::Node::BinaryOperator(node) => {
            let kind = node.binop();
            let left = lower_node(node.left(), acc);
            let right = lower_node(node.right(), acc);

            let target = acc.allocate();

            acc.push(hir::Instr::BinaryOperator { kind, left, right, target });

            target
        }

        ast::Node::Value(value) => {
            let target = acc.allocate();
            acc.push(hir::Instr::Load { value: *value, target });
            target
        }
    }
}
