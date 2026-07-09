//! Expression parser for the DTCS expression language (subset, Phase 0.6).

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, Span, UnaryOp};
use crate::diagnostics::{codes, Diagnostic, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Ident(String),
    String(String),
    Integer(i64),
    Decimal(f64),
    True,
    False,
    LParen,
    RParen,
    Comma,
    Op(&'static str),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    kind: TokenKind,
    span: Span,
}

pub fn parse_expression(source: &str) -> Result<Expr, ParseError> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let is_eof = matches!(token.kind, TokenKind::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let mut parser = Parser::new(source, tokens);
    let expr = parser.parse_or()?;
    parser.expect_eof()?;
    Ok(expr)
}

pub fn to_diagnostic(expression: &Expression, err: ParseError) -> Diagnostic {
    Diagnostic {
        id: codes::INVALID_EXPRESSION.to_string(),
        severity: Severity::Error,
        stage: DiagnosticStage::Analysis,
        category: DiagnosticCategory::Syntax,
        message: err.message,
        object_ref: Some(format!("expressions.{}", expression.id)),
        remediation: Some(
            "Fix expression syntax (operators, parentheses, or string literals)".into(),
        ),
    }
}

struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    _source: std::marker::PhantomData<&'a str>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str, tokens: Vec<Token>) -> Self {
        let _ = source;
        Self {
            tokens,
            pos: 0,
            _source: std::marker::PhantomData,
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or_else(|| self.tokens.last().expect("lexer provides EOF"))
    }

    fn consume(&mut self) -> Token {
        let current = self.peek().clone();
        if !matches!(current.kind, TokenKind::Eof) {
            self.pos += 1;
        }
        current
    }

    fn expect_eof(&self) -> Result<(), ParseError> {
        let token = self.peek();
        if matches!(token.kind, TokenKind::Eof) {
            Ok(())
        } else {
            Err(ParseError {
                message: "unexpected trailing input".into(),
                span: token.span.clone(),
            })
        }
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.match_op("||") {
            let op_token = self.tokens[self.pos - 1].clone();
            let right = self.parse_and()?;
            left = Expr::Binary {
                op: BinaryOp::Or,
                span: join_span(&left, &right, &op_token.span),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        while self.match_op("&&") {
            let op_token = self.tokens[self.pos - 1].clone();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op: BinaryOp::And,
                span: join_span(&left, &right, &op_token.span),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Op("==") => Some(BinaryOp::Eq),
                TokenKind::Op("!=") => Some(BinaryOp::Neq),
                TokenKind::Op("<") => Some(BinaryOp::Lt),
                TokenKind::Op("<=") => Some(BinaryOp::Lte),
                TokenKind::Op(">") => Some(BinaryOp::Gt),
                TokenKind::Op(">=") => Some(BinaryOp::Gte),
                _ => None,
            };
            let Some(op) = op else { break };
            let op_token = self.consume();
            let right = self.parse_additive()?;
            left = Expr::Binary {
                op,
                span: join_span(&left, &right, &op_token.span),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Op("+") => Some(BinaryOp::Add),
                TokenKind::Op("-") => Some(BinaryOp::Sub),
                _ => None,
            };
            let Some(op) = op else { break };
            let op_token = self.consume();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                op,
                span: join_span(&left, &right, &op_token.span),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Op("*") => Some(BinaryOp::Mul),
                TokenKind::Op("/") => Some(BinaryOp::Div),
                _ => None,
            };
            let Some(op) = op else { break };
            let op_token = self.consume();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                op,
                span: join_span(&left, &right, &op_token.span),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::Op("!") => {
                let op_token = self.consume();
                let inner = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    span: Span {
                        start: op_token.span.start,
                        end: end_span(&inner),
                    },
                    expr: Box::new(inner),
                })
            }
            TokenKind::Op("-") => {
                let op_token = self.consume();
                let inner = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    span: Span {
                        start: op_token.span.start,
                        end: end_span(&inner),
                    },
                    expr: Box::new(inner),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.consume();
        match token.kind {
            TokenKind::True => Ok(Expr::Literal {
                value: LiteralValue::Boolean(true),
                span: token.span,
            }),
            TokenKind::False => Ok(Expr::Literal {
                value: LiteralValue::Boolean(false),
                span: token.span,
            }),
            TokenKind::String(value) => Ok(Expr::Literal {
                value: LiteralValue::String(value),
                span: token.span,
            }),
            TokenKind::Integer(value) => Ok(Expr::Literal {
                value: LiteralValue::Integer(value),
                span: token.span,
            }),
            TokenKind::Decimal(value) => Ok(Expr::Literal {
                value: LiteralValue::Decimal(value),
                span: token.span,
            }),
            TokenKind::Ident(name) => {
                if self.match_kind(TokenKind::LParen) {
                    let (args, end_span) = self.parse_args()?;
                    Ok(Expr::Call {
                        callee: name,
                        args,
                        span: Span {
                            start: token.span.start,
                            end: end_span.end,
                        },
                    })
                } else {
                    Ok(Expr::FieldRef {
                        target: name,
                        span: token.span,
                    })
                }
            }
            TokenKind::LParen => {
                let inner = self.parse_or()?;
                self.expect_kind(TokenKind::RParen)?;
                Ok(inner)
            }
            _ => Err(ParseError {
                message: "expected a literal, field reference, function call, or parenthesized expression".into(),
                span: token.span,
            }),
        }
    }

    fn parse_args(&mut self) -> Result<(Vec<Expr>, Span), ParseError> {
        let mut args = Vec::new();

        if self.match_kind(TokenKind::RParen) {
            let end = self.tokens[self.pos - 1].span.clone();
            return Ok((args, end));
        }

        loop {
            args.push(self.parse_or()?);
            if self.match_kind(TokenKind::Comma) {
                continue;
            }
            let end = self.expect_kind(TokenKind::RParen)?;
            return Ok((args, end));
        }
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if same_kind(&self.peek().kind, &kind) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Span, ParseError> {
        let token = self.peek().clone();
        if same_kind(&token.kind, &kind) {
            self.consume();
            Ok(token.span)
        } else {
            Err(ParseError {
                message: format!("expected '{}'", token_kind_name(&kind)),
                span: token.span,
            })
        }
    }

    fn match_op(&mut self, op: &'static str) -> bool {
        if matches!(self.peek().kind, TokenKind::Op(candidate) if candidate == op) {
            self.consume();
            true
        } else {
            false
        }
    }
}

fn token_kind_name(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::LParen => "(",
        TokenKind::RParen => ")",
        TokenKind::Comma => ",",
        TokenKind::Eof => "end of input",
        TokenKind::Op(op) => op,
        TokenKind::Ident(_) => "identifier",
        TokenKind::String(_) => "string literal",
        TokenKind::Integer(_) => "integer",
        TokenKind::Decimal(_) => "decimal",
        TokenKind::True | TokenKind::False => "boolean",
    }
}

fn same_kind(left: &TokenKind, right: &TokenKind) -> bool {
    use TokenKind::*;
    match (left, right) {
        (LParen, LParen) | (RParen, RParen) | (Comma, Comma) | (Eof, Eof) => true,
        (Op(a), Op(b)) => a == b,
        (True, True) | (False, False) => true,
        (Ident(_), Ident(_)) => true,
        (String(_), String(_)) => true,
        (Integer(_), Integer(_)) => true,
        (Decimal(_), Decimal(_)) => true,
        _ => false,
    }
}

fn end_span(expr: &Expr) -> usize {
    match expr {
        Expr::Literal { span, .. }
        | Expr::FieldRef { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Call { span, .. } => span.end,
    }
}

fn join_span(left: &Expr, right: &Expr, op_span: &Span) -> Span {
    let start = match left {
        Expr::Literal { span, .. }
        | Expr::FieldRef { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Call { span, .. } => span.start,
    };
    Span {
        start,
        end: right_span_end(right).max(op_span.end),
    }
}

fn right_span_end(expr: &Expr) -> usize {
    match expr {
        Expr::Literal { span, .. }
        | Expr::FieldRef { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Call { span, .. } => span.end,
    }
}

struct Lexer<'a> {
    source: &'a str,
    chars: std::str::CharIndices<'a>,
    peeked: Option<(usize, char)>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices(),
            peeked: None,
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        if let Some(peeked) = self.peeked.take() {
            return Some(peeked);
        }
        self.chars.next()
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_ws();
        let Some((start, ch)) = self.bump() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: self.source.len(),
                    end: self.source.len(),
                },
            });
        };

        match ch {
            '(' => Ok(Token {
                kind: TokenKind::LParen,
                span: Span {
                    start,
                    end: start + 1,
                },
            }),
            ')' => Ok(Token {
                kind: TokenKind::RParen,
                span: Span {
                    start,
                    end: start + 1,
                },
            }),
            ',' => Ok(Token {
                kind: TokenKind::Comma,
                span: Span {
                    start,
                    end: start + 1,
                },
            }),
            '"' | '\'' => self.lex_string(start, ch),
            '0'..='9' => self.lex_number(start, ch),
            '!' => {
                if self.try_match('=') {
                    Ok(Token {
                        kind: TokenKind::Op("!="),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Op("!"),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '=' => {
                if self.try_match('=') {
                    Ok(Token {
                        kind: TokenKind::Op("=="),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Err(ParseError {
                        message: "unexpected '='; did you mean '=='?".into(),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '<' => {
                if self.try_match('=') {
                    Ok(Token {
                        kind: TokenKind::Op("<="),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Op("<"),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '>' => {
                if self.try_match('=') {
                    Ok(Token {
                        kind: TokenKind::Op(">="),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Op(">"),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '&' => {
                if self.try_match('&') {
                    Ok(Token {
                        kind: TokenKind::Op("&&"),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Err(ParseError {
                        message: "unexpected '&'; did you mean '&&'?".into(),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '|' => {
                if self.try_match('|') {
                    Ok(Token {
                        kind: TokenKind::Op("||"),
                        span: Span {
                            start,
                            end: start + 2,
                        },
                    })
                } else {
                    Err(ParseError {
                        message: "unexpected '|'; did you mean '||'?".into(),
                        span: Span {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            '+' | '-' | '*' | '/' => Ok(Token {
                kind: TokenKind::Op(match ch {
                    '+' => "+",
                    '-' => "-",
                    '*' => "*",
                    '/' => "/",
                    _ => unreachable!(),
                }),
                span: Span {
                    start,
                    end: start + 1,
                },
            }),
            _ if is_ident_start(ch) => self.lex_ident(start, ch),
            _ => Err(ParseError {
                message: format!("unexpected character '{ch}'"),
                span: Span {
                    start,
                    end: start + ch.len_utf8(),
                },
            }),
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some((_, ch)) if ch.is_whitespace()) {
            self.bump();
        }
    }

    fn try_match(&mut self, expected: char) -> bool {
        match self.peek() {
            Some((_, ch)) if ch == expected => {
                self.bump();
                true
            }
            _ => false,
        }
    }

    fn lex_string(&mut self, start: usize, quote: char) -> Result<Token, ParseError> {
        let mut out = String::new();
        let mut end: Option<usize> = None;

        while let Some((idx, ch)) = self.bump() {
            let ch_end = idx + ch.len_utf8();
            if ch == quote {
                return Ok(Token {
                    kind: TokenKind::String(out),
                    span: Span { start, end: ch_end },
                });
            }
            if ch == '\\' {
                let Some((idx2, escaped)) = self.bump() else {
                    return Err(ParseError {
                        message: "unterminated escape sequence in string literal".into(),
                        span: Span { start, end: ch_end },
                    });
                };
                end = Some(idx2 + escaped.len_utf8());
                out.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    other => other,
                });
                continue;
            }
            end = Some(ch_end);
            out.push(ch);
        }

        Err(ParseError {
            message: "unterminated string literal".into(),
            span: Span {
                start,
                end: end.unwrap_or(start + 1),
            },
        })
    }

    fn lex_number(&mut self, start: usize, first: char) -> Result<Token, ParseError> {
        let mut buf = String::new();
        buf.push(first);
        let mut end = start + first.len_utf8();
        let mut has_dot = false;

        while let Some((idx, ch)) = self.peek() {
            if ch.is_ascii_digit() {
                self.bump();
                buf.push(ch);
                end = idx + 1;
                continue;
            }
            if ch == '.' && !has_dot {
                self.bump();
                buf.push(ch);
                has_dot = true;
                end = idx + 1;
                continue;
            }
            break;
        }

        if has_dot {
            let value = buf.parse::<f64>().map_err(|_| ParseError {
                message: "invalid decimal literal".into(),
                span: Span { start, end },
            })?;
            Ok(Token {
                kind: TokenKind::Decimal(value),
                span: Span { start, end },
            })
        } else {
            let value = buf.parse::<i64>().map_err(|_| ParseError {
                message: "invalid integer literal".into(),
                span: Span { start, end },
            })?;
            Ok(Token {
                kind: TokenKind::Integer(value),
                span: Span { start, end },
            })
        }
    }

    fn lex_ident(&mut self, start: usize, first: char) -> Result<Token, ParseError> {
        let mut buf = String::new();
        buf.push(first);
        let mut end = start + first.len_utf8();

        while let Some((idx, ch)) = self.peek() {
            if is_ident_continue(ch) {
                self.bump();
                buf.push(ch);
                end = idx + ch.len_utf8();
            } else {
                break;
            }
        }

        let kind = match buf.as_str() {
            "true" | "TRUE" | "True" => TokenKind::True,
            "false" | "FALSE" | "False" => TokenKind::False,
            _ => TokenKind::Ident(buf),
        };

        Ok(Token {
            kind,
            span: Span { start, end },
        })
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | ':' | '.')
}
