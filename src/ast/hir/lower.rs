use super::accumulator::*;
use crate::{ast, hir};

pub fn lower(tree: &[ast::Stmt]) -> hir::Block {
    let mut acc = Accumulator::new();
    for stmt in tree {
        lower_stmt(stmt, &mut acc);
    }
    hir::Block::new(acc.unwrap())
}

pub fn lower_stmt(stmt: &ast::Stmt, acc: &mut Accumulator) {
    match stmt {
        ast::Stmt::Expr(expr) => {
            lower_expr(expr, acc);
        },
        ast::Stmt::Let { identifier, expr } => {
            todo!()
        }
        ast::Stmt::Return(expr) => {
            let source = lower_expr(expr, acc);
            acc.push(hir::Instr::Return { source });
        }
    }
}

pub fn lower_expr(expr: &ast::Expr, acc: &mut Accumulator) -> hir::Register {
    match expr {
        ast::Expr::BinaryOperator { binop, left, right} => {
            let left = lower_expr(left, acc);
            let right = lower_expr(right, acc);

            let target = acc.allocate();

            acc.push(hir::Instr::BinaryOperator { binop: *binop, left, right, target });

            target
        }

        ast::Expr::Identifier(name) => {
            todo!()
        }

        ast::Expr::Value(value) => {
            let target = acc.allocate();
            acc.push(hir::Instr::Load { value: *value, target });
            target
        }
    }
}
