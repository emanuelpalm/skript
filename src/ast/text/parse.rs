use crate::ast::{BinaryOperator, Node};
use crate::ast::text::{tokenize, Error, Parser, Token};
use crate::ops::Binop;

pub fn parse(source: &[u8]) -> Result<Node, Error> {
    let tokens = tokenize(source)?;
    let mut parser = Parser::new(&tokens);

    parse_node(&mut parser)
}

fn parse_node(parser: &mut Parser) -> Result<Node, Error> {
    parse_factor(parser)
}

fn parse_factor(parser: &mut Parser) -> Result<Node, Error> {
    let mut left = parse_term(parser)?;
    loop {
        let code = match parser.peek() {
            Some(Token::Asterisk) => Binop::Mul,
            Some(Token::Slash) => Binop::Div,
            _ => return Ok(left),
        };
        parser.skip1();
        let right = parse_term(parser)?;
        left = Node::BinaryOperator(BinaryOperator::new(code, left.into(), right.into()));
    }
}

fn parse_term(parser: &mut Parser) -> Result<Node, Error> {
    let mut left = parse_primary(parser)?;
    loop {
        let code = match parser.peek() {
            Some(Token::Plus) => Binop::Add,
            Some(Token::Dash) => Binop::Sub,
            _ => return Ok(left),
        };
        parser.skip1();
        let right = parse_primary(parser)?;
        left = Node::BinaryOperator(BinaryOperator::new(code, left.into(), right.into()));
    }
}

fn parse_primary(parser: &mut Parser) -> Result<Node, Error> {
    match parser.next() {
        Some(Token::ParenthesisLeft) => {
            let node = parse_node(parser)?;
            match parser.next() {
                Some(Token::ParenthesisRight) => Ok(node),
                _ => Err(Error::ExpectedParenthesisRight),
            }
        },
        Some(Token::Number(value)) => Ok(Node::Value(*value)),
        Some(token) => Err(Error::UnexpectedToken(*token)),
        None => Err(Error::UnexpectedEnd),
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::Binop;
    use super::*;

    #[test]
    fn produces_correct_ast_from_source() {
        let source = "12 + (3 / 4)";
        let result = parse(source.as_bytes());
        assert_eq!(result, Ok(Node::BinaryOperator(BinaryOperator::new(
            Binop::Add,
            Node::Value(12.0).into(),
            Node::BinaryOperator(BinaryOperator::new(
                Binop::Div,
                Node::Value(3.0).into(),
                Node::Value(4.0).into(),
            )).into()))));
    }
}