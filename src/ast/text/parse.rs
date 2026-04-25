use crate::ast::{BinaryOperator, BinaryOperatorCode, Node};
use crate::ast::text::{tokenize, Error, Parser, Token};

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
            Some(Token::Asterisk) => {
                parser.skip1();
                BinaryOperatorCode::Mul
            }
            Some(Token::Slash) => {
                parser.skip1();
                BinaryOperatorCode::Div
            }
            _ => return Ok(left),
        };
        let right = parse_term(parser)?;
        left = Node::BinaryOperator(BinaryOperator::new(code, left.into(), right.into()));
    }
}

fn parse_term(parser: &mut Parser) -> Result<Node, Error> {
    let mut left = parse_primary(parser)?;
    loop {
        let code = match parser.peek() {
            Some(Token::Plus) => {
                parser.skip1();
                BinaryOperatorCode::Add
            }
            Some(Token::Dash) => {
                parser.skip1();
                BinaryOperatorCode::Sub
            }
            _ => return Ok(left),
        };
        let right = parse_primary(parser)?;
        left = Node::BinaryOperator(BinaryOperator::new(code, left.into(), right.into()));
    }
}

fn parse_primary(parser: &mut Parser) -> Result<Node, Error> {
    match parser.peek() {
        Some(Token::ParenthesisLeft) => {
            parser.skip1();
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
