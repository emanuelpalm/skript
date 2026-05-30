use super::{Error, ErrorKind, Parser, tokenize};
use crate::ast::{Class, Expr, Stmt};
use crate::ops::Binop;

pub fn parse(source: &[u8]) -> Result<Vec<Stmt>, Error> {
    let tokens = tokenize(source)?;
    let mut p = Parser::new(&tokens);

    let mut result: Vec<Stmt> = Vec::new();
    while let Some(stmt) = parse_stmt(source, &mut p)? {
        result.push(stmt);
    }
    Ok(result)
}

fn parse_stmt(s: &[u8], p: &mut Parser) -> Result<Option<Stmt>, Error> {
    let stmt = match p.peek_class() {
        Some(Class::Let) => parse_stmt_let(s, p)?,
        Some(Class::Return) => parse_stmt_return(s, p)?,
        Some(_) => parse_stmt_expr(s, p)?,
        None => return Ok(None),
    };
    p.expect_class_or(Class::Semicolon, ErrorKind::ExpectedSemicolonAfterStatement)?;
    Ok(Some(stmt))
}

fn parse_stmt_let(s: &[u8], p: &mut Parser) -> Result<Stmt, Error> {
    p.skip1();
    let identifier = p.expect_class_or(Class::Identifier, ErrorKind::ExpectedIdentifierAfterLet)?;
    p.expect_class_or(Class::Equal, ErrorKind::ExpectedEqualAfterLetIdentifier)?;
    let expr = parse_expr(s, p)?;
    Ok(Stmt::Let {
        identifier: identifier.resolve_as_string(s),
        expr: expr.into(),
    })
}

fn parse_stmt_return(s: &[u8], p: &mut Parser) -> Result<Stmt, Error> {
    p.skip1();
    let expr = parse_expr(s, p)?;
    Ok(Stmt::Return(expr.into()))
}

fn parse_stmt_expr(s: &[u8], p: &mut Parser) -> Result<Stmt, Error> {
    let expr = parse_expr(s, p)?;
    Ok(Stmt::Expr(expr.into()))
}

fn parse_expr(s: &[u8], p: &mut Parser) -> Result<Expr, Error> {
    parse_factor(s, p)
}

fn parse_factor(s: &[u8], p: &mut Parser) -> Result<Expr, Error> {
    let mut left = parse_term(s, p)?;
    loop {
        let binop = match p.peek_class() {
            Some(Class::Asterisk) => Binop::Mul,
            Some(Class::Slash) => Binop::Div,
            _ => return Ok(left),
        };
        p.skip1();
        let right = parse_term(s, p)?;
        left = Expr::BinaryOperator {
            binop,
            left: left.into(),
            right: right.into(),
        };
    }
}

fn parse_term(s: &[u8], p: &mut Parser) -> Result<Expr, Error> {
    let mut left = parse_primary(s, p)?;
    loop {
        let binop = match p.peek_class() {
            Some(Class::Plus) => Binop::Add,
            Some(Class::Dash) => Binop::Sub,
            _ => return Ok(left),
        };
        p.skip1();
        let right = parse_primary(s, p)?;
        left = Expr::BinaryOperator {
            binop,
            left: left.into(),
            right: right.into(),
        };
    }
}

fn parse_primary(s: &[u8], p: &mut Parser) -> Result<Expr, Error> {
    let token = p.next()?;

    match token.class() {
        Class::Identifier => Ok(Expr::Identifier(token.resolve_as_string(s))),
        Class::Number => Ok(Expr::Value(token.resolve_as_f64(s))),
        Class::ParenthesisLeft => {
            let expr = parse_expr(s, p)?;
            match p.next()? {
                token if token.class() == Class::ParenthesisRight => Ok(expr),
                token => Err(Error::new(
                    ErrorKind::ExpectedClosingParenthesis,
                    token.span(),
                )),
            }
        }
        _ => Err(Error::new(ErrorKind::UnexpectedToken, token.span())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::Binop;

    #[test]
    fn produces_correct_ast_from_source() {
        let source = "let digits = 12 + (3 / 4); digits - 1;";
        let result = parse(source.as_bytes());
        assert_eq!(
            result,
            Ok(vec![
                Stmt::Let {
                    identifier: "digits".into(),
                    expr: Expr::BinaryOperator {
                        binop: Binop::Add,
                        left: Expr::Value(12.0).into(),
                        right: Expr::BinaryOperator {
                            binop: Binop::Div,
                            left: Expr::Value(3.0).into(),
                            right: Expr::Value(4.0).into(),
                        }
                        .into()
                    }
                    .into(),
                },
                Stmt::Expr(
                    Expr::BinaryOperator {
                        binop: Binop::Sub,
                        left: Expr::Identifier("digits".into()).into(),
                        right: Expr::Value(1.0).into()
                    }
                    .into()
                ),
            ])
        );
    }
}
