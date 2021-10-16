use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, many1, many_m_n},
    sequence::{delimited, pair, preceded},
    bytes::complete::tag,
    IResult,
};

use crate::sexp::{Expr, Atom};
use crate::parser::string;

fn parse_bool<'a>(input: &'a str) -> IResult<&'a str, Atom> {
    alt((
        value(Atom::Bool(false), tag("false")),
        value(Atom::Bool(true), tag("true"))
    ))(input)
}

fn parse_list<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    let inside_parser = map(
        pair(opt(parse_expr), many0(preceded(multispace1, parse_expr))),
        |p: (Option<Expr>, Vec<Expr>)| {
            let mut v = p.1.clone();
            if let Some(val) = p.0 {
                v.push(val);
                v.rotate_right(1);
            }
            Expr::List(Box::new(v))
        },
    );

    preceded(
        multispace0,
        delimited(char('('), inside_parser, preceded(multispace0, char(')'))),
    )(input)
}

pub fn parse_num<'a>(input: &'a str) -> IResult<&'a str, Atom> {
    map(
        map_res(
            recognize(preceded(opt(char('-')), digit1)),
            |out: &'a str| i64::from_str_radix(&out, 10),
        ),
        Atom::Num,
    )(input)
}

pub fn parse_atom<'a>(input: &'a str) -> IResult<&'a str, Atom> {
    alt((
        parse_bool,
        map(string::parse_string, Atom::Str),
        parse_num, // map(|s: &'a str| be_i64(s.map(|c| c.encode_utf8())), Atom::Num)
        parse_sym
    ))(input)
}

fn parse_sym<'a>(input: &'a str) -> IResult<&'a str, Atom> {
    let first_char_parser = many_m_n(1, 1, alpha1);
    let char_parser = alt((
        alpha1,
        digit1,
        recognize(many1(one_of("-_")))
    ));
    let rest_chars_parser = many0(char_parser);
    let inside_parser = pair(first_char_parser, rest_chars_parser);
    map(recognize(inside_parser), |s: &'a str| Atom::Sym(s.to_string()))(input)
}

pub fn parse_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    // map(parse_num, |x| Expr::Atom(x))(input)
    preceded(multispace0, alt((parse_list, map(parse_atom, Expr::Atom))))(input)
}
